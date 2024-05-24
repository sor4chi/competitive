import { $, Glob, file, write } from "bun";
import { mkdir } from "node:fs/promises";
import { execSync } from "node:child_process";

const glob = new Glob("tools/in/*.txt");
await $`cd tools && cargo run -r --bin gen seeds.txt`;
// await $`g++ ./a.cpp -o ./a.out`;
await $`cd solver && cargo build --release --bin two_cranes && cp ./target/release/solver ../a.out`;

const reports = [];

const sortedPaths = Array.from(glob.scanSync()).sort((a, b) =>
  a.localeCompare(b)
);

const pathStatus = {
  good: [],
  notGood: [],
  runtimeError: [],
};

for (const path of sortedPaths) {
  console.log(path);
  await new Promise((resolve) => setTimeout(resolve, 500));
  const input = await file(path).text();
  const filename = path.split("/").pop();
  try {
    execSync(`echo "${input}" | ./a.out > tools/out/${filename}`);
  } catch (e) {
    console.log("Runtime Error");
    pathStatus.runtimeError.push(path);
    continue;
  }
  const { stdout } =
    await $`cd tools && cargo run -r --bin vis in/${filename} out/${filename}`.quiet();
  const res = stdout.toString();
  const scoreRE = new RegExp("Score = ([0-9.]+)");
  const score = scoreRE.exec(res)[1];
  console.log("Score:", score);
  reports.push({ score: Number(score), filename });
  if (Number(score) < 300 && Number(score) > 0) {
    pathStatus.good.push(path);
  } else {
    pathStatus.notGood.push(path);
  }
}

const total = {
  score: reports.reduce((a, b) => a + b.score, 0),
  filename: "total",
};
const avg = { score: total.score / reports.length, filename: "avg" };
const max = reports.reduce((a, b) => (a.score > b.score ? a : b));
const min = reports.reduce((a, b) => (a.score < b.score ? a : b));
const p90 = reports.sort((a, b) => a.score - b.score)[
  Math.floor(reports.length * 0.9)
];
const p50 = reports.sort((a, b) => a.score - b.score)[
  Math.floor(reports.length * 0.5)
];
console.table({ total, avg, max, min, p90, p50 });

const reportFile = `./reports/${new Date().toISOString()}.json`;
await mkdir("reports", { recursive: true });
await write(
  reportFile,
  JSON.stringify({ total, avg, max, min, p90, p50 }, null, 2)
);

console.log("Good:", pathStatus.good.length);
console.log("Not Good:", pathStatus.notGood.length);
console.log("Runtime Error:", pathStatus.runtimeError.length);
console.log("Total:", sortedPaths.length);

await write("./reports/pathStatus.json", JSON.stringify(pathStatus, null, 2));
