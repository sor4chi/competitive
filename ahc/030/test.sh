#!/bin/sh

set -eu

function usage {
cat <<EOM
Usage: $(basename "$0") [OPTION]...
  -h         Display help
  -t VALUE   A target c++ file (eg: a.cpp)
  -i VALUE   A input file (eg: 0000.txt)
EOM

  exit 2
}

DIR=$(dirname $0)
TARGET=""
INPUT=""

while getopts ":t:i:h" optKey; do
  case "$optKey" in
    t)
        TARGET=$OPTARG
      ;;
    i)
        INPUT=$OPTARG
      ;;
    '-h'|'--help'|* )
      usage
      ;;
  esac
done

if [ -z "$TARGET" ]; then
  echo "Target file is required"
  usage
fi

if [ -z "$INPUT" ]; then
  echo "Input file is required"
  usage
fi

function run {
  rm -f a.out
  g++ $TARGET -o a.out
  cd tools
  cargo run -r --bin tester ../a.out <in/$INPUT >out.txt
}

run || (cd $DIR && exit 1)
cd $DIR
exit 0
