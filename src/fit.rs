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
use std::io::{Error};

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

// Reserved field numbers.
const FIELD_MSG_INDEX: u8 = 254;
const FIELD_TIMESTAMP: u8 = 253;
const FIELD_PART_INDEX: u8 = 250;

const RECORD_HDR_NORMAL: u8 = 0x80;
const RECORD_HDR_MSG_TYPE: u8 = 0x40;
const RECORD_HDR_MSG_TYPE_SPECIFIC: u8 = 0x20;
const RECORD_HDR_RESERVED: u8 = 0x10;
const RECORD_HDR_LOCAL_MSG_TYPE: u8 = 0x0f;
const RECORD_HDR_LOCAL_MSG_TYPE_COMPRESSED: u8 = 0x60;

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
pub const GLOBAL_MSG_NUM_COURSE: u16 = 31;
pub const GLOBAL_MSG_NUM_COURSE_POINT: u16 = 32;
pub const GLOBAL_MSG_NUM_TOTALS: u16 = 33;
pub const GLOBAL_MSG_NUM_ACTIVITY: u16 = 34;
pub const GLOBAL_MSG_NUM_SOFTWARE: u16 = 35;
pub const GLOBAL_MSG_NUM_FILE_CAPABILITIES: u16 = 37;
pub const GLOBAL_MSG_NUM_MESG_CAPABILITIES: u16 = 38;
pub const GLOBAL_MSG_NUM_FIELD_CAPABILITIES: u16 = 39;
pub const GLOBAL_MSG_NUM_FILE_CREATOR: u16 = 49;
pub const GLOBAL_MSG_NUM_BLOOD_PRESSURE: u16 = 51;
pub const GLOBAL_MSG_NUM_SPEED_ZONE: u16 = 53;
pub const GLOBAL_MSG_NUM_MONITORING: u16 = 55;
pub const GLOBAL_MSG_NUM_TRAINING_FILE: u16 = 72;
pub const GLOBAL_MSG_NUM_HRV: u16 = 78;
pub const GLOBAL_MSG_NUM_ANT_RX: u16 = 80;
pub const GLOBAL_MSG_NUM_ANT_TX: u16 = 81;
pub const GLOBAL_MSG_NUM_ANT_CHANNEL_ID: u16 = 82;
pub const GLOBAL_MSG_NUM_LENGTH: u16 = 101;
pub const GLOBAL_MSG_NUM_MONITORING_INFO: u16 = 103;
pub const GLOBAL_MSG_NUM_PAD: u16 = 105;
pub const GLOBAL_MSG_NUM_SLAVE_DEVICE: u16 = 106;
pub const GLOBAL_MSG_NUM_CONNECTIVITY: u16 = 127;
pub const GLOBAL_MSG_NUM_WEATHER_CONDITIONS: u16 = 128;
pub const GLOBAL_MSG_NUM_WEATHER_ALERT: u16 = 129;
pub const GLOBAL_MSG_NUM_CADENCE_ZONE: u16 = 131;
pub const GLOBAL_MSG_NUM_HR: u16 = 132;
pub const GLOBAL_MSG_NUM_SEGMENT_LAP: u16 = 142;
pub const GLOBAL_MSG_NUM_MEMO_GLOB: u16 = 145;
pub const GLOBAL_MSG_NUM_SEGMENT_ID: u16 = 148;
pub const GLOBAL_MSG_NUM_SEGMENT_LEADERBOARD_ENTRY: u16 = 149;
pub const GLOBAL_MSG_NUM_SEGMENT_POINT: u16 = 150;
pub const GLOBAL_MSG_NUM_SEGMENT_FILE: u16 = 151;
pub const GLOBAL_MSG_NUM_WORKOUT_SESSION: u16 = 158;
pub const GLOBAL_MSG_NUM_WATCHFACE_SETTINGS: u16 = 159;
pub const GLOBAL_MSG_NUM_GPS_METADATA: u16 = 160;
pub const GLOBAL_MSG_NUM_CAMERA_EVENT: u16 = 161;
pub const GLOBAL_MSG_NUM_TIMESTAMP_CORRELATION: u16 = 162;
pub const GLOBAL_MSG_NUM_GYROSCOPE_DATA: u16 = 164;
pub const GLOBAL_MSG_NUM_ACCELEROMETER_DATA: u16 = 165;
pub const GLOBAL_MSG_NUM_THREE_D_SENSOR_CALIBRATION: u16 = 167;
pub const GLOBAL_MSG_NUM_VIDEO_FRAME: u16 = 169;
pub const GLOBAL_MSG_NUM_OBDII_DATA: u16 = 174;
pub const GLOBAL_MSG_NUM_NMEA_SENTENCE: u16 = 177;
pub const GLOBAL_MSG_NUM_AVIATION_ATTITUDE: u16 = 178;
pub const GLOBAL_MSG_NUM_VIDEO: u16 = 184;
pub const GLOBAL_MSG_NUM_VIDEO_TITLE: u16 = 185;
pub const GLOBAL_MSG_NUM_VIDEO_DESCRIPTION: u16 = 186;
pub const GLOBAL_MSG_NUM_VIDEO_CLIP: u16 = 187;
pub const GLOBAL_MSG_NUM_OHR_SETTINGS: u16 = 188;
pub const GLOBAL_MSG_NUM_EXD_SCREEN_CONFIGURATION: u16 = 200;
pub const GLOBAL_MSG_NUM_EXD_DATA_FIELD_CONFIGURATION: u16 = 201;
pub const GLOBAL_MSG_NUM_EXD_DATA_CONCEPT_CONFIGURATION: u16 = 202;
pub const GLOBAL_MSG_NUM_FIELD_DESCRIPTION: u16 = 206;
pub const GLOBAL_MSG_NUM_DEVELOPER_DATA_ID: u16 = 207;
pub const GLOBAL_MSG_NUM_MAGNETOMETER_DATA: u16 = 208;
pub const GLOBAL_MSG_NUM_BAROMETER_DATA: u16 = 209;
pub const GLOBAL_MSG_NUM_ONE_D_SENSOR_CALIBRATION: u16 = 210;
pub const GLOBAL_MSG_NUM_SET: u16 = 225;
pub const GLOBAL_MSG_NUM_STRESS_LEVEL: u16 = 227;
pub const GLOBAL_MSG_NUM_DIVE_SETTINGS: u16 = 258;
pub const GLOBAL_MSG_NUM_DIVE_GAS: u16 = 259;
pub const GLOBAL_MSG_NUM_DIVE_ALARM: u16 = 262;
pub const GLOBAL_MSG_NUM_EXERCISE_TITLE: u16 = 264;
pub const GLOBAL_MSG_NUM_DIVE_SUMMARY: u16 = 268;
pub const GLOBAL_MSG_NUM_JUMP: u16 = 285;
pub const GLOBAL_MSG_NUM_CLIMB_PRO: u16 = 317;

