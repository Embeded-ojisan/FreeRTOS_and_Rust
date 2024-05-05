# FreeRTOS_and_Rust
FreeRTOSとRustを繋げたものです。

# 使い方
- ① bindgenを利用してFreeRTOSとRustの中だつもの(バインディング)を生成
```
$ cd script && ./bindgen.sh && cd ../
```
- ② buildを実施
```
$ cargo build
```
- ③-1 qemuを起動
```
$ ./qemu_run.sh
```
- ③-2 qemuと接続(別ターミナルで実施)
```
$ gdb-multiarch
(gdb)  target remote localhost:1234
(gdb)  file target/thumbv7m-none-eabi/debug/FreeRTOS_and_Rust
(gdb)  b Reset
```