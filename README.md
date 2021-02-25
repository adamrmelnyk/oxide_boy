# Oxide Boy

A gameboy / DMG emulator

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

Note: Without a ROM, this emulator will lock up at instruction 0xE9 in the boot ROM.

## TODO

* Implement DMA Transfer
* MBC1
  * Read a save file for the RAM if one exists (And if it has a battery)
* Timers
  * The last thing the timer needs to do is trigger the interrupt from the step function which is marked with a TODO
* GUI
  * Render tiles
  * Render sprites
  * Add a library to display everything
* Add the cartrige to the device
  * Currently we stop at the anti-piracy step when the boot rom checks to see if the nintendo logo in the cartrige is correct.
* Sound
  * Step function needs to be implemented
* I/O
* CLI args so we can accept a ROM / Cartridge
* Shutdown after locking up at 0xe9 of the boot ROM
* Hard code the boot rom so we don't need a file for it?

### Resources

This emulator couldn't have been built without the help of many others (this is an inexhaustive list):

* [Challenging projects](https://web.eecs.utk.edu/~azh/blog/challengingprojects.html): The Blog post that inspired the project.
* [Rylev's book](https://rylev.github.io/DMG-01/public/book/introduction.html): This book has a lot of TODO's but it was a good starting point that you can build off of. The author also did a [talk](https://media.ccc.de/v/rustfest-rome-3-gameboy-emulator#t=1551) on the emulator project.
* [GB CPU Docs](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
* [GB OP Codes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Emudev](https://emudev.de/gameboy-emulator/overview/)
* [The Ultimate Gameboy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&t=29m12s)
* [GBDev.io](https://gbdev.io/)
* [An archived page](https://web.archive.org/web/20181011215339/http://www.codeslinger.co.uk/pages/projects/gameboy.html) that contains lots of documentation and implementation details for parts of the DMG.
