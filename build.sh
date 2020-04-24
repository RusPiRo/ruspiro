#!/bin/sh
#-------------------------------------------------------------------------------
#
#-------------------------------------------------------------------------------

if [ $# -eq 0 ] 
    then 
        echo "provide the package name to be build"
        exit 1
fi

export CC=aarch64-elf-gcc
cargo xbuild --release --package $1

for var in "$@"
do
    if [ $var = "kernel" ]
        then
            cd ./kernel
            cargo objcopy -- -O binary ..\\target\\aarch64-unknown-linux-gnu\\release\\kernel ..\\target\\kernel8.img
    fi

    if [ $var = "qemu" ]
        then
            qemu-system-aarch64 -M raspi3 -kernel ../target/kernel8.img -serial null -serial stdio
    fi
done

    

