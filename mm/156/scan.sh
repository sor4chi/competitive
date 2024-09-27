echo "Compiling InputScanner.cpp..."
g++ InputScanner.cpp -o a.out

for seed in $(seq 1 100); do
    echo "Scanning seed $seed..."
    timeout 1s java -jar tester.jar -exec "./a.out in/$seed" -seed $seed -novis
done
