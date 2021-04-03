# fit
FIT file parser written in Rust.

## Example
```rust
extern crate fit;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let file = std::fs::File::open("tests/20210218_zwift.fit").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let fit = crate::fit::read(&mut reader);
}
```
## Current Status
Work in progress.

## Revision History
* n/a

## License
This project is licensed under the [MIT license](./LICENSE).
