import { $, Glob, file, write } from "bun";
import { mkdir } from "node:fs/promises";
import { execSync } from "node:child_process";

const glob = new Glob("tools/in/*.txt");
await $`cd tools && cargo run -r --bin gen seeds.txt`;
await $`g++ ./a.cpp -o ./a.out`;

const reports = [];

for (const path of glob.scanSync()) {
  const input = await file(path).text();
  const filename = path.split("/").pop();
  execSync(`echo "${input}" | ./a.out > tools/report.out`);
  const { stdout } =
    await $`cd tools && cargo run -r --bin vis in/${filename} report.out`.quiet();
  const res = stdout.toString();
  const scoreRE = new RegExp("Score = ([0-9.]+)");
  const score = scoreRE.exec(res)[1];
  reports.push({ score: Number(score), filename });
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
