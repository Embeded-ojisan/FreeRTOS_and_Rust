#!/bin/bash

# --以下はclangのオプション
# https://solid.kmckk.com/SOLID/doc/latest/solid_toolchain/overview.html
bindgen ./freertos/FreeRTOS/FreeRTOS/Source/include/task.h -- --include ./freertos/FreeRTOS/FreeRTOS/Source/include/FreeRTOS.h -I/usr/include -I./ -I./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_
CM3 > ./bindings/task.rs