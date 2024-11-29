seed=$1
if [[ $seed =~ ^[0-9]+$ ]]; then
    seed=$(printf "%04d" $seed)
fi

dir=$(cd $(dirname $0) && pwd)

cd $dir/solver
cargo build --release

cd $dir/tools
cargo run -r --bin tester ../solver/target/release/solve < in/$seed.txt > ../.out
