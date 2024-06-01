SEED=0003
# cd solver && cargo build && cp target/debug/solver ../bin && cd ../tools && cargo run --release --bin tester ../bin <in/$SEED.txt >out/$SEED.txt
# 上のコマンドの標準エラー出力をファイルに保存する
cd solver && cargo build && cp target/debug/solver ../bin && cd ../tools && cargo run --release --bin tester ../bin <in/$SEED.txt >out/$SEED.txt 2>err/$SEED.txt
# 標準エラー出力から "Score = " の後ろの数字だけを取り出してファイルに保存する
cat err/$SEED.txt | grep "Score = " | sed -e "s/Score = //g" | echo $(</dev/stdin)
