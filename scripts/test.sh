#!/bin/sh

# Capture the start time
START_TIME=$(date +%s)

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
  status=$(head -n 1 $test)
  status=${status:8}
  output=$($EXE $test)

  if [ $? -eq $status ]; then
    echo "[PASS] $test_name"
    ((PASS=PASS+1))
  else
    echo "[FAIL] $test_name"
    echo $output
    ((FAIL=FAIL+1))
  fi
done


END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

if [ $PASS -eq $TOT ]; then
  echo "[OK] $PASS/$TOT tests passed; duration: $(($DURATION))s"
else
  echo "[FAIL] $FAIL/$TOT tests failed; duration: $(($DURATION))s"
fi
