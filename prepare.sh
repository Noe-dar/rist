#!/bin/bash

cd riscv-opcodes || exit
make EXTENSIONS="$1" latex

mv instr_dict.yaml ../crates/rist_core