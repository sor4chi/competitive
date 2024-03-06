# Masters Qual A ビジュアライザ

## Requirements

```bash
cargo install wasm-pack
npm install -g pnpm
```

## Development

```bash
pnpm i
cd wasm && wasm-pack build --target web --out-dir ../public/wasm && cd ..
pnpm dev
```

## Implementation

`wasm/src/lib.rs`に実装するもの
| 関数名 | 説明 |
|---|---|
|`gen(seed: i32) -> String`|seed を与えて String の形で入力ファイルを出力する関数|
|`vis(_input: String, _output: String, turn: usize) -> Ret`|入力・出力・ターン数を与えて、その時点のスコア・エラー文・SVG の画像を返す関数|
|`get_max_turn(_input: String, _output: String) -> usize`|入力・出力を与えたときに、その出力が何ターンからなるものかを計算する関数(スライダーで動かすときなどに必要)|

```bash
wasm-pack build --target web --out-dir ../public/wasm
```
