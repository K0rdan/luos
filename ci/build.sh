#!/bin/bash

if [ $TARGET = x86_64-unknown-linux-gnu ]; then
  cargo build

elif [ $TARGET = thumbv6m-none-eabi ]; then
  xargo build --target $TARGET
fi
