seed=$1

if [ -z "$seed" ]; then
    seed=0
fi

echo "Compiling Arrows.cpp..."
g++ Arrows.cpp -o a.out

echo "Testing seed $seed..."
java -jar tester.jar -exec "./a.out" -seed $seed -delay 10 -noanimate
