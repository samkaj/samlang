#!/bin/sh

ROOT_DIR=$(dirname "$0")/..
TEST_DIR=$ROOT_DIR/tests

cd $ROOT_DIR

cargo build --release
EXE=$ROOT_DIR/target/release/slang
TOT=0
PASS=0
FAIL=0

for test in $TEST_DIR/*.sk; do
  ((TOT = TOT + 1))
  test_name=$(basename $test)
  echo "Running test $test_name"
  status=$(head -n 1 $test)
  status=${status:8}
  $EXE $test > /dev/null

  if [ $? -eq $status ]; then
    echo "Test $test_name passed"
    ((PASS=PASS+1))
  else
    echo "Test $test_name failed"
    ((FAIL=FAIL+1))
  fi
done

echo "[$PASS/$TOT] passed"
