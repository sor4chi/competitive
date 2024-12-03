seed=$1
if [ -z $seed ]; then
    seed=0
fi
if [[ $seed =~ ^[0-9]+$ ]]; then
    seed=$(printf "%04d" $seed)
fi

dir=$(cd $(dirname $0) && pwd)

cd $dir/solver
cargo build --release

cd $dir/tools
cargo run -r --bin tester $dir/solver/target/release/solve $seed <in/$seed.txt >$dir/.out
