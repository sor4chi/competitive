import { execSync } from "child_process";
import fs from "fs";

console.log("Compiling...");
execSync("cd solver && cargo build -r");

const report = {};

const c = {
  red: (s) => `\x1b[31m${s}\x1b[0m`,
  yellow: (s) => `\x1b[33m${s}\x1b[0m`,
  green: (s) => `\x1b[32m${s}\x1b[0m`,
};

const percentageLogger = (rate) => {
  return rate > 0
    ? c.green(`+${(rate * 100).toFixed(2)}%`)
    : rate < 0
    ? c.red(`${(rate * 100).toFixed(2)}%`)
    : `${(rate * 100).toFixed(2)}%`;
};

const SEED_START = 1;
const SEED_END = 100;
console.log(`Testing seeds from ${SEED_START} to ${SEED_END}...`);

execSync("rm -rf tools/out");
execSync("rm -rf tools/err");
execSync("mkdir tools/out");
execSync("mkdir tools/err");

for (let seed = SEED_START; seed <= SEED_END; seed++) {
  seed = seed.toString().padStart(4, "0");
  console.log(`Testing seed ${seed}...`);
  execSync(
    `cd solver && time ./target/release/solve < ../tools/in/${seed}.txt > ../tools/out/${seed}.txt`
  );
  // 点数は標準エラー出力に出力されるので、一度ファイルにリダイレクトしてから読み取る
  execSync(
    `cd tools && cargo run -r --bin vis in/${seed}.txt out/${seed}.txt 2> err/${seed}.txt`
  );
  const res = fs.readFileSync(`tools/err/${seed}.txt`);
  const SCORE_RE = /Score = (\d+)/;
  const match = SCORE_RE.exec(res.toString());
  if (match) {
    console.log(`Seed ${seed}: ${match[1]}`);
    report[seed] = parseInt(match[1]);
  } else {
    console.log(`Seed ${seed}: Failed`);
  }
}

const reports = fs.readdirSync("reports").filter((f) => f.endsWith(".json"));
reports.sort();
reports.forEach((f) => {
  const otherReport = JSON.parse(fs.readFileSync(`reports/${f}`, "utf-8"));
  const diffRate = {};
  const res = [];
  for (const seed in report) {
    const diff = report[seed] - otherReport[seed];
    diffRate[seed] = diff / report[seed];
    res.push(percentageLogger(diffRate[seed]));
  }
  console.log(`Diff overview with ${f}:`);
  console.log(res.join(" "));
  const avgDiffRate =
    Object.values(diffRate).reduce((a, b) => a + b, 0) /
    Object.keys(diffRate).length;
  console.log(`Average diff rate: ${percentageLogger(avgDiffRate)}`);
});

const now = new Date();
const reportPath = `${now.getTime()}.json`;
fs.writeFileSync(`reports/${reportPath}`, JSON.stringify(report, null, 2));