#!/bin/sh

cd "$1"

cd "./rs-re"

cargo build

cd "../"

clang "./main.c" -c

clang "./main.o" "./rs-re/target/debug/librs_re.a"
