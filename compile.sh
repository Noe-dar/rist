#!/bin/bash

clang -Wl,-Ttext=0x0 -target riscv64 -mno-relax -nostdlib -march=rv64g -O3 -o test test.c
llvm-objcopy -O binary test test.bin