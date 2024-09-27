seed=$1
if [[ $seed =~ ^[0-9]+$ ]]; then
    seed=$(printf "%04d" $seed)
fi

cd solver
cargo build --release

rm -rf figure
mkdir -p figure
time ./target/release/solve <../in/$seed.txt >../.out
