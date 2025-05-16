# Hyperlight KubeCon NA 2024 Demo

This repo houses the code for the demo we will be presenting during our keynote at KubeCon NA 2024.

## Project Structure

```plaintext
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── demo-guest
├── demo-main
├── puml
└── target
```

- `demo-guest`: `x86_64-unknown-none` native Hyperlight guest application.
- `demo-main`: HTTP server using Hyperlight's host SDK.

## `demo-guest`

Hyperlight executes arbitrary code safely and quickly. It does that by booting up an OS-less VM that 
executes functions.

- `demo-guest` exports two _guest functions_:
    - `PrintOutput`, and
    - `DereferenceRawNullPointer`.

> Note: OS-less VMs cannot make any syscalls, so we escape the VM to make _host function calls_ to, say,
> print to standard output.

## `demo-main`

![Architecture](https://www.plantuml.com/plantuml/png/ZP9RRzfE4CNVzrFCt_zdgr2Lvg88XOij7WggKZLvQ2gqjGTiiHUwkmv5LRvxPsrhWm3JmijAypjd1iwvjuuRLqd1_jiQlfOS1D_hoe6LwBZYh2Xp1ElGe7RxBLh6xAPKMswu18CPCMk1y9i1VSOyswoDhbG-qKARkxi7Sa8x7CB_uzxe-YPXvx65VfnxQ3eO9BrUDpDxybIQVXWGTHwRVVB8-sGokUft8erFFxdffMatGy_SSwFfz3hvsCqzSIERkwxIGVTTN_WAtRus9BjH_p8tVLXyRap7wOKVEozjvh79m7_PAsCySzh0Luk6iRTycY3W815kmTJlDhitDbfeU9n7478XLEdbYsp98_fTWKBeUAUDo4aKWWgoPR6hTNJSYPGQPKIvBbm4RNQaAiI_R0f1rWfQ4aIdmaotYX1hK2tzh5EuesmROSWv2Fqi25_PsOjj7IOvX623GpEwS7IE1bUnuRjOLKB4MsSYE2d8eTSoDshxjujUP9nHXLm4eJ-jYu-g7hLfaO7I3k67TavBqAkZJRUG_CAihb2fW8F_Fs0M4gKECUPx2B69XstPRY6IV8FsMHdyMQ5rzjb5h8alphyLPPwZ-avuMCN2wnW8zn9oE8jfMXUFtQFAAod-0000)

# How to run?

```
cd demo-main
cargo run
```

# How to debug?

To spawn a Hyperlight VM and attach a debugger to it, you:
1. Start a devcontainer
2. Run the demo with the `gdb` feature:
  ```
  cd demo-main
  cargo run --features gdb
  ```
3. Open a separate terminal and send an HTTP request to the demo server to spawn a cold Hyperlight VM:
  ```
  curl http://localhost:3030/hyperlight/hello-world/cold
  ```
4. Attach a debugger to the spawned VM by using one of the `Run and Debug` configurations in your VSCode editor:
  - `Remote LLDB attach`
  - `Remote GDB attach`
5. Enjoy debugging!
