# fit_file
FIT file parser written in Rust. FIT (Flexible and Interoperable Data Transfer) is a binary file format that is commonly used to exchange fitness data, such as that from a sports watch or bike computer.

This implementation uses a callback, as shown in the example below. The callback is an efficient method for dealing with large files as it allows us to avoid returning a large array containing each of the records that was processed.

## Example
```rust
use fit_file as fit;
use fit_file::fit_file;

/// Called for each record message as it is processed.
fn callback(timestamp: u32, global_message_num: u16, _local_msg_type: u8, _message_index: u16, fields: Vec<crate::fit_file::FitFieldValue>, data: &mut Context) {
    if global_message_num == crate::fit::GLOBAL_MSG_NUM_SESSION {
        let msg = crate::fit::FitSessionMsg::new(fields);
        let sport_names = crate::fit::init_sport_name_map();
        let sport_id = msg.sport.unwrap();

        println!("Sport: {}", sport_names.get(&sport_id).unwrap());
    }
    else if global_message_num == crate::fit::GLOBAL_MSG_NUM_RECORD {
        let msg = crate::fit::FitRecordMsg::new(fields);

        data.num_records_processed += 1;

        println!("Timestamp: {} Latitude: {} Longitude: {}", timestamp, crate::fit::semicircles_to_degrees(msg.position_lat.unwrap()), crate::fit::semicircles_to_degrees(msg.position_long.unwrap()));
    }
}

/// Context structure. An instance of this will be passed to the parser and ultimately to the callback function so we can use it for whatever.
struct Context {
    num_records_processed: u16,
}

impl Context {
    pub fn new() -> Self {
        let context = Context{ num_records_processed: 0 };
        context
    }
}

fn main() {
    let file = std::fs::File::open("tests/20210218_zwift.fit").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut context = Context::new();
    crate::fit::read(&mut reader, callback, &mut context).unwrap();
    println!("{} records processed", context.num_records_processed);
}
```
## Current Status
Work in progress.

## Revision History
* 0.5.0 - Supports the Length message.
* 0.4.0 - Returns developer defined fields, though doesn't really do anything with them.
* 0.3.0 - Fixed numerous issues interpreting the specification. Now passing with several Garmin test files, instead of just the original Wahoo test file.
* 0.2.0 - Updated critical message structures by implementing a code generator that is just barely good enough.
* 0.1.0 - Basic functionality, can extract position and timestamp data for run and cycling activities.

## License
This project is licensed under the [MIT license](./LICENSE).
