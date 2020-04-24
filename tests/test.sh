#!/bin/sh

if [ $# -eq 0 ] 
    then 
        echo "provide the package name to be run/tested"
        exit 1
fi

export CC=aarch64-elf-gcc
if cargo xbuild --release --package $1; then
    cd $1
    cargo objcopy -- -O binary ..\\target\\aarch64-unknown-none\\release\\kernel ..\\target\\kernel8.img
    qemu-system-aarch64 -M raspi3 -kernel ../target/kernel8.img -serial null -serial stdio
fi