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
  * Timer step function
  * Jump, ret, call, jump_relative etc. functions need to use the conditional cycle amounts
* GUI
  * LY is the only ppu register being inc'd, the rest of the step function in the ppu needs to be implemented.
* Add the cartrige to the device
  * Currently we stop at the anti-piracy step when the boot rom checks to see if the nintendo logo in the cartrige is correct.
* Sound
* I/O
* Hard code the boot rom so we don't need a file for it?

### Resources

This emulator couldn't have been built without the help of many others (this is an inexhaustive list):

* [Challenging projects](https://web.eecs.utk.edu/~azh/blog/challengingprojects.html): The Blog post that inspired the project.
* [Rylev's book](https://rylev.github.io/DMG-01/public/book/appendix/cartridge_header.html): This book has a lot of TODO's but it was a good starting point that you can build off of. The author also did a [talk](https://media.ccc.de/v/rustfest-rome-3-gameboy-emulator#t=1551) on the emulator project.
* [GB CPU Docs](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
* [GB OP Codes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Emudev](https://emudev.de/gameboy-emulator/overview/)
* [The Ultimate Gameboy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&t=29m12s)
* [GBDev.io](https://gbdev.io/)
