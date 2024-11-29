g++ src/main.cpp -o a.out

SEED=$1
if [ -z "$SEED" ]; then
    SEED=1
fi

java -jar tester.jar -exec "./a.out" -seed $SEED -delay 1