pub const FIT_SPORT_GENERIC: u8 = 0;
pub const FIT_SPORT_RUNNING: u8 = 1;
pub const FIT_SPORT_CYCLING: u8 = 2;
pub const FIT_SPORT_TRANSITION: u8 = 3; // Mulitsport transition
pub const FIT_SPORT_FITNESS_EQUIPMENT: u8 = 4;
pub const FIT_SPORT_SWIMMING: u8 = 5;
pub const FIT_SPORT_BASKETBALL: u8 = 6;
pub const FIT_SPORT_SOCCER: u8 = 7;
pub const FIT_SPORT_TENNIS: u8 = 8;
pub const FIT_SPORT_AMERICAN_FOOTBALL: u8 = 9;
pub const FIT_SPORT_TRAINING: u8 = 10;
pub const FIT_SPORT_WALKING: u8 = 11;
pub const FIT_SPORT_CROSS_COUNTRY_SKIING: u8 = 12;
pub const FIT_SPORT_ALPINE_SKIING: u8 = 13;
pub const FIT_SPORT_SNOWBOARDING: u8 = 14;
pub const FIT_SPORT_ROWING: u8 = 15;
pub const FIT_SPORT_MOUNTAINEERING: u8 = 16;
pub const FIT_SPORT_HIKING: u8 = 17;
pub const FIT_SPORT_MULTISPORT: u8 = 18;
pub const FIT_SPORT_PADDLING: u8 = 19;
pub const FIT_SPORT_FLYING: u8 = 20;
pub const FIT_SPORT_E_BIKING: u8 = 21;
pub const FIT_SPORT_MOTORCYCLING: u8 = 22;
pub const FIT_SPORT_BOATING: u8 = 23;
pub const FIT_SPORT_DRIVING: u8 = 24;
pub const FIT_SPORT_GOLF: u8 = 25;
pub const FIT_SPORT_HANG_GLIDING: u8 = 26;
pub const FIT_SPORT_HORSEBACK_RIDING: u8 = 27;
pub const FIT_SPORT_HUNTING: u8 = 28;
pub const FIT_SPORT_FISHING: u8 = 29;
pub const FIT_SPORT_INLINE_SKATING: u8 = 30;
pub const FIT_SPORT_ROCK_CLIMBING: u8 = 31;
pub const FIT_SPORT_SAILING: u8 = 32;
pub const FIT_SPORT_ICE_SKATING: u8 = 33;
pub const FIT_SPORT_SKY_DIVING: u8 = 34;
pub const FIT_SPORT_SNOWSHOEING: u8 = 35;
pub const FIT_SPORT_SNOWMOBILING: u8 = 36;
pub const FIT_SPORT_STAND_UP_PADDLEBOARDING: u8 = 37;
pub const FIT_SPORT_SURFING: u8 = 38;
pub const FIT_SPORT_WAKEBOARDING: u8 = 39;
pub const FIT_SPORT_WATER_SKIING: u8 = 40;
pub const FIT_SPORT_KAYAKING: u8 = 41;
pub const FIT_SPORT_RAFTING: u8 = 42;
pub const FIT_SPORT_WINDSURFING: u8 = 43;
pub const FIT_SPORT_KITESURFING: u8 = 44;
pub const FIT_SPORT_TACTICAL: u8 = 45;
pub const FIT_SPORT_JUMPMASTER: u8 = 46;
pub const FIT_SPORT_BOXING: u8 = 47;
pub const FIT_SPORT_FLOOR_CLIMBING: u8 = 48;
pub const FIT_SPORT_DIVING: u8 = 53;
pub const FIT_SPORT_ALL: u8 = 254;

