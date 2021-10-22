#! /bin/sh

RED='\033[1;31m'
GREEN='\033[1;32m'
NC='\033[0m' # No Color

cargo run /csc/411/images/sudoku/sudoku.pgm > /dev/null 2>&1
result=$?
if [ $result -eq 0 ];
then
    echo ${GREEN}PASSED${NC} TEST 1: good_grid_file
else
    echo ${RED}FAILED${NC} TEST 1: good_grid_file
fi

cargo run /csc/411/images/sudoku/bad.pgm > /dev/null 2>&1
result=$?
if [ $result -eq 1 ];
then
    echo ${GREEN}PASSED${NC} TEST 2: bad_grid_file
else
    echo ${RED}FAILED${NC} TEST 2: bad_grid_file
fi

./good_grid_stdin.sh > /dev/null 2>&1
result=$?
if [ $result -eq 0 ];
then
    echo ${GREEN}PASSED${NC} TEST 3: good_grid_stdin
else
    echo ${RED}FAILED${NC} TEST 3: good_grid_stdin
fi

./bad_grid_stdin.sh > /dev/null 2>&1
result=$?
if [ $result -eq 1 ];
then
    echo ${GREEN}PASSED${NC} TEST 4: bad_grid_stdin
else
    echo ${RED}FAILED${NC} TEST 4: bad_grid_stdin
fi