seed=$1
seed=$(printf "%04d" $seed)

pbcopy <./tools/in/$seed.txt
echo "Copied $seed.txt to clipboard"
