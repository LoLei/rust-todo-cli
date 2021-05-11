# rust-todo-cli
First project that's more than "hello world".

Performs CRUD operations on items in a database. The database implementation is easily
interchangeable. `lib.rs` could also be used by a different main binary, e.g. a web server/API.

Since this is just a practice project and the usability and use case is
questionable, it is not published on crates.io, but it can still be tested
locally by cloning this repository and running some of the following commands.

## Usage

### Compile
```
cargo build --release
```

### Run
```
./target/release/rust-todo-cli
# Or compile & run:
cargo run --release --
```

For simplicity's sake, below examples use the alias `rtc` for the binary.

### Help
```
rtc -h
Rust TODO CLI 0.1
Lorenz Leitner
CLI for managing TODOs

USAGE:
    rtc [OPTIONS] <--add <NAME>...|--delete <ID>|--update <ID,STATUS>|--filter <FILTERS>...|--getall>

FLAGS:
    -g, --getall     Get all items in the database
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --add <NAME>...          Add an item, return ID
    -d, --delete <ID>            Delete an item
    -f, --filter <FILTERS>...    Filter items by name, status and/or date
    -u, --update <ID,STATUS>     Update an item
```

### Examples
```
rtc -a my todo item
rtc -u 10 done
rtc -d 5
rtc -g
rtc -f status=open
rtc -f name='todo item'
rtc -f status=done after=2021-01-01 before=2021-01-05
```

### Testing
```
cargo test
```

### Clippy
Excluding some pedantic warnings:
```
./clippy.sh
```
