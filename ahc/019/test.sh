SEED=0
PADDED_SEED=$(printf "%04d" $SEED)
INPUT_FILE=in/$PADDED_SEED.txt
OUTPUT_FILE=out/$PADDED_SEED.txt
g++ a.cpp -o a.out && cd tools && ../a.out < $INPUT_FILE > $OUTPUT_FILE
cargo run --release --bin vis $INPUT_FILE $OUTPUT_FILE
