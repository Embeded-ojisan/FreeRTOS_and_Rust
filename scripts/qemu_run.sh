# 対向側
## gdb-multiarchでgdb内に入る
### target remote localhost:1234
### file target/thumbv7m-none-eabi/debug/FreeRTOS_and_Rust
### b Reset
### continue

qemu-system-arm \
      -d cpu\
      -d exec\
      -cpu cortex-m3 \
      --machine lm3s6965evb \
      -nographic \
      -semihosting-config enable=on,target=native \
      -gdb tcp::1234 \
      -S \
      -kernel ../target/thumbv7m-none-eabi/debug/FreeRTOS_and_Rust