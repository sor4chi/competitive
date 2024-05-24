# g++ a.cpp -o a.out && ./a.out < test.in > test.out
cd solver && cargo run --bin two_cranes < ../test.in > ../test.out
