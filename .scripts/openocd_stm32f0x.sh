#!/bin/bash

openocd -f $OPENCD_ROOT/interface/stlink-v2.cfg -f $OPENCD_ROOT/target/stm32f0x.cfg
