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

type FieldDefinitionMap = Vec<FieldDefinition>;

fn read_n<R: Read>(reader: &mut BufReader<R>, bytes_to_read: u64) -> Result< Vec<u8> >
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let _n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    Ok(buf)
}

#[derive(Debug, Default)]
pub struct FitHeader {
    pub header: Vec<u8>,
    pub header_buf2: [u8; 2]   // Additional information introduced with the 14 byte header
}

impl FitHeader {
    pub fn new() -> Self {
        let header = FitHeader{ header: Vec::new(), header_buf2: [0u8; 2] };
        header
    }

    /// Reads the FIT File Header from the buffer.
    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<()> {

        // Reads first 12 bytes of the header (12 bytes is the minimum header size for a valid FIT file).
        self.header = read_n(reader, 12)?;

        // Does this file use the newer, 14 byte header. 
        if self.header[HEADER_FILE_SIZE_OFFSET] == 14 {
            let mut additional_bytes = read_n(reader, 2)?;
            self.header.append(&mut additional_bytes);
        }
        Ok(())
    }

    /// Validates the FIT File Header. Call after calling read().
    pub fn validate(&self) -> bool {
        let mut valid  = self.header[HEADER_DATA_TYPE_0_OFFSET] == '.' as u8;
        valid = valid && self.header[HEADER_DATA_TYPE_1_OFFSET] == 'F' as u8;
        valid = valid && self.header[HEADER_DATA_TYPE_2_OFFSET] == 'I' as u8;
        valid = valid && self.header[HEADER_DATA_TYPE_3_OFFSET] == 'T' as u8;
        valid
    }

    /// Prints the raw bytes comprising the FIT File Header.
    pub fn print(&self) {
        for byte in self.header.iter() {
            print!("{:#04x} ", byte);
        }
    }

    /// Calculates and returns the data size from the FIT File Header.
    pub fn data_size(&self) -> u32 {
        let mut data_size = self.header[HEADER_DATA_SIZE_LSB_OFFSET] as u32;
        data_size = data_size | (self.header[HEADER_DATA_SIZE_1_OFFSET] as u32) << 8;
        data_size = data_size | (self.header[HEADER_DATA_SIZE_2_OFFSET] as u32) << 16;
        data_size = data_size | (self.header[HEADER_DATA_SIZE_MSB_OFFSET] as u32) << 24;
        data_size
    }
}

/// Encapsulates a custom field definition, as described by definition messages and used by data messages.
struct FieldDefinition {
    field_def: u8,
    size: u8,
    base_type: u8
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

    /// Assumes the buffer is pointing to the beginning of the definition message, reads the message, and updates the field definitions.
    fn read_definition_message<R: Read>(&mut self, reader: &mut BufReader<R>, definitions: &mut FieldDefinitionMap) -> Result<()> {

        // Definition message (5 bytes).
        // 0: Reserved
        // 1: Architecture
        // 2-3: Global Message Number
        // 4: Number of Fields
        let mut definition_header: [u8; 5] = [0; 5];
        reader.read_exact(&mut definition_header)?;

        // Read each field.
        for _i in 0..definition_header[4] {

            // Read the field definition (3 bytes).
            let mut field_def_bytes: [u8; 3] = [0; 3];
            reader.read_exact(&mut field_def_bytes)?;

            // Add the definition to the hash map.
            let field_def = FieldDefinition { field_def:field_def_bytes[0], size:field_def_bytes[1], base_type:field_def_bytes[2] };
            definitions.push(field_def);
        }

        // Read the number of developer fields (1 byte).
        let mut num_dev_fields: [u8; 1] = [0; 1];
        reader.read_exact(&mut num_dev_fields)?;

        // Read each developer field.
        for _i in 0..num_dev_fields[0] {

            // Field definition (3 bytes).
            let mut field_def_bytes: [u8; 3] = [0; 3];
            reader.read_exact(&mut field_def_bytes)?;
        }

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the data message, reads the message.
    fn read_data_message<R: Read>(&mut self, reader: &mut BufReader<R>, definitions: &mut FieldDefinitionMap) -> Result<()> {

        // Local message type.
        let local_msg_type = self.header_byte[0] & RECORD_HDR_LOCAL_MSG_TYPE;

        // Read data for each message definition.
        for def in definitions {
            let data = read_n(reader, def.size as u64);
        }

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the compressed timestamp message, reads the message.
    fn read_compressed_timestamp_message<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<()> {

        // Compressed Timestamp Header.
        let time_offset = self.header_byte[0] & 0x0f;
        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the normal message, reads the message.
    fn read_normal_message<R: Read>(&mut self, reader: &mut BufReader<R>, definitions: &mut FieldDefinitionMap) -> Result<()> {

        // Data or definition message?
        // A value of zero indicates a data message.
        if self.header_byte[0] & RECORD_HDR_MSG_TYPE != 0 {
            self.read_definition_message(reader, definitions)?;
        }
        else {
            self.read_data_message(reader, definitions)?;
        }

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the next record message, reads the message.
    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>, definitions: &mut FieldDefinitionMap) -> Result<()> {

        // The first byte is a bit field that tells us more about the record.
        reader.read_exact(&mut self.header_byte)?;

        // Normal header or compressed timestamp header?
        // A value of zero indicates a normal header.
        if self.header_byte[0] & RECORD_HDR_NORMAL != 0 {
            self.read_compressed_timestamp_message(reader)?;
        }
        else {
            self.read_normal_message(reader, definitions)?;
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

    fn check_crc(&self) {
    }

    /// Reads the FIT data from the buffer.
    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<()> {

        // Read the file header.
        self.header.read(reader)?;

        // Make sure the header is valid.
        if self.header.validate() {
            let mut done = false;
            let mut definitions = FieldDefinitionMap::new();

            // Read each record.
            while !done {
                let mut record = FitRecord::new();

                match record.read(reader, &mut definitions) {
                    Ok(_i) => self.records.push(record),
                    Err(_e) => done = true,
                }
            }

            // Read the CRC.
            self.check_crc();
        }

        Ok(())
    }
}

pub fn read<R: Read>(reader: &mut BufReader<R>) -> Result<Fit> {
    let mut fit: Fit = Fit::new();
    fit.read(reader)?;
    Ok(fit)
}
