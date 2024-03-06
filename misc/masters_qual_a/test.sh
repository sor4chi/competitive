g++ a.cpp -o a.out && ./a.out <test.in >test.out
# g++ b.cpp -o a.out && ./a.out <test.in >test.out
# g++ generate_board.cpp -o a.out && ./a.out <test.in >test.out

# cd tools && cargo run -r --bin score ../test.in ../test.out

# SEED="0010"
# g++ b.cpp -o a.out && ./a.out <"tools/in/$SEED.txt" >test.out
# cd tools && cargo run -r --bin score "in/$SEED.txt" "../test.out"