type Callback = fn(timestamp: u32, global_message_num: u16, local_message_type: u8, data: Vec<FieldValue>);

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
    global_msg_name_map.insert(GLOBAL_MSG_NUM_COURSE, "Course".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_COURSE_POINT, "Course Point".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_TOTALS, "Totals".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_ACTIVITY, "Activity".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SOFTWARE, "Software".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_FILE_CAPABILITIES, "File Capabilities".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_MESG_CAPABILITIES, "Message Capabilities".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_FIELD_CAPABILITIES, "Field Capabilities".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_FILE_CREATOR, "File Creator".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_BLOOD_PRESSURE, "Blood Pressure".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SPEED_ZONE, "Speed Zone".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_MONITORING, "Monitoring".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_TRAINING_FILE, "Training File".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_HRV, "HRV".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_ANT_RX, "ANT RX".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_ANT_TX, "ANT TX".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_ANT_CHANNEL_ID, "ANT Channel ID".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_LENGTH, "Length".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_MONITORING_INFO, "Monitoring Info".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_PAD, "Pad".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SLAVE_DEVICE, "Slave Device".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_CONNECTIVITY, "Connectivity".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_WEATHER_CONDITIONS, "Weather".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_WEATHER_ALERT, "Weather Alert".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_CADENCE_ZONE, "Cadence Zone".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_HR, "HR".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SEGMENT_LAP, "Segment Lap".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_MEMO_GLOB, "Memo Glob".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SEGMENT_ID, "Segment ID".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SEGMENT_LEADERBOARD_ENTRY, "Segment Leaderboard Entry".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SEGMENT_POINT, "Segment Point".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SEGMENT_FILE, "Segment File".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_WORKOUT_SESSION, "Workout Session".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_WATCHFACE_SETTINGS, "Watch Face Settings".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_GPS_METADATA, "GPS".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_CAMERA_EVENT, "Camera Event".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_TIMESTAMP_CORRELATION, "Timestamp Correlation".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_GYROSCOPE_DATA, "Cyroscope Data".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_ACCELEROMETER_DATA, "Accelerometer Data".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_THREE_D_SENSOR_CALIBRATION, "3D Sensor Calibration".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_VIDEO_FRAME, "Video Frame".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_OBDII_DATA, "OBDII Data".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_NMEA_SENTENCE, "NMEA Sentence".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_AVIATION_ATTITUDE, "Aviation Attitude".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_VIDEO, "Video".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_VIDEO_TITLE, "Video Title".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_VIDEO_DESCRIPTION, "Video Description".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_VIDEO_CLIP, "Video Clip".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_OHR_SETTINGS, "OHR Settings".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_EXD_SCREEN_CONFIGURATION, "EXD Screen Configuration".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_EXD_DATA_FIELD_CONFIGURATION, "EXD Data Field Configuration".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_EXD_DATA_CONCEPT_CONFIGURATION, "EXD Data Concept Configuration".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_FIELD_DESCRIPTION, "Field Description".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_DEVELOPER_DATA_ID, "Developer Data ID".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_MAGNETOMETER_DATA, "Magnetometer Data".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_BAROMETER_DATA, "Barometer Data".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_ONE_D_SENSOR_CALIBRATION, "1D Sensor Calibration".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_SET, "Set".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_STRESS_LEVEL, "Stress Level".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_DIVE_SETTINGS, "Dive Settings".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_DIVE_GAS, "Dive Gas".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_DIVE_ALARM, "Dive Alarm".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_EXERCISE_TITLE, "Exercise Title".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_DIVE_SUMMARY, "Dive Summary".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_JUMP, "Jump".to_string());
    global_msg_name_map.insert(GLOBAL_MSG_NUM_CLIMB_PRO, "Climb Pro".to_string());
    global_msg_name_map
}

fn read_n<R: Read>(reader: &mut BufReader<R>, bytes_to_read: u64) -> Result< Vec<u8> >
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let _n = chunk.read_to_end(&mut buf).expect("Didn't read enough");

    Ok(buf)
}

fn read_u32<R: Read>(reader: &mut BufReader<R>, big_endian: bool) -> Result<u32>
{
    let bytes = read_n(reader, 4)?;
    let num = byte_array_to_int(bytes, 4, big_endian) as u32;

    Ok(num)
}

fn read_byte<R: Read>(reader: &mut BufReader<R>) -> Result<u8>
{
    let mut byte: [u8; 1] = [0; 1];
    reader.read_exact(&mut byte)?;

    Ok(byte[0])
}

fn read_string<R: Read>(reader: &mut BufReader<R>) -> Result<String>
{
    let mut result = String::new();
    let mut done = false;

    while !done {
        let buf = read_n(reader, 1)?;

        if buf[0] == 0 {
            done = true;
        }
        else {
            result.push(buf[0] as char);
        }
    }

    Ok(result)
}

fn byte_array_to_string(bytes: Vec<u8>, num_bytes: usize) -> String {
    let mut result = String::new();

    for i in 0..num_bytes {
        result.push(bytes[i] as char);
    }
    result
}

