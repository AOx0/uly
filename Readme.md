# uly

Command line tool to create and open local Ulysses documents.

## Installation
```
cargo install --git https://github.com/AOx0/uly
```

## Usage
```
uly 1.0.1
Create local Ulysses documents from the terminal

USAGE:
    uly <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    create    Create a new Ulysses document [alias: c]
    help      Print this message or the help of the given subcommand(s)
    open      Open an already existing Ulysses document [alias: o]

EXAMPLES:
    uly create Test     # Creates a new document named Test.ulysses
    uly c Test          # Creates a new document named Test.ulysses
    uly c -o            # Create a new document and open it with Ulysses
    uly o Test          # Open the existing file Test.ulysses with Ulysses
    uly help c          # Display help msg for subcommand 'create'
```

