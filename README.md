# Oxide Boy

A gameboy emulator

## Building

You will need nightly Rust to build this project. After installing nightly rust, simply run

```sh
cargo build
```

## Running

After building you can run the project by using

```sh
cargo run
```

## TODO

* Timers
* GUI
  * Gaphics need to be implented to get past operation 0x68. The LY register (mem location 0xff44) will need to be incremented as we preform cpu cycles.
* I/O
* Be able to run through the whole boot rom
  * Fix looping where we're jumping to 0x64 forever
* Hard code the boot rom so we don't need a file for it?

### Current issues

* currently execute should be returning a u16 as are all the other fuctions. Right now there is some sort of inconsitency happening where I decided some of them would return and some wouldn't.
