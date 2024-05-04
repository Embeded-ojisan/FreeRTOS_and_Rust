#!/bin/bash

# --以下はclangのオプション
# https://solid.kmckk.com/SOLID/doc/latest/solid_toolchain/overview.html
bindgen ./freertos/FreeRTOS/FreeRTOS/Source/include/FreeRTOS.h -- -I/usr/include -I./ -I./freertos/FreeRTOS/FreeRTOS/Source/include/ -I./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3 > ./FreeRTOS.rs