#!/bin/bash

clang -target riscv64 -mno-relax -march=rv64g -c test.s
clang -Wl,-Ttext=0x0 -target riscv64 -mno-relax -nostdlib -march=rv64g -o test test.s
llvm-objcopy -O binary test test.bin