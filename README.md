# filesystemwatcher-win-rs
A sample ReadDirectoryChangesW usage in Rust.

## How to use

1. `cargo run (dirname)`
2. You'll see some logs like the following:
    ```
    File added
    Name: QaWJfiSR.tar.part
    FileSize: 0
    File added
    Name: firefox-99.0a1.en-US.linux-i686.awsy.tests.tar.gz
    FileSize: 0
    File removed
    Name: firefox-99.0a1.en-US.linux-i686.awsy.tests.tar.gz
    FileSize: 0
    File renamed from
    Name: QaWJfiSR.tar.part
    FileSize: 23702
    File renamed to
    Name: firefox-99.0a1.en-US.linux-i686.awsy.tests.tar.gz
    FileSize: 23702
    ```
