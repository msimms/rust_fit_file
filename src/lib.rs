// by Michael J. Simms
// Copyright (c) 2021 Michael J. Simms

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod fit_file;

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::collections::HashMap;
    extern crate csv;

    /// Called for each record message as it is processed.
    fn callback(timestamp: u32, global_message_num: u16, local_msg_type: u8, fields: Vec<crate::fit_file::FitFieldValue>, context: *mut c_void) {

        if global_message_num == crate::fit_file::GLOBAL_MSG_NUM_SESSION {
            let msg = crate::fit_file::FitSessionMsg::new(fields);
            let sport_names = crate::fit_file::init_sport_name_map();
            let sport_id = msg.sport.unwrap();

            println!("Sport: {}", sport_names.get(&sport_id).unwrap());
        }
        else if global_message_num == crate::fit_file::GLOBAL_MSG_NUM_RECORD {
            let msg = crate::fit_file::FitRecordMsg::new(fields);

            println!("Timestamp: {} Latitude: {} Longitude: {}", timestamp, crate::fit_file::semicircles_to_degrees(msg.position_lat.unwrap()), crate::fit_file::semicircles_to_degrees(msg.position_long.unwrap()));

            // Increment the number of records processed.
            let data: &mut Context = unsafe { &mut *(context as *mut Context) };
            data.num_records_processed = data.num_records_processed + 1;
        }
        else {
            let global_message_names = crate::fit_file::init_global_msg_name_map();

            match global_message_names.get(&global_message_num) {
                Some(name) => print!("Callback for {} message, local message type {}, Timestamp {}, Values: ", name, local_msg_type, timestamp),
                None => print!("Callback for Global Message Num {}, Local Message Type {} Timestamp {}, Values: ", global_message_num, local_msg_type, timestamp)
            }

            for field in fields {
                print!("{} ", field.field_def);

                match field.field_type {
                    crate::fit_file::FieldType::FieldTypeNotSet => { print!("[not set] "); },
                    crate::fit_file::FieldType::FieldTypeUInt => { print!("{} ", field.num_uint); },
                    crate::fit_file::FieldType::FieldTypeSInt => { print!("{} ", field.num_sint); },
                    crate::fit_file::FieldType::FieldTypeFloat => { print!("{} ", field.num_float); },
                    crate::fit_file::FieldType::FieldTypeByteArray => {},
                    crate::fit_file::FieldType::FieldTypeStr => { print!("\"{}\" ", field.string); },
                }
            }
            println!("");
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

    #[test]
    fn file1_zwift() {
        let file = std::fs::File::open("tests/20210218_zwift.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut context = Context::new();
        let context_ptr: *mut c_void = &mut context as *mut _ as *mut c_void;
        let fit = crate::fit_file::read(&mut reader, callback, context_ptr);

        match fit {
            Ok(fit) => {
                fit.header.print();
                println!("Num records processed: {}", context.num_records_processed);
            }
            _ => { println!("Error"); },
        }
    }

    #[test]
    fn file2_bike() {
        let file = std::fs::File::open("tests/20191117_bike_wahoo_elemnt.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut context = Context::new();
        let context_ptr: *mut c_void = &mut context as *mut _ as *mut c_void;
        let fit = crate::fit_file::read(&mut reader, callback, context_ptr);

        match fit {
            Ok(fit) => {
                fit.header.print();
                println!("Num records processed: {}", context.num_records_processed);
            }
            _ => { println!("Error"); },
        }
    }

    #[test]
    fn file3_swim() {
        let file = std::fs::File::open("tests/20200529_short_ocean_swim.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut context = Context::new();
        let context_ptr: *mut c_void = &mut context as *mut _ as *mut c_void;
        let fit = crate::fit_file::read(&mut reader, callback, context_ptr);

        match fit {
            Ok(fit) => {
                fit.header.print();
                println!("Num records processed: {}", context.num_records_processed);
            }
            _ => (),
        }
    }


    fn print_messag_struct(name: String, field_map: &HashMap::<String, u8>) {
        println!("pub struct {} {{", name);
        for (field_name, field_id) in field_map {
            println!("    pub {}: Option<>,", field_name);
        }
        println!("}}");
        println!("");
        println!("impl {} {{", name);
        println!("");
        println!("    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.");
        println!("    pub fn new(fields: Vec<FitFieldValue>) -> Self {{");
        print!("        let mut msg = {} {{", name);
        let mut split_count = 0;
        for (field_name, _field_id) in field_map {
            print!("{}: None, ", field_name);
            if split_count % 3 == 0 {
                println!("");
                print!("            ");
            }
            split_count = split_count + 1;
        }
        println!("");
        println!("        }}");
        println!("");
        println!("        for field in fields {{");
        println!("            match field.field_def {{");
        for (field_name, field_id) in field_map {
            println!("                {} => {{ msg.{} = Some(field.get ()); }},", field_id, field_name);
        }
        println!("");
        println!("            }}");
        println!("        }}");
        println!("        msg");
        println!("    }}");
        println!("}}");
        println!("");
    }

    #[test]
    fn create_message_structs() {
        let file_path = "tests/Messages-Table.csv";
        let file = match std::fs::File::open(&file_path) {
            Err(why) => panic!("Couldn't open {} {}", file_path, why),
            Ok(file) => file,
        };

        let mut reader = csv::Reader::from_reader(file);
        let mut current_msg_name = String::new();
        let mut field_map = HashMap::<String, u8>::new();

        for record in reader.records() {
            let record = record.unwrap();

            // First column is the message name.
            let msg_name: String = record[0].parse().unwrap();
            if msg_name.len() > 0 {

                // Print the previous definition, if there is one.
                if current_msg_name.len() > 0 {
                    print_messag_struct(current_msg_name, &field_map);
                }

                current_msg_name = String::from(msg_name);
            }
            else {
                let field_id = &record[1];
                if field_id.len() > 0 {
                    let field_id_num: u8 = field_id.parse::<u8>().unwrap();
                    let field_name: String = record[2].parse().unwrap();

                    field_map.insert(field_name, field_id_num);
                }
            }
        }
    }
}
