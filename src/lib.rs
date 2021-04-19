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

mod fit;

#[cfg(test)]
mod tests {
    fn callback(timestamp: u32, global_message_num: u16, local_msg_type: u8, fields: Vec<crate::fit::FieldValue>) {
        let global_message_names = crate::fit::init_global_msg_name_map();

        match global_message_names.get(&global_message_num) {
            Some(name) => print!("Callback for global message num {} ({}), local message type {}, timestamp {}, values: ", global_message_num, name, local_msg_type, timestamp),
            None => print!("Callback for global message num {}, local message type {} timestamp {}, values: ", global_message_num, local_msg_type, timestamp)
        }

        for field in fields {
            match field.field_type {
                crate::fit::FieldType::FieldTypeNotSet => { print!("[not set] "); },
                crate::fit::FieldType::FieldTypeInt => { print!("{} ", field.num_int); },
                crate::fit::FieldType::FieldTypeFloat => { print!("{} ", field.num_float); },
                crate::fit::FieldType::FieldTypeByteArray => {},
                crate::fit::FieldType::FieldTypeStr => { print!("\"{}\" ", field.string); },
            }
        }
        println!("");
    }

    #[test]
    fn file1_zwift() {
        let file = std::fs::File::open("tests/20210218_zwift.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let fit = crate::fit::read(&mut reader, callback);

        match fit {
            Ok(fit2) => {
                fit2.header.print();
            }
            _ => (),
        }
    }

    #[test]
    fn file2_bike() {
        let file = std::fs::File::open("tests/20191117_bike_wahoo_elemnt.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let fit = crate::fit::read(&mut reader, callback);

        match fit {
            Ok(fit2) => {
                fit2.header.print();
            }
            _ => (),
        }
    }

    #[test]
    fn file3_swim() {
        let file = std::fs::File::open("tests/20200529_short_ocean_swim.fit").unwrap();
        let mut reader = std::io::BufReader::new(file);
        let fit = crate::fit::read(&mut reader, callback);

        match fit {
            Ok(fit2) => {
                fit2.header.print();
            }
            _ => (),
        }
    }
}
