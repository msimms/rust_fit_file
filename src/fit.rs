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
 #![allow(dead_code)]

use std::io::Result;
use std::io::Read;
use std::io::BufReader;
use std::cmp::Ordering;
use std::collections::HashMap;

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

const DEF_MSG_RESERVED: usize = 0;
const DEF_MSG_ARCHITECTURE: usize = 1; // 1 = Definition and Data Message are Big Endian
const DEF_MSG_GLOBAL_MSG_NUM: usize = 2;
const DEF_MSG_NUM_FIELDS: usize = 4;

const RECORD_HDR_NORMAL: u8 = 0x80;
const RECORD_HDR_MSG_TYPE: u8 = 0x40;
const RECORD_HDR_MSG_TYPE_SPECIFIC: u8 = 0x20;
const RECORD_HDR_RESERVED: u8 = 0x10;
const RECORD_HDR_LOCAL_MSG_TYPE: u8 = 0x0f;

pub const GLOBAL_MSG_NUM_FILE_ID: u16 = 0;
pub const GLOBAL_MSG_NUM_CAPABILITIES: u16 = 1;
pub const GLOBAL_MSG_NUM_DEVICE_SETTINGS: u16 = 2;
pub const GLOBAL_MSG_NUM_USER_PROFILE: u16 = 3;
pub const GLOBAL_MSG_NUM_HRM_PROFILE: u16 = 4;
pub const GLOBAL_MSG_NUM_SDM_PROFILE: u16 = 5;
pub const GLOBAL_MSG_NUM_BIKE_PROFILE: u16 = 6;
pub const GLOBAL_MSG_NUM_ZONES_TARGET: u16 = 7;
pub const GLOBAL_MSG_NUM_HR_ZONE: u16 = 8;
pub const GLOBAL_MSG_NUM_POWER_ZONE: u16 = 9;
pub const GLOBAL_MSG_NUM_MET_ZONE: u16 = 10;
pub const GLOBAL_MSG_NUM_SPORT: u16 = 12;
pub const GLOBAL_MSG_NUM_GOAL: u16 = 15;
pub const GLOBAL_MSG_NUM_SESSION: u16 = 18;
pub const GLOBAL_MSG_NUM_LAP: u16 = 19;
pub const GLOBAL_MSG_NUM_RECORD: u16 = 20;
pub const GLOBAL_MSG_NUM_EVENT: u16 = 21;
pub const GLOBAL_MSG_NUM_DEVICE_INFO: u16 = 23;
pub const GLOBAL_MSG_NUM_WORKOUT: u16 = 26;
pub const GLOBAL_MSG_NUM_WORKOUT_STEP: u16 = 27;
pub const GLOBAL_MSG_NUM_SCHEDULE: u16 = 28;
pub const GLOBAL_MSG_NUM_WEIGHT_SCALE: u16 = 30;

type FieldDefinitionMap = Vec<FieldDefinition>;
type Callback = fn(global_message_num: u16, local_message_type: u8, data: Vec<u64>);

pub fn init_global_msg_name_map() -> HashMap<u16, String> {
    let mut global_msg_name_map = HashMap::<u16, String>::new();

    global_msg_name_map.insert(GLOBAL_MSG_NUM_FILE_ID, "File ID".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_CAPABILITIES, "Capabilities".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_DEVICE_SETTINGS, "Device Settings".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_USER_PROFILE, "User Profile".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_HRM_PROFILE, "HRM Profile".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SDM_PROFILE, "SDM Profile".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_BIKE_PROFILE, "Bike Profile".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_ZONES_TARGET, "Zones Target".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_HR_ZONE, "HR Zone".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_POWER_ZONE, "Power Zone".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_MET_ZONE, "MET Zone".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SPORT, "Sport".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_GOAL, "Goal".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SESSION, "Session".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_LAP, "Lap".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_RECORD, "Record".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_EVENT, "Event".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_DEVICE_INFO, "Device Info".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_WORKOUT, "Workout".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_WORKOUT_STEP, "Workout Step".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SCHEDULE, "Schedule".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_WEIGHT_SCALE, "Weight Scale".to_string());
    global_msg_name_map
}

fn read_n<R: Read>(reader: &mut BufReader<R>, bytes_to_read: u64) -> Result< Vec<u8> >
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let _n = chunk.read_to_end(&mut buf).expect("Didn't read enough");

    Ok(buf)
}

