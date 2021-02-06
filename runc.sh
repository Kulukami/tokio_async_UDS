#!/bin/bash

cargo build --release
if [ $? -ne 0 ]; then
    echo "failed"
    exit
else
    echo "succeed"
fi
clear
./target/release/async_client