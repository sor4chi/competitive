import { $, Glob, file, write } from "bun";
import { execSync } from "node:child_process";
import { mkdir } from "node:fs/promises";

const glob = new Glob("tools/in/*.txt");
await $`g++ ./a.cpp -o ./a.out`;

const scoreMap = new Map();

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const paths = Array.from(glob.scanSync());
for (let i = 0; i < paths.length; i++) {
  const path = paths[i];
  const input = await file(path).text();
  const { stdout } = await $`./a.out < ${new Response(input)}`.quiet();
  await write("report.out", stdout);
  await sleep(500);
  const res = execSync(
    'cd tools && cargo run -r --bin score "' +
      path.replace("tools/", "") +
      '" "../report.out"'
  ).toString();
  await sleep(500);
  const scoreRE = new RegExp("Score = ([0-9.]+)");
  const score = scoreRE.exec(res)[1];
  if (score == "0") {
    console.error("score 0 for", path);
    os.exit(1);
  }
  scoreMap.set(path, Number(score));
  console.log(`${i + 1}/${paths.length}`, "path:", path, "score:", score);
}

const scores = Array.from(scoreMap.values());
const total = scores.reduce((a, b) => a + b, 0);
const avg = total / scores.length;
const max = Math.max(...scores);
const min = Math.min(...scores);
const p90 = scores.sort((a, b) => a - b)[Math.floor(scores.length * 0.9)];
const p50 = scores.sort((a, b) => a - b)[Math.floor(scores.length * 0.5)];
console.table({ total, avg, max, min, p90, p50 });

const reportFile = `./reports/${new Date().toISOString()}.json`;
await mkdir("reports", { recursive: true });
await write(
  reportFile,
  JSON.stringify({ total, avg, max, min, p90, p50 }, null, 2)
);

console.log("lower 10 scores' paths/scores:");
console.table(
  Array.from(scoreMap.entries())
    .sort((a, b) => a[1] - b[1])
    .slice(0, 10)
);
