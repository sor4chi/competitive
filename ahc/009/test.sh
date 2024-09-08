seed=$1
if [[ $seed =~ ^[0-9]+$ ]]; then
    seed=$(printf "%04d" $seed)
fi

cd solver
cargo build --release

time ./target/release/solve <../tools/in/$seed.txt >../.out
