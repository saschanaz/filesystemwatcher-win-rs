# filesystemwatcher-win-rs
A sample ReadDirectoryChangesW usage in Rust.

## How to use

1. `cargo run (dirname)`
2. You'll see some logs like the following:
    ```
    File added
    Name: b3zHTnIi.tar.part
    File added
    Name: firefox-99.0a1.en-US.linux-i686.awsy.tests(5).tar.gz
    File removed
    Name: firefox-99.0a1.en-US.linux-i686.awsy.tests(5).tar.gz
    File renamed
    Name: b3zHTnIi.tar.part
    File renamed
    Name: firefox-99.0a1.en-US.linux-i686.awsy.tests(5).tar.gz
    ```
