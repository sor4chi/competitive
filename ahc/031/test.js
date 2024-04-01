import { $, Glob, file } from "bun";
import { execSync } from "node:child_process";

const glob = new Glob("tools/in/*.txt");
await $`g++ ./a.cpp -o ./a.out`;

const paths = Array.from(glob.scanSync());
for (let i = 0; i < paths.length; i++) {
  const path = paths[i];
  const filename = path.split("/").pop();
  const input = await file(path).text();
  console.log(`${i + 1}/${paths.length}`, "path:", path);
  execSync(`echo "${input}" | ./a.out > tools/report.out`);
  const buffer = execSync(
    `cd tools && cargo run -r --bin vis in/${filename} report.out`
  );
  const res = buffer.toString();
  console.log(res);
}
