#!/usr/bin/env bash
set -e
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

function check_leaks {
  echo "Checking $1 for memory leaks"
  valgrind --leak-check=full --num-callers=50 --log-file="$1.leaks.log" $1
  num_leaks=$(grep 'ERROR SUMMARY: [0-9]+' -Eo "$1.leaks.log" | grep '[0-9]+' -Eo)
  echo "Detected $num_leaks memory leaks"
  if (( num_leaks == 0 ))
  then
    return 0
  else
    cat $1.leaks.log
    return -1
  fi
}

check_leaks $1
