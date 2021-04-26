# fit_file
FIT file parser written in Rust. FIT (Flexible and Interoperable Data Transfer) is a binary file format that is commonly used to exchange fitness data, such as that from a sports watch or bike computer.

## Example
```rust
extern crate fit;

use std::io::BufReader;
use std::fs::File;

/// Called for each record message as it is processed.
fn callback(timestamp: u32, global_message_num: u16, local_msg_type: u8, fields: Vec<crate::fit::FieldValue>) {
    if global_message_num == crate::fit::GLOBAL_MSG_NUM_SESSION {
        let msg = crate::fit::FitSessionMsg::new(fields);
        let sport_names = crate::fit::init_sport_name_map();
        let sport_id = msg.sport.unwrap();

        println!("Sport: {}", sport_names.get(&sport_id).unwrap());
    }
    else if global_message_num == crate::fit::GLOBAL_MSG_NUM_RECORD {
        let msg = crate::fit::FitRecordMsg::new(fields);

        println!("Timestamp: {} Latitude: {} Longitude: {}", timestamp, crate::fit::semicircles_to_degrees(msg.position_lat.unwrap()), crate::fit::semicircles_to_degrees(msg.position_long.unwrap()));
    }
}

/// Context structure. An instance of this will be passed to the parser and ultimately to the callback function so we can use it for whatever.
struct Context {
    num_records_processed: u16,
}

impl Context {
    pub fn new() -> Self {
        let msg = Context{ num_records_processed: 0 };
        msg
    }
}

fn main() {
    let file = std::fs::File::open("tests/20210218_zwift.fit").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut context = Context::new();
    let context_ptr: *mut c_void = &mut context as *mut _ as *mut c_void;
    let fit = crate::fit::read(&mut reader, callback, context_ptr);
}
```
## Current Status
Work in progress.

## Revision History
* 0.1.0 - Basic functionality, can extract position and timestamp data for run and cycling activities.

## License
This project is licensed under the [MIT license](./LICENSE).
