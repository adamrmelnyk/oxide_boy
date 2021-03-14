# Oxide Boy

A gameboy / DMG emulator

This project is currently in progress and has a few missing parts. Currently there is no sound or user input, though the vast majority of the project including the run loop and CPU instructions are implemented. Currently running only draws tiles, but not sprites, and executes the instructions in the console if logging is enabled.

## Building

You will need nightly Rust to build this project. After installing nightly rust, simply run

```sh
cargo build
```

## CLI

```sh
oxide_boy 0.1.0

USAGE:
    oxide_boy <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    default    For development purposes: Runs the default rom at src/dmg/rom/DEFAULT_ROM.bin
    help       Prints this message or the help of the given subcommand(s)
    run        Runs the specified ROM
```

## Running

After building you can run the project by using:

```sh
oxide_boy run my_file.bin
```

or for development, to use the default rom file:

```sh
oxide_boy default
```

you can also run using the start scipt:

```sh
./start.sh
```

Note: Without a ROM, this emulator will lock up at instruction 0xE9 in the boot ROM. To run the default rom you will need to place the ROM at `/src/dmg/rom/DEFAULT_ROM.bin`. You can find some roms made for testing [here](https://github.com/retrio/gb-test-roms)

## Testing

Currently running tests requires a rom to be present at `/src/dmg/rom/DEFAULT_ROM.bin`. Some of the tests will fail without this, but will likely change in the future.

```sh
cargo test
```

## TODO

* MBC1
  * Read a save file for the RAM if one exists (And if it has a battery)
* Timers
  * Trigger the interrupt from the step function
* GUI
  * Render sprites
* Sound
  * Step function needs to be implemented
* I/O
  * Integrate minifb with the joypad to read keyboard input
  * Joypad step function
* Shutdown after locking up at 0xe9 of the boot ROM

### Resources

This emulator couldn't have been built without the help of many others (this is an inexhaustive list):

* [Challenging projects](https://web.eecs.utk.edu/~azh/blog/challengingprojects.html): The Blog post that inspired the project.
* [Rylev's book](https://rylev.github.io/DMG-01/public/book/introduction.html): This book has a lot of TODO's but it was a good starting point that you can build off of. The author also did a [talk](https://media.ccc.de/v/rustfest-rome-3-gameboy-emulator#t=1551) on the emulator project.
* [Pan Docs](https://gbdev.io/pandocs): An excellent guide with a lot of information on the nuts and bolts of the DMG and the primary source for this project.
* [GB CPU Docs](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
* [GB OP Codes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Emudev](https://emudev.de/gameboy-emulator/overview/)
* [The Ultimate Gameboy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&t=29m12s)
* [Boot ROM](https://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM) Walk through of the operations in the DMG boot rom.
* [Codeslingers implementation](https://web.archive.org/web/20181011215339/http://www.codeslinger.co.uk/pages/projects/gameboy.html) that contains lots of documentation and implementation details for parts of the DMG.
