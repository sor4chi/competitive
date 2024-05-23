# g++ a.cpp -o a.out && ./a.out < test.in > test.out
cd solver && cargo run < ../test.in > ../test.out
