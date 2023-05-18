# 🚀 hld - hdu Link eDitor

`hld` is a small but powerful linker written in Rust. It can help you link multiple object files together to produce an executable or shared library.

## Installation

You can install `hld` using the `cargo` package manager:

```
cargo install hld
```

## Usage

### Command-line Options

`hld` supports the following command-line options:

```
USAGE:
    hld [OPTIONS] <OUTPUT> --entry <SYMBOL>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --entry <SYMBOL>      Specify the entry symbol
    -L, --library-path <DIR>  Add a directory to the library search path
    -l, --library <LIB>       Link with a library
    -o, --output <OUTPUT>     Set the output file name
    -T, --script <FILE>       Use a linker script

ARGS:
    <OUTPUT>    Set the output file name

```

### Examples

Link multiple object files:

```
hld -o my_program a.o b.o c.o
```

Link a shared library:

```
hld -o my_shared_library.so a.o b.o c.o -shared
```

Use a custom linker script:

```
hld -o my_program a.o b.o c.o -T custom.ld
```

Add library search paths and link libraries:

```
hld -o my_program a.o b.o c.o -L /usr/local/lib -l my_lib
```

Specify the entry symbol:

```
hld -o my_program a.o b.o c.o -e main
```

## Contributing

Contributions are welcome! Please submit bug reports and pull requests. Before submitting a pull request, please read our contribution guidelines.

## License

`hld` is licensed under the MIT license. Please see the LICENSE file for more information.
