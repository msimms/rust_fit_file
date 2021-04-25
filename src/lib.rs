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

    /// Called for each record message as it is processed.
    fn callback(timestamp: u32, global_message_num: u16, local_msg_type: u8, fields: Vec<crate::fit_file::FitFieldValue>) {

        if global_message_num == crate::fit_file::GLOBAL_MSG_NUM_SESSION {
            let msg = crate::fit_file::FitSessionMsg::new(fields);
            let sport_names = crate::fit_file::init_sport_name_map();
            let sport_id = msg.sport.unwrap();

            println!("Sport: {}", sport_names.get(&sport_id).unwrap());
        }
        else if global_message_num == crate::fit_file::GLOBAL_MSG_NUM_RECORD {
            let msg = crate::fit_file::FitRecordMsg::new(fields);

            println!("Timestamp: {} Latitude: {} Longitude: {}", timestamp, crate::fit_file::semicircles_to_degrees(msg.position_lat.unwrap()), crate::fit_file::semicircles_to_degrees(msg.position_long.unwrap()));
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

    #[test]
    fn file1_zwift() {
        let file = std::fs::File::open("tests/20210218_zwift.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let fit = crate::fit_file::read(&mut reader, callback);

        match fit {
            Ok(fit) => {
                fit.header.print();
            }
            _ => (),
        }
    }

    #[test]
    fn file2_bike() {
        let file = std::fs::File::open("tests/20191117_bike_wahoo_elemnt.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let fit = crate::fit_file::read(&mut reader, callback);

        match fit {
            Ok(fit) => {
                fit.header.print();
            }
            _ => (),
        }
    }

    #[test]
    fn file3_swim() {
        let file = std::fs::File::open("tests/20200529_short_ocean_swim.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let fit = crate::fit_file::read(&mut reader, callback);

        match fit {
            Ok(fit) => {
                fit.header.print();
            }
            _ => (),
        }
    }
}
