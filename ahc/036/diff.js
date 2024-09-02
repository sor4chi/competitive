import fs from "fs";

const c = {
  red: (s) => `\x1b[31m${s}\x1b[0m`,
  yellow: (s) => `\x1b[33m${s}\x1b[0m`,
  green: (s) => `\x1b[32m${s}\x1b[0m`,
};

const percentageLogger = (rate) => {
  return rate < 0
    ? c.green(`${(rate * 100).toFixed(2)}%`)
    : rate > 0
    ? c.red(`+${(rate * 100).toFixed(2)}%`)
    : `${(rate * 100).toFixed(2)}%`;
};

const base = process.argv[2];
const other = process.argv[3];
const baseReport = JSON.parse(fs.readFileSync(base, "utf-8"));
const otherReport = JSON.parse(fs.readFileSync(other, "utf-8"));
const diffRate = {};
for (const seed in baseReport) {
  const diff = baseReport[seed] - otherReport[seed];
  diffRate[seed] = {
    base: diff < 0 ? c.green(baseReport[seed]) : c.red(baseReport[seed]),
    other: diff > 0 ? c.green(otherReport[seed]) : c.red(otherReport[seed]),
    diff: percentageLogger(diff / baseReport[seed]),
  };
}
const baseFileName = base.split("/").pop();
const otherFileName = other.split("/").pop();
console.log(`Diff rates between ${baseFileName} and ${otherFileName}:`);
console.table(diffRate);

// get worst case and best case
const diffArray = Object.entries(diffRate).map(([seed, { diff }]) => ({
  seed,
  diff: /(^\+|-)?\d+\.\d+/.exec(diff)[0],
}));
diffArray.sort((a, b) => a.diff - b.diff);
console.log(`Worst case: ${diffArray[0].seed} ${diffArray[0].diff}`);
console.log(
  `Best case: ${diffArray[diffArray.length - 1].seed} ${
    diffArray[diffArray.length - 1].diff
  }`
);
