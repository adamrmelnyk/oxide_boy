# GB Emulator

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

* make registers private
* Implement the rest of the operations
* GUI
* I/O

### Current issues

* currently execute should be returning a u16 as are all the other fuctions. Right now there is some sort of inconsitency happening where I decided some of them would return and some wouldn't.
