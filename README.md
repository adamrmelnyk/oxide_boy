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

### Resources

This emulator couldn't have been built without the help of many others (this is an inexhaustive list):

* [Challenging projects](https://web.eecs.utk.edu/~azh/blog/challengingprojects.html): The Blog post that inspired the project.
* [GB CPU Docs](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
* [GB OP Codes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Emudev](https://emudev.de/gameboy-emulator/overview/)
* [The Ultimate Gameboy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&t=29m12s)
* [GBDev.io](https://gbdev.io/)
