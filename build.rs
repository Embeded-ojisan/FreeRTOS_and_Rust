extern crate cc;

fn main() {
    cc::Build::new()
        .file("./freertos/FreeRTOS/FreeRTOS/Source/tasks.c")
        .include("/usr/include/")
        .include("./")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/include")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3")
        .compile("tasks");

    println!("cargo:rerun-if-changed=./freertos/FreeRTOS/FreeRTOS/Source/tasks.c");

    cc::Build::new()
        .file("./freertos/FreeRTOS/FreeRTOS/Source/event_groups.c")
        .include("/usr/include/")
        .include("./")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/include")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3")
        .compile("event_groups");

    println!("cargo:rerun-if-changed=./freertos/FreeRTOS/FreeRTOS/Source/event_groups.c");

    cc::Build::new()
        .file("./freertos/FreeRTOS/FreeRTOS/Source/list.c")
        .include("/usr/include/")
        .include("./")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/include")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3")
        .compile("list");

    println!("cargo:rerun-if-changed=./freertos/FreeRTOS/FreeRTOS/Source/list.c");

    cc::Build::new()
        .file("./freertos/FreeRTOS/FreeRTOS/Source/queue.c")
        .include("/usr/include/")
        .include("./")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/include")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3")
        .compile("queue");

    println!("cargo:rerun-if-changed=./freertos/FreeRTOS/FreeRTOS/Source/queue.c");

    cc::Build::new()
        .file("./freertos/FreeRTOS/FreeRTOS/Source/stream_buffer.c")
        .include("/usr/include/")
        .include("./")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/include")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3")
        .compile("stream_buffer");

    println!("cargo:rerun-if-changed=./freertos/FreeRTOS/FreeRTOS/Source/stream_buffer.c");

    cc::Build::new()
        .file("./freertos/FreeRTOS/FreeRTOS/Source/timers.c")
        .include("/usr/include/")
        .include("./")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/include")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3")
        .compile("timers");

    println!("cargo:rerun-if-changed=./freertos/FreeRTOS/FreeRTOS/Source/timers.c");

    cc::Build::new()
        .file("./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3/port.c")
        .include("/usr/include/")
        .include("./")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/include")
        .include("./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3")
        .compile("port");

    println!("cargo:rerun-if-changed=./freertos/FreeRTOS/FreeRTOS/Source/portable/GCC/ARM_CM3/port.c");
}