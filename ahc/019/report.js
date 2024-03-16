import { $, Glob, file, write } from "bun";
import { mkdir } from "node:fs/promises";

const glob = new Glob("tools/in/*.txt");
await $`cd tools && cargo run -r --bin gen seeds.txt`;
await $`g++ ./a.cpp -o ./a.out`;

const scores = [];

for await (const path of glob.scan()) {
  const input = await file(path).text();
  const { stdout } = await $`./a.out --export < ${new Response(input)}`.quiet();
  await write("report.out", stdout);
  const { stdout: stdout2 } =
    await $`cd tools && cargo run --release --bin vis ${path.replace(
      "tools/",
      ""
    )} ../report.out`.quiet();
  const scoreRE = new RegExp("Score = ([0-9.]+)");
  const score = scoreRE.exec(stdout2.toString())[1];
  scores.push(Number(score));
  console.log(`Path: ${path}, Score: ${score}`);
}

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