fn byte_array_to_num(bytes: Vec<u8>, num_bytes: usize, big_endian: bool) -> u64 {
    if num_bytes == 1 {
        return bytes[0] as u64;
    }

    let mut num = 0;
    let mut offset = 0;

    if big_endian {
        for i in 0..num_bytes {
            num = num | (bytes[i] as u64) << offset;
            offset = offset + 8;
        }
    }
    else {
        for i in 0..num_bytes {
            num = (num << offset) | (bytes[num_bytes - i - 1] as u64);
            offset = offset + 8;
        }
    }
    num
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
#[derive(Debug, Default)]
struct FieldDefinition {
    field_def: u8,
    size: u8,
    base_type: u8
}

impl Ord for FieldDefinition {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.field_def, &self.size, &self.base_type).cmp(&(other.field_def, &other.size, &other.base_type))
    }
}

impl PartialOrd for FieldDefinition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FieldDefinition {
    fn eq(&self, other: &Self) -> bool {
        (self.field_def, &self.size) == (other.field_def, &other.size)
    }
}

impl Eq for FieldDefinition { }

#[derive(Debug, Default)]
struct State {
    current_architecture: bool, // 1 = big endian
    current_global_msg_num: u16,
    local_msg_defs: HashMap<u8, FieldDefinitionMap> // Describes the format of local message types
}

impl State {
    pub fn new() -> Self {
        let state = State{ current_architecture: false, current_global_msg_num:0, local_msg_defs: HashMap::<u8, FieldDefinitionMap>::new() };
        state
    }

    pub fn print(&self) {
        println!("Architecture Is Big Endian: {}", self.current_architecture);
        println!("Current Global Msg Num: {}", self.current_global_msg_num);
        for (local_msg_type, field_definitions) in &self.local_msg_defs {
            println!("Local Message Type {}", local_msg_type);
            for field_definition in field_definitions {
                println!("Field Num {}: Size {} Base Type {:#x}", field_definition.field_def, field_definition.size, field_definition.base_type);
            }
        }
    }
}

#[derive(Debug, Default)]
struct FitRecord {
}

impl FitRecord {
    pub fn new() -> Self {
        let rec = FitRecord{ };
        rec
    }

