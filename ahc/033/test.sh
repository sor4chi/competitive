# g++ a.cpp -o a.out && ./a.out < test.in > test.out
# cd solver && cargo run --bin solver <../test.in >../test.out
cd solver && cargo build --release
# measure time
time ./target/release/solver <../test.in >../test.out
