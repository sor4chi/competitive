import { $, Glob, file, write } from "bun";
import { mkdir } from "node:fs/promises";

const glob = new Glob("tools/in/*.txt");
await $`cd tools && cargo run -r --bin gen < seeds.txt`;
await $`g++ ./a.cpp -o ./a.out`;

const scoreMap = new Map();
const MODE = 1; // 0: parallel, 1: serial

if (MODE === 0) {
  await Promise.all(
    Array.from(glob.scanSync()).map(async (path) => {
      const input = await file(path).text();
      const { stdout } = await $`./a.out --export < ${new Response(
        input
      )}`.quiet();
      const res = stdout.toString();
      const scoreRE = new RegExp("Score = ([0-9.]+)");
      const score = scoreRE.exec(res)[1];
      scoreMap.set(path, Number(score));
    })
  );
} else {
  const paths = Array.from(glob.scanSync());
  for (let i = 0; i < paths.length; i++) {
    const path = paths[i];
    const input = await file(path).text();
    const { stdout } = await $`./a.out --export < ${new Response(
      input
    )}`.quiet();
    const res = stdout.toString();
    const scoreRE = new RegExp("Score = ([0-9.]+)");
    const score = scoreRE.exec(res)[1];
    scoreMap.set(path, Number(score));
    console.log(`${i + 1}/${paths.length}`, "path:", path, "score:", score);
  }
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
