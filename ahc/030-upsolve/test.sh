# get seed from argument
if [ $# -ne 1 ]; then
    echo "Usage: $0 <seed>"
    exit 1
fi

# validate seed
if ! [[ $1 =~ ^[0-9]+$ ]]; then
    echo "Invalid seed: $1"
    exit 1
fi

ROOT=$(dirname $(realpath $0))

# check if the seed exists
if [ ! -e $ROOT/tools/in/$1.txt ]; then
    echo "Seed $1 does not exist"
    exit 1
fi

cd $ROOT/solver && cargo build --release
cd $ROOT/tools && cargo run -r --bin tester $ROOT/solver/target/release/solve <in/$1.txt >out/$1.txt
