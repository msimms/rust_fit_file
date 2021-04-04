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

use std::io::Result;
use std::io::Read;
use std::io::BufReader;

const HEADER_FILE_SIZE_OFFSET: usize = 0;
const HEADER_PROTOCOL_VERSION_OFFSET: usize = 1;
const HEADER_PROFILE_VERSION_LSB_OFFSET: usize = 2;
const HEADER_PROFILE_VERSION_MSB_OFFSET: usize = 3;
const HEADER_DATA_SIZE_LSB_OFFSET: usize = 4;
const HEADER_DATA_SIZE_1_OFFSET: usize = 5;
const HEADER_DATA_SIZE_2_OFFSET: usize = 6;
const HEADER_DATA_SIZE_MSB_OFFSET: usize = 7;
const HEADER_DATA_TYPE_0_OFFSET: usize = 8;  // .
const HEADER_DATA_TYPE_1_OFFSET: usize = 9;  // F
const HEADER_DATA_TYPE_2_OFFSET: usize = 10;  // I
const HEADER_DATA_TYPE_3_OFFSET: usize = 11; // T
const HEADER_CRC_1_OFFSET: usize = 12;
const HEADER_CRC_2_OFFSET: usize = 13;

const RECORD_HDR_NORMAL: u8 = 0x80;
const RECORD_HDR_MSG_TYPE: u8 = 0x40;
const RECORD_HDR_MSG_TYPE_SPECIFIC: u8 = 0x20;
const RECORD_HDR_RESERVED: u8 = 0x10;
const RECORD_HDR_LOCAL_MSG_TYPE: u8 = 0x0f;

#[derive(Debug, Default)]
pub struct FitHeader {
    pub header_buf: [u8; 14]
}

impl FitHeader {
    pub fn new() -> Self {
        let header = FitHeader{ header_buf: [0u8; 14] };
        header
    }

    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<()> {
        reader.read_exact(&mut self.header_buf)?;
        Ok(())
    }

    pub fn validate(&self) -> bool {
        let mut valid = self.header_buf[HEADER_DATA_TYPE_0_OFFSET] == '.' as u8;
        valid = valid && self.header_buf[HEADER_DATA_TYPE_1_OFFSET] == 'F' as u8;
        valid = valid && self.header_buf[HEADER_DATA_TYPE_2_OFFSET] == 'I' as u8;
        valid = valid && self.header_buf[HEADER_DATA_TYPE_3_OFFSET] == 'T' as u8;
        valid
    }

    pub fn print(&self) {
        for byte in self.header_buf.iter() {
            print!("{:#04x} ", byte);
        }
    }

    pub fn data_size(&self) -> u32 {
        let mut data_size = self.header_buf[HEADER_DATA_SIZE_LSB_OFFSET] as u32;
        data_size = data_size | (self.header_buf[HEADER_DATA_SIZE_1_OFFSET] as u32) << 8;
        data_size = data_size | (self.header_buf[HEADER_DATA_SIZE_2_OFFSET] as u32) << 16;
        data_size = data_size | (self.header_buf[HEADER_DATA_SIZE_MSB_OFFSET] as u32) << 24;
        data_size
    }
}

#[derive(Debug, Default)]
pub struct FitRecord {
    pub header_byte: [u8; 1]
}

impl FitRecord {
    pub fn new() -> Self {
        let rec = FitRecord{ header_byte: [0u8; 1] };
        rec
    }

    pub fn read_local_msg_type0<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<()> {
        // Next byte tells us the type of the local message.
        let mut msg_type: [u8; 1] = [0; 1];
        reader.read_exact(&mut msg_type)?;

        match msg_type[0] {
            4 => (),
            _ => (),
        }
        Ok(())
    }

    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<()> {
        // The first byte is a bit field that tells us more about the record.
        reader.read_exact(&mut self.header_byte)?;

        // Normal header or compressed timestamp header?
        // A value of zero indicates a normal header.
        if self.header_byte[0] & RECORD_HDR_NORMAL != 0 {

            // Compressed Timestamp Header.
            let time_offset = self.header_byte[0] & 0x0f;
        }
        else {

            // Data or definition message?
            // A value of zero indicates a data message.
            if self.header_byte[0] & RECORD_HDR_MSG_TYPE != 0 {

                // Definition message (5 bytes).
                // 0: Reserved
                // 1: Architecture
                // 2-3: Global Message Number
                // 4: Number of Fields
                let mut definition_header: [u8; 5] = [0; 5];
                reader.read_exact(&mut definition_header)?;

                // Read each field.
                for _i in 0..definition_header[4] {

                    // Field definition (3 bytes).
                    let field_definition: [u8; 3] = [0; 3];
                    reader.read_exact(&mut definition_header)?;
                }

                // Read the number of developer fields (1 byte).
                let mut num_dev_fields: [u8; 1] = [0; 1];
                reader.read_exact(&mut num_dev_fields)?;

                // Read each developer field.
                for _i in 0..num_dev_fields[0] {

                    // Field definition (3 bytes).
                    let field_definition: [u8; 3] = [0; 3];
                    reader.read_exact(&mut definition_header)?;
                }
            }
            else {

                // Local message type.
                let local_msg_type = self.header_byte[0] & RECORD_HDR_LOCAL_MSG_TYPE;

                match local_msg_type {
                    0 => self.read_local_msg_type0(reader)?,
                    _ => (),
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Fit {
    pub header: FitHeader,
    pub records: Vec<FitRecord>
}

impl Fit {
    pub fn new() -> Self {
        let fit = Fit{ header: FitHeader::new(), records: Vec::new() };
        fit
    }

    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<()> {
        // Read the file header.
        self.header.read(reader)?;

        // Make sure the header is valid.
        if self.header.validate() {

            // Read each record.
            let mut done = false;
            while !done {
                let mut record = FitRecord::new();

                match record.read(reader) {
                    Ok(i) => self.records.push(record),
                    Err(e) => done = true,
                }
            }

            // Read the CRC.
        }

        Ok(())
    }
}

pub fn read<R: Read>(reader: &mut BufReader<R>) -> Fit {
    let mut fit: Fit = Fit::new();
    fit.read(reader);
    fit
}
