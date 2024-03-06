// exec using `bun run scripts/hoist-cargo.js`
import { Glob, write, file } from "bun";

const PROJECT_PATH = __dirname + "/../";

console.log("Hoisting cargo.toml paths to vscode settings.json ...");
const VSCODE_SETTING_PATH = PROJECT_PATH + ".vscode/settings.json";
const settingJson = await file(VSCODE_SETTING_PATH).json();
const cargoTomlGlob = new Glob("**/Cargo.toml");
const cargoTomlPaths = [];
for await (const path of cargoTomlGlob.scan(PROJECT_PATH)) {
  cargoTomlPaths.push(path);
}
settingJson["rust-analyzer.linkedProjects"] = cargoTomlPaths;
await write(VSCODE_SETTING_PATH, JSON.stringify(settingJson, null, 2));
console.log("Done!");
