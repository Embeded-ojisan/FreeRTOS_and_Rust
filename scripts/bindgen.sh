#!/bin/bash
bindgen ../freertos/FreeRTOSheader.h \
    --use-core --ctypes-prefix cty \
    -- \
        --include ../freertos/FreeRTOS/FreeRTOS/Source/include/FreeRTOS.h \
        -I/usr/include \
        -I../ \
        -I../freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3 \
    > ../bindgen/FreeRTOSheader.rs