fn byte_array_to_int(bytes: Vec<u8>, num_bytes: usize, big_endian: bool) -> u64 {
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

fn byte_array_to_float(bytes: Vec<u8>, num_bytes: usize, _big_endian: bool) -> f64 {
    if num_bytes == 1 {
        return bytes[0] as f64;
    }
    else if num_bytes == 4 {
        let bytes = [0, 0 , 0, 0];
        return f32::from_bits(u32::from_be_bytes(bytes)) as f64;
    }
    else if num_bytes == 8 {
        let bytes = [0, 0, 0, 0, 0, 0, 0, 0];
        return f64::from_bits(u64::from_be_bytes(bytes)) as f64;
    }

    0.0
}

fn print_byte_array(bytes: Vec<u8>) {
    for byte in bytes {
        print!("{:#04x} ", byte);
    }
}

pub enum FieldType {
    FieldTypeNotSet,
    FieldTypeInt,
    FieldTypeFloat,
    FieldTypeByteArray,
    FieldTypeStr
}

pub struct FieldValue {
    pub field_type: FieldType,
    pub num_int: u64,
    pub num_float: f64,
    pub byte_array: Vec<u8>,
    pub string: String
}

impl FieldValue {
    pub fn new() -> Self {
        let state = FieldValue{ field_type: FieldType::FieldTypeNotSet, num_int: 0, num_float: 0.0, byte_array: Vec::<u8>::new(), string: String::new() };
        state
    }
}

/// Encapsulates a custom field definition, as described by definition messages and used by data messages.
#[derive(Copy, Clone, Debug, Default)]
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

type FieldDefinitionList = Vec<FieldDefinition>;

/// Describes a global message that was defined in the FIT file.
#[derive(Debug, Default)]
struct GlobalMessage {
    local_msg_defs: HashMap<u8, FieldDefinitionList>, // Describes the format of local messages, key is the local message type
}

impl GlobalMessage {
    pub fn new() -> Self {
        let msg = GlobalMessage{ local_msg_defs: HashMap::<u8, FieldDefinitionList>::new() };
        msg
    }

    /// For debugging purposes.
    fn print(&self) {
        for (local_msg_type, local_msg_def) in &self.local_msg_defs {
            println!("   Local Message Type {}:", local_msg_type);

            for field_definition in local_msg_def {
                println!("      Field Num {}: Size {} Base Type {:#x}", field_definition.field_def, field_definition.size, field_definition.base_type);
            }
        }
    }

    /// Creates an entry in the local message map for the a local message with the specified number.
    /// If the message definition already exists then replace it.
    fn insert_msg_def(&mut self, local_msg_type: u8, local_msg_def: FieldDefinitionList) {
        if self.local_msg_defs.contains_key(&local_msg_type) {
            self.local_msg_defs.remove(&local_msg_type);
        }
        self.local_msg_defs.insert(local_msg_type, local_msg_def);
    }

    /// Retrieves the local message with the specified number.
    fn retrieve_msg_def(&self, local_msg_type: u8) -> Option<&FieldDefinitionList> {
        self.local_msg_defs.get(&local_msg_type)
    }
}

/// Contains everything we need to remember about the state of the file parsing operation.
#[derive(Debug, Default)]
struct FitState {
    is_big_endian: bool, // 1 = big endian
    current_global_msg_num: u16, // Most recently defined global message number
    global_msg_map: HashMap<u16, GlobalMessage>, // Associates global messages with local message definitions, key is the global message number
    timestamp: u32, // For use with the compressed timestamp header
    num_records_read: usize // Total number of records read, for debugging purposes
}

impl FitState {
    pub fn new() -> Self {
        let state = FitState{ is_big_endian: false, current_global_msg_num: 0, global_msg_map: HashMap::<u16, GlobalMessage>::new(), timestamp: 0, num_records_read:0 };
        state
    }

    /// For debugging purposes.
    fn print(&self) {
        println!("----------------------------------------");
        println!("Architecture Is Big Endian: {}", self.is_big_endian);
        println!("Current Global Message Number: {}", self.current_global_msg_num);

        for (global_msg_num, global_msg_def) in &self.global_msg_map {
            println!("Global Message Number {}:", global_msg_num);
            global_msg_def.print();
        }
    }

    /// Creates an entry in the global message hash map for the specified message number.
    fn insert_global_msg(&mut self, global_msg_num: u16) {
        if !self.global_msg_map.contains_key(&global_msg_num) {
            self.global_msg_map.insert(global_msg_num, GlobalMessage::new());
        }
    }

    fn insert_local_msg_def(&mut self, global_msg_num: u16, local_msg_type: u8, local_msg_def: FieldDefinitionList) {
        self.global_msg_map.entry(global_msg_num)
            .and_modify(|e| { e.insert_msg_def(local_msg_type, local_msg_def) })
            .or_insert(GlobalMessage::new());
    }

    fn retrieve_msg_def(&self, global_msg_num: u16, local_msg_type: u8) -> Option<&FieldDefinitionList> {
        let global_msg_map = self.global_msg_map.get(&global_msg_num)?;
        global_msg_map.retrieve_msg_def(local_msg_type)
    }
}

/// Parses and validates the FIT file header.
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

/// Parses FIT file records.
#[derive(Debug, Default)]
struct FitRecord {
}

impl FitRecord {
    pub fn new() -> Self {
        let rec = FitRecord{ };
        rec
    }

    /// Assumes the buffer is pointing to the beginning of the definition message, reads the message, and updates the field definitions.
    fn read_definition_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut FitState) -> Result<()> {

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
        let global_msg_num = byte_array_to_int(definition_header[DEF_MSG_GLOBAL_MSG_NUM..(DEF_MSG_GLOBAL_MSG_NUM + 2)].to_vec(), 2, state.is_big_endian) as u16;
        state.current_global_msg_num = global_msg_num;
        state.is_big_endian = definition_header[DEF_MSG_ARCHITECTURE] == 1;

        // Make sure we have an entry in the hash map for this global message. This will do nothing if it already exists.
        state.insert_global_msg(global_msg_num);

        // Read each field.
        let mut msg_defs: FieldDefinitionList = FieldDefinitionList::new();
        let num_fields = definition_header[DEF_MSG_NUM_FIELDS];
        for _i in 0..num_fields {

            // Read the field definition (3 bytes).
            let field_num = read_byte(reader)?;
            let field_bytes = read_byte(reader)?;
            let field_type = read_byte(reader)?;

            // Add the definition.
            let field_def = FieldDefinition { field_def:field_num, size:field_bytes, base_type:field_type };
            msg_defs.push(field_def);
        }

        // Is there any developer information in this record?
        if header_byte & RECORD_HDR_MSG_TYPE_SPECIFIC != 0 {

            // Read the number of developer fields (1 byte).
            let num_dev_fields = read_byte(reader)?;

            // Read each developer field.
            for _i in 0..num_dev_fields {

                // Read the field definition (3 bytes).
                let field_num = read_byte(reader)?;
                let field_bytes = read_byte(reader)?;
                let field_type = read_byte(reader)?;

                // Add the definition.
                let field_def = FieldDefinition { field_def:field_num, size:field_bytes, base_type:field_type };
                msg_defs.push(field_def);
            }
        }

        // Associate the field definitions with the local message type.
        state.insert_local_msg_def(global_msg_num, local_msg_type, msg_defs);

        //println!("Message Definition: global message num: {} local message type: {} number of fields: {}", global_msg_num, local_msg_type, num_fields);
        //state.print();

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the data message, reads the message.
    fn read_data_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut FitState, callback: Callback) -> Result<()> {

        // Local message type. The local message type is stored differently for compressed data headers.
        let local_msg_type;
        if header_byte & RECORD_HDR_NORMAL != 0 {
            local_msg_type = (header_byte & RECORD_HDR_LOCAL_MSG_TYPE_COMPRESSED) >> 5;
        }
        else {
            local_msg_type = header_byte & RECORD_HDR_LOCAL_MSG_TYPE;
        }

        // Retrieve the field definitions based on the message type.
        let mut new_timestamp = state.timestamp;
        let msg_defs = state.retrieve_msg_def(state.current_global_msg_num, local_msg_type);
        match msg_defs {
            Some(msg_defs) => {

                // Read data for each message definition.
                let mut fields = Vec::new();
                for def in msg_defs.iter() {

                    let mut field = FieldValue::new();

                    // Read the number of bytes prescribed by the field definition.
                    let data = read_n(reader, def.size as u64)?;

                    // Is this a special field, like a timestamp?
                    if def.field_def == FIELD_MSG_INDEX {
                        panic!("Message Index not implemented: global message num: {} local message type: {}.", state.current_global_msg_num, local_msg_type);
                    }
                    else if def.field_def == FIELD_TIMESTAMP {
                        new_timestamp = byte_array_to_int(data, 4, state.is_big_endian) as u32;
                    }
                    else if def.field_def == FIELD_PART_INDEX {
                        panic!("Part Index not implemented: global message num: {} local message type: {}.", state.current_global_msg_num, local_msg_type);
                    }

                    // Normal field.
                    else {
                        match def.base_type {
                            0x00 => { field.num_int = byte_array_to_int(data, 1, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x01 => { field.num_int = byte_array_to_int(data, 1, state.is_big_endian) & 0x7f; field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x02 => { field.num_int = byte_array_to_int(data, 1, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x83 => { field.num_int = byte_array_to_int(data, 2, state.is_big_endian) & 0x7FFF; field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x84 => { field.num_int = byte_array_to_int(data, 2, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x85 => { field.num_int = byte_array_to_int(data, 4, state.is_big_endian) & 0x7FFFFFFF; field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x86 => { field.num_int = byte_array_to_int(data, 4, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x07 => { field.string = byte_array_to_string(data, def.size as usize); field.field_type = FieldType::FieldTypeStr; /*println!("{} {}", def.size, field.string);*/ fields.push(field); },
                            0x88 => { field.num_float = byte_array_to_float(data, 4, state.is_big_endian); field.field_type = FieldType::FieldTypeFloat; fields.push(field); },
                            0x89 => { field.num_float = byte_array_to_float(data, 8, state.is_big_endian); field.field_type = FieldType::FieldTypeFloat; fields.push(field); },
                            0x0A => { field.num_int = byte_array_to_int(data, 1, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x8B => { field.num_int = byte_array_to_int(data, 2, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x8C => { field.num_int = byte_array_to_int(data, 4, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x0D => { field.byte_array = data; field.field_type = FieldType::FieldTypeByteArray; fields.push(field); },
                            0x8E => { field.num_int = byte_array_to_int(data, 8, state.is_big_endian) & 0x7FFFFFFFFFFFFFFF; field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x8F => { field.num_int = byte_array_to_int(data, 8, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            0x90 => { field.num_int = byte_array_to_int(data, 8, state.is_big_endian); field.field_type = FieldType::FieldTypeInt; fields.push(field); },
                            _ => { panic!("Base Type not implemented {:#x}", def.base_type); }
                        }
                    }
                }
                state.num_records_read = state.num_records_read + 1;
                state.timestamp = new_timestamp;

                // Tell the people.
                callback(state.timestamp, state.current_global_msg_num, local_msg_type, fields);
            },
            None    => {
                let e = Error::new(std::io::ErrorKind::Other, "oh no!");
                return Err(e);
            },
        }

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the compressed timestamp message, reads the message.
    fn read_compressed_timestamp_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut FitState, callback: Callback) -> Result<()> {

        // Compressed Timestamp Header.
        let time_offset = (header_byte & 0x0f) as u32;
        if time_offset >= state.timestamp & 0x0000001F { // offset value is greater than least significant 5 bits of previous timestamp
            state.timestamp = (state.timestamp & 0xFFFFFFE0) + time_offset;
        }
        else {
            state.timestamp = (state.timestamp & 0xFFFFFFE0) + time_offset + 0x00000020;
        }

        // Read the data fields that follow.
        self.read_data_message(reader, header_byte, state, callback)?;

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the normal message, reads the message.
    fn read_normal_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut FitState, callback: Callback) -> Result<()> {

        // Reserve bit should be zero in normal messages.
        if header_byte & RECORD_HDR_RESERVED != 0 {
            panic!("Reserve bit set");
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
    fn read<R: Read>(&mut self, reader: &mut BufReader<R>, state: &mut FitState, callback: Callback) -> Result<()> {

        // The first byte is a bit field that tells us more about the record.
        let header_byte = read_byte(reader)?;
        //println!("Header Byte: {:#04x}", header_byte);

        // Normal header or compressed timestamp header?
        // A value of zero indicates a normal header.
        if header_byte & RECORD_HDR_NORMAL != 0 {
            self.read_compressed_timestamp_message(reader, header_byte, state, callback)?;
        }
        else {
            self.read_normal_message(reader, header_byte, state, callback)?;
        }

        Ok(())
    }
}

/// Parses a FIT file.
#[derive(Debug, Default)]
pub struct Fit {
    pub header: FitHeader
}

impl Fit {
    pub fn new() -> Self {
        let fit = Fit{ header: FitHeader::new() };
        fit
    }

    fn check_crc(&self, crc: u16, byte: u8) {
        let crc_table: [u16; 16] = [
            0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401,
            0xA001, 0x6C00, 0x7800, 0xB401, 0x5000, 0x9C01, 0x8801, 0x4400
        ];

        // Compute checksum of lower four bits of byte.
        let mut crc2 = crc;
        let mut tmp: u16 = crc_table[(crc2 & 0xf) as usize];
        crc2 = (crc2 >> 4) & 0x0fff;
        crc2 = crc2 ^ tmp ^ crc_table[(byte & 0xf) as usize];

        // Now compute checksum of upper four bits of byte.
        tmp = crc_table[(crc2 & 0xf) as usize];
        crc2 = (crc2 >> 4) & 0x0fff;
        crc2 = crc2 ^ tmp ^ crc_table[((byte >> 4) & 0xf) as usize];
    }

    /// Reads the FIT data from the buffer.
    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>, callback: Callback) -> Result<()> {

        // Read the file header.
        self.header.read(reader)?;

        // Make sure the header is valid.
        if self.header.validate() {

            let mut state = FitState::new();

            // Read each record.
            while !reader.buffer().is_empty() {
                let mut record = FitRecord::new();
                record.read(reader, &mut state, callback)?;
            }

            // Read the CRC.
            //self.check_crc();
        }

        Ok(())
    }
}

pub fn read<R: Read>(reader: &mut BufReader<R>, callback: Callback) -> Result<Fit> {
    let mut fit: Fit = Fit::new();
    fit.read(reader, callback)?;

    Ok(fit)
}
