import { $, Glob, file, write } from "bun";
import { mkdir } from "node:fs/promises";

const glob = new Glob("tools/in/*.txt");
await $`cd tools && cargo run -r --bin gen seeds.txt`;
await $`g++ ./a.cpp -o ./a.out`;

const scores = [];

await Promise.all(
  Array.from(glob.scanSync()).map(async (path) => {
    const input = await file(path).text();
    const { stdout } = await $`./a.out < ${new Response(input)}`.quiet();
    // stdoutの最初の行を取得
    const res = stdout.toString();
    const firstLine = res.split("\n")[0];
    console.log(`input: ${path}, score: ${firstLine}`);
    scores.push(100000 - 5 * Number(firstLine));
  })
);

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