    /// Assumes the buffer is pointing to the beginning of the definition message, reads the message, and updates the field definitions.
    fn read_definition_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut State) -> Result<()> {

        // Local message type.
        let local_msg_type = header_byte & RECORD_HDR_LOCAL_MSG_TYPE;

        // Definition message (5 bytes).
        // 0: Reserved
        // 1: Architecture
        // 2-3: Global Message Number
        // 4: Number of Fields
        let mut definition_header: [u8; 5] = [0; 5];
        reader.read_exact(&mut definition_header)?;

        // Make a note of the Architecture and Global Message Number.
        state.current_architecture = definition_header[DEF_MSG_ARCHITECTURE] == 1;
        state.current_global_msg_num = byte_array_to_num(definition_header[DEF_MSG_GLOBAL_MSG_NUM..(DEF_MSG_GLOBAL_MSG_NUM + 2)].to_vec(), 2, state.current_architecture) as u16;

        // Read each field.
        let mut msg_defs: FieldDefinitionMap = FieldDefinitionMap::new();
        let num_fields = definition_header[DEF_MSG_NUM_FIELDS];
        //println!("def msg: local msg type: {} current_global_msg_num: {} num_fields: {}", local_msg_type, state.current_global_msg_num, num_fields);
        for _i in 0..num_fields {

            // Read the field definition (3 bytes).
            let mut field_def_bytes: [u8; 3] = [0; 3];
            reader.read_exact(&mut field_def_bytes)?;

            // Add the definition to the hash map.
            let field_def = FieldDefinition { field_def:field_def_bytes[0], size:field_def_bytes[1], base_type:field_def_bytes[2] };

            // Insert sorted.
            match msg_defs.binary_search(&field_def) {
                Ok(_pos) => {} // element already in vector @ `pos` 
                Err(pos) => msg_defs.insert(pos, field_def),
            }
        }

        // Is there any developer information in this record?
        if header_byte & RECORD_HDR_MSG_TYPE_SPECIFIC != 0 {

            // Read the number of developer fields (1 byte).
            let mut num_dev_fields: [u8; 1] = [0; 1];
            reader.read_exact(&mut num_dev_fields)?;

            // Read each developer field.
            for _i in 0..num_dev_fields[0] {

                // Field definition (3 bytes).
                let mut field_def_bytes: [u8; 3] = [0; 3];
                reader.read_exact(&mut field_def_bytes)?;
            }
        }

        // Associate the field definitions with the local message type.
        state.local_msg_defs.insert(local_msg_type, msg_defs);

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the data message, reads the message.
    fn read_data_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut State, callback: Callback) -> Result<()> {

        // Local message type.
        let local_msg_type = header_byte & RECORD_HDR_LOCAL_MSG_TYPE;

        // Retrieve the field definitions based on the message type.
        let msg_defs = state.local_msg_defs.get(&local_msg_type).unwrap();

        // Read data for each message definition.
        let mut records = Vec::new();
        for def in msg_defs.iter() {
            let data = read_n(reader, def.size as u64)?;

            match def.base_type {
                0x00 => { let num = byte_array_to_num(data, 1, state.current_architecture); records.push(num); },
                0x01 => { let num = byte_array_to_num(data, 1, state.current_architecture); records.push(num & 0x7f); },
                0x02 => { let num = byte_array_to_num(data, 1, state.current_architecture); records.push(num); },
                0x83 => { let num = byte_array_to_num(data, 2, state.current_architecture); records.push(num & 0x7FFF); },
                0x84 => { let num = byte_array_to_num(data, 2, state.current_architecture); records.push(num); },
                0x85 => { let num = byte_array_to_num(data, 4, state.current_architecture); records.push(num & 0x7FFFFFFF); },
                0x86 => { let num = byte_array_to_num(data, 4, state.current_architecture); records.push(num); },
                0x07 => { panic!("base type not implemented {:#x}", def.base_type); },
                0x88 => { panic!("base type not implemented {:#x}", def.base_type); },
                0x89 => { panic!("base type not implemented {:#x}", def.base_type); },
                0x0A => { let num = byte_array_to_num(data, 1, state.current_architecture); records.push(num); },
                0x8B => { let num = byte_array_to_num(data, 2, state.current_architecture); records.push(num); },
                0x8C => { let num = byte_array_to_num(data, 4, state.current_architecture); records.push(num); },
                0x0D => { for i in 0..def.size {
                        records.push(data[i as usize] as u64); 
                    }
                 },
                0x8E => { panic!("base type not implemented {:#x}", def.base_type); },
                0x8F => { let num = byte_array_to_num(data, 8, state.current_architecture); records.push(num); },
                0x90 => { panic!("base type not implemented {:#x}", def.base_type); },
                _ => { panic!("base type not implemented {:#x}", def.base_type); }
            }
        }
        callback(state.current_global_msg_num, local_msg_type, records);

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the compressed timestamp message, reads the message.
    fn read_compressed_timestamp_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut State, callback: Callback) -> Result<()> {

        // Compressed Timestamp Header.
        let time_offset = header_byte & 0x0f;
        panic!("not implemented");

        // Read the data fields that follow.
        self.read_data_message(reader, header_byte, state, callback)?;

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the normal message, reads the message.
    fn read_normal_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut State, callback: Callback) -> Result<()> {

        // Reserve bit should be zero in normal messages.
        if header_byte & RECORD_HDR_RESERVED != 0 {
            panic!("reserve bit set");
        }

        // Data or definition message?
        // A value of zero indicates a data message.
        if header_byte & RECORD_HDR_MSG_TYPE != 0 {
            self.read_definition_message(reader, header_byte, state)?;
        }
        else {
            self.read_data_message(reader, header_byte, state, callback)?;
        }

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the next record message, reads the message.
    fn read<R: Read>(&mut self, reader: &mut BufReader<R>, state: &mut State, callback: Callback) -> Result<()> {

        // The first byte is a bit field that tells us more about the record.
        let mut header_byte: [u8; 1] = [0; 1];
        reader.read_exact(&mut header_byte)?;

        // Normal header or compressed timestamp header?
        // A value of zero indicates a normal header.
        if header_byte[0] & RECORD_HDR_NORMAL != 0 {
            self.read_compressed_timestamp_message(reader, header_byte[0], state, callback)?;
        }
        else {
            self.read_normal_message(reader, header_byte[0], state, callback)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Fit {
    pub header: FitHeader
}

impl Fit {
    pub fn new() -> Self {
        let fit = Fit{ header: FitHeader::new() };
        fit
    }

    fn check_crc(&self) {
    }

    /// Reads the FIT data from the buffer.
    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>, callback: Callback) -> Result<()> {

        // Read the file header.
        self.header.read(reader)?;

        // Make sure the header is valid.
        if self.header.validate() {
            let mut done = false;
            let mut state = State::new();

            // Read each record.
            while !done {
                let mut record = FitRecord::new();
                record.read(reader, &mut state, callback)?;
                done = reader.buffer().is_empty();
            }

            // Read the CRC.
            self.check_crc();
        }

        Ok(())
    }
}

pub fn read<R: Read>(reader: &mut BufReader<R>, callback: Callback) -> Result<Fit> {
    let mut fit: Fit = Fit::new();
    fit.read(reader, callback)?;

    Ok(fit)
}
