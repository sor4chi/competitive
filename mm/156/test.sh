seed=$1

if [ -z "$seed" ]; then
    seed=0
fi

echo "Compiling Reversi.cpp..."
g++ Reversi.cpp -o a.out

echo "Testing seed $seed..."
java -jar tester.jar -exec "./a.out" -seed $seed -delay 20
