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
use std::convert::TryInto;

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

pub const FIT_ENUM_INVALID: u8 = 0xff;
pub const FIT_STROKE_TYPE_INVALID: u8 = FIT_ENUM_INVALID;
pub const FIT_STROKE_TYPE_NO_EVENT: u8 = 0;
pub const FIT_STROKE_TYPE_OTHER: u8 = 1; // stroke was detected but cannot be identified
pub const FIT_STROKE_TYPE_SERVE: u8 = 2;
pub const FIT_STROKE_TYPE_FOREHAND: u8 = 3;
pub const FIT_STROKE_TYPE_BACKHAND: u8 = 4;
pub const FIT_STROKE_TYPE_SMASH: u8 = 5;
pub const FIT_STROKE_TYPE_COUNT: u8 = 6;

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

/// Builds a hash map that maps Sports IDs to human-readable strings.
pub fn init_sport_name_map() -> HashMap<u8, String> {
    let mut sport_name_map = HashMap::<u8, String>::new();

    sport_name_map.insert(FIT_SPORT_GENERIC, "Generic".to_string());
    sport_name_map.insert(FIT_SPORT_RUNNING, "Running".to_string());
    sport_name_map.insert(FIT_SPORT_CYCLING, "Cycling".to_string());
    sport_name_map.insert(FIT_SPORT_TRANSITION, "Transition".to_string());
    sport_name_map.insert(FIT_SPORT_FITNESS_EQUIPMENT, "Fitness Equipment".to_string());
    sport_name_map.insert(FIT_SPORT_SWIMMING, "Swimming".to_string());
    sport_name_map.insert(FIT_SPORT_BASKETBALL, "Basketball".to_string());
    sport_name_map.insert(FIT_SPORT_SOCCER, "Soccer".to_string());
    sport_name_map.insert(FIT_SPORT_TENNIS, "Tennis".to_string());
    sport_name_map.insert(FIT_SPORT_AMERICAN_FOOTBALL, "American Football".to_string());
    sport_name_map.insert(FIT_SPORT_TRAINING, "Training".to_string());
    sport_name_map.insert(FIT_SPORT_WALKING, "Walking".to_string());
    sport_name_map.insert(FIT_SPORT_CROSS_COUNTRY_SKIING, "Cross Country Skiing".to_string());
    sport_name_map.insert(FIT_SPORT_ALPINE_SKIING, "Alpine Skiing".to_string());
    sport_name_map.insert(FIT_SPORT_SNOWBOARDING, "Snowboarding".to_string());
    sport_name_map.insert(FIT_SPORT_ROWING, "Rowing".to_string());
    sport_name_map.insert(FIT_SPORT_MOUNTAINEERING, "Mountaineering".to_string());
    sport_name_map.insert(FIT_SPORT_HIKING, "Hiking".to_string());
    sport_name_map.insert(FIT_SPORT_MULTISPORT, "Multisport".to_string());
    sport_name_map.insert(FIT_SPORT_PADDLING, "Paddling".to_string());
    sport_name_map.insert(FIT_SPORT_FLYING, "Flying".to_string());
    sport_name_map.insert(FIT_SPORT_E_BIKING, "E-Biking".to_string());
    sport_name_map.insert(FIT_SPORT_MOTORCYCLING, "Motorcycling".to_string());
    sport_name_map.insert(FIT_SPORT_BOATING, "Boating".to_string());
    sport_name_map.insert(FIT_SPORT_DRIVING, "Driving".to_string());
    sport_name_map.insert(FIT_SPORT_GOLF, "Golf".to_string());
    sport_name_map.insert(FIT_SPORT_HANG_GLIDING, "Hang Gliding".to_string());
    sport_name_map.insert(FIT_SPORT_HORSEBACK_RIDING, "Horseback Riding".to_string());
    sport_name_map.insert(FIT_SPORT_HUNTING, "Hunting".to_string());
    sport_name_map.insert(FIT_SPORT_FISHING, "Fishing".to_string());
    sport_name_map.insert(FIT_SPORT_INLINE_SKATING, "Inline Skating".to_string());
    sport_name_map.insert(FIT_SPORT_ROCK_CLIMBING, "Rock Climbing".to_string());
    sport_name_map.insert(FIT_SPORT_SAILING, "Sailing".to_string());
    sport_name_map.insert(FIT_SPORT_ICE_SKATING, "Ice Skating".to_string());
    sport_name_map.insert(FIT_SPORT_SKY_DIVING, "Sky Diving".to_string());
    sport_name_map.insert(FIT_SPORT_SNOWSHOEING, "Snowshoeing".to_string());
    sport_name_map.insert(FIT_SPORT_SNOWMOBILING, "Snowmobiling".to_string());
    sport_name_map.insert(FIT_SPORT_STAND_UP_PADDLEBOARDING, "Paddleboarding".to_string());
    sport_name_map.insert(FIT_SPORT_SURFING, "Surfing".to_string());
    sport_name_map.insert(FIT_SPORT_WAKEBOARDING, "Wakeboarding".to_string());
    sport_name_map.insert(FIT_SPORT_WATER_SKIING, "Water Skiing".to_string());
    sport_name_map.insert(FIT_SPORT_KAYAKING, "Kayaking".to_string());
    sport_name_map.insert(FIT_SPORT_RAFTING, "Rafting".to_string());
    sport_name_map.insert(FIT_SPORT_WINDSURFING, "Windsurfng".to_string());
    sport_name_map.insert(FIT_SPORT_KITESURFING, "Kitesurfing".to_string());
    sport_name_map.insert(FIT_SPORT_TACTICAL, "Tactical".to_string());
    sport_name_map.insert(FIT_SPORT_JUMPMASTER, "Jumpmaster".to_string());
    sport_name_map.insert(FIT_SPORT_BOXING, "Boxing".to_string());
    sport_name_map.insert(FIT_SPORT_FLOOR_CLIMBING, "Floor Climbing".to_string());
    sport_name_map.insert(FIT_SPORT_DIVING, "Diving".to_string());
    sport_name_map.insert(FIT_SPORT_ALL, "All".to_string());
    sport_name_map
}

/// Utility function for reading a given number of bytes from a BufReader into a vec.
fn read_n<R: Read>(reader: &mut BufReader<R>, bytes_to_read: u64) -> Result< Vec<u8> >
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let _n = chunk.read_to_end(&mut buf).expect("Didn't read enough");

    Ok(buf)
}

/// Utility function for reading a 32-bit unsigned integer from a BufReader.
fn read_u32<R: Read>(reader: &mut BufReader<R>, is_big_endian: bool) -> Result<u32>
{
    let bytes = read_n(reader, 4)?;
    let num = byte_array_to_uint(bytes, 4, is_big_endian) as u32;

    Ok(num)
}

/// Utility function for reading a byte from a BufReader.
fn read_byte<R: Read>(reader: &mut BufReader<R>) -> Result<u8>
{
    let mut byte: [u8; 1] = [0; 1];
    reader.read_exact(&mut byte)?;

    Ok(byte[0])
}

/// Utility function for reading a null-terminated string from the BufReader.
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

/// Utility function for converting a byte array into a string of the specified number of bytes.
fn byte_array_to_string(bytes: Vec<u8>, num_bytes: usize) -> String {
    let mut result = String::new();

    for i in 0..num_bytes {
        result.push(bytes[i] as char);
    }
    result
}

/// Utility function for converting a byte array to an unsigned int of the given size.
fn byte_array_to_uint(bytes: Vec<u8>, num_bytes: usize, is_big_endian: bool) -> u64 {
    if num_bytes == 1 {
        return bytes[0] as u64;
    }

    let mut num = 0;
    let mut offset = 0;

    if is_big_endian {
        for i in 0..num_bytes {
            num = (num << offset) | (bytes[i] as u64);
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

/// Utility function for converting a byte array to an u64
fn byte_array_to_uint64(bytes: Vec<u8>, is_big_endian: bool) -> u64 {
    let temp = byte_array_to_uint(bytes, 8, is_big_endian);
    temp
}

/// Utility function for converting a byte array to an u32
fn byte_array_to_uint32(bytes: Vec<u8>, is_big_endian: bool) -> u32 {
    let temp = byte_array_to_uint(bytes, 4, is_big_endian) as u32;
    temp
}

/// Utility function for converting a byte array to an u16
fn byte_array_to_uint16(bytes: Vec<u8>, is_big_endian: bool) -> u16 {
    let temp = byte_array_to_uint(bytes, 2, is_big_endian) as u16;
    temp
}

/// Utility function for converting a byte array to an u8
fn byte_array_to_uint8(bytes: Vec<u8>, is_big_endian: bool) -> u8 {
    let temp = byte_array_to_uint(bytes, 1, is_big_endian) as u8;
    temp
}

/// Utility function for converting a byte array to an i64
fn byte_array_to_sint64(bytes: Vec<u8>, is_big_endian: bool) -> i64 {
    let temp = byte_array_to_uint(bytes, 8, is_big_endian) as i64;
    temp
}

/// Utility function for converting a byte array to an i32
fn byte_array_to_sint32(bytes: Vec<u8>, is_big_endian: bool) -> i32 {
    let temp = byte_array_to_uint(bytes, 4, is_big_endian) as i32;
    temp
}

/// Utility function for converting a byte array to an i16
fn byte_array_to_sint16(bytes: Vec<u8>, is_big_endian: bool) -> i16 {
    let temp = byte_array_to_uint(bytes, 2, is_big_endian) as i16;
    temp
}

/// Utility function for converting a byte array to an i8
fn byte_array_to_sint8(bytes: Vec<u8>, is_big_endian: bool) -> i8 {
    let temp = byte_array_to_uint(bytes, 1, is_big_endian) as i8;
    temp
}

/// Utility function for converting a byte array to either a 32 or 64-bit float.
fn byte_array_to_float(bytes: Vec<u8>, num_bytes: usize, _is_big_endian: bool) -> f64 {
    if num_bytes == 1 {
        return bytes[0] as f64;
    }
    else if num_bytes == 4 {
        let byte_array = bytes.try_into().unwrap_or_else(|bytes: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 4, bytes.len()));
        return f32::from_bits(u32::from_be_bytes(byte_array)) as f64;
    }
    else if num_bytes == 8 {
        let byte_array = bytes.try_into().unwrap_or_else(|bytes: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 8, bytes.len()));
        return f64::from_bits(u64::from_be_bytes(byte_array)) as f64;
    }

    0.0
}

/// Utility function for converting between semicircles and degrees.
pub fn semicircles_to_degrees(semicircles: i32) -> f64 {
    let degrees = (semicircles as f64) * 0.000000083819032; // (180.0 / f64::powf(2.0, 31.0));
    degrees
}

pub struct FitSessionMsg {
    pub event: Option<u8>,
    pub event_type: Option<u8>,
    pub start_time: Option<u32>,
    pub start_position_lat: Option<i32>,
    pub start_position_long: Option<i32>,
    pub total_elapsed_time: Option<u32>,
    pub total_timer_time: Option<u32>,
    pub total_distance: Option<u32>,
    pub total_cycles: Option<u32>,
    pub total_calories: Option<u16>,
    pub total_fat_calories: Option<u16>,
    pub avg_speed: Option<u16>,
    pub max_speed: Option<u16>,
    pub avg_heart_rate: Option<u8>,
    pub max_heart_rate: Option<u8>,
    pub avg_cadence: Option<u8>,
    pub max_cadence: Option<u8>,
    pub avg_power: Option<u16>,
    pub max_power: Option<u16>,
    pub total_ascent: Option<u16>,
    pub total_descent: Option<u16>,
    pub total_training_effect: Option<u8>,
    pub first_lap_index: Option<u8>
}

impl FitSessionMsg {
    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FieldValue>) -> Self {
        let mut msg = FitSessionMsg{ event: None, event_type: None, start_time: None, start_position_lat: None, start_position_long: None,
            total_elapsed_time: None, total_timer_time: None, total_distance: None, total_cycles: None, total_calories: None, total_fat_calories: None,
            avg_speed: None, max_speed: None, avg_heart_rate: None, max_heart_rate: None, avg_cadence: None, max_cadence: None, avg_power: None, max_power: None,
            total_ascent: None, total_descent: None, total_training_effect: None, first_lap_index: None };

        for field in fields {
            match field.field_def {
                0 => { msg.event = Some(field.get_u8()); },
                1 => { msg.event_type = Some(field.get_u8()); },
                2 => { msg.start_time = Some(field.get_u32()); },
                3 => { msg.start_position_lat = Some(field.get_i32()); },
                4 => { msg.start_position_long = Some(field.get_i32()); },
                7 => { msg.total_elapsed_time = Some(field.get_u32()); },
                8 => { msg.total_timer_time = Some(field.get_u32()); },
                9 => { msg.total_distance = Some(field.get_u32()); },
                10 => { msg.total_cycles = Some(field.get_u32()); },
                11 => { msg.total_calories = Some(field.get_u16()); },
                13 => { msg.total_fat_calories = Some(field.get_u16()); },
                14 => { msg.avg_speed = Some(field.get_u16()); },
                15 => { msg.max_speed = Some(field.get_u16()); },
                16 => { msg.avg_heart_rate = Some(field.get_u8()); },
                17 => { msg.max_heart_rate = Some(field.get_u8()); },
                18 => { msg.avg_cadence = Some(field.get_u8()); },
                19 => { msg.max_cadence = Some(field.get_u8()); },
                20 => { msg.avg_power = Some(field.get_u16()); },
                21 => { msg.max_power = Some(field.get_u16()); },
                22 => { msg.total_ascent = Some(field.get_u16()); },
                23 => { msg.total_descent = Some(field.get_u16()); },
                24 => { msg.total_training_effect = Some(field.get_u8()); },
                _ => { panic!("Session field not implemented {:#x}", field.field_def); }
            }
        }
        msg
    }
}

pub struct FitDeviceInfoMsg {
    pub timestamp: Option<u32>, // 1 * s + 0,
    pub serial_number: Option<u32>, //
    pub cum_operating_time: Option<u32>, // 1 * s + 0, Reset by new battery or charge.
    pub product_name: Option<String>, // Optional free form string to indicate the devices name or model
    pub manufacturer: Option<u16>, //
    pub product: Option<u16>,
    pub software_version: Option<u16>,
    pub battery_voltage: Option<u16>, // 256 * V + 0,
    pub ant_device_number: Option<u16>, //
    pub device_index: Option<u8>, //
    pub device_type: Option<u8>, //
    pub hardware_version: Option<u8>, //
    pub battery_status: Option<u8>, //
    //FIT_BODY_LOCATION sensor_position, // Indicates the location of the sensor
    pub descriptor: Option<String>, // Used to describe the sensor or location
    pub ant_transmission_type: Option<u8>, //
    //FIT_ANT_NETWORK ant_network, //
    pub source_type: Option<u8> //
}

impl FitDeviceInfoMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FieldValue>) -> Self {
        let mut msg = FitDeviceInfoMsg{ timestamp: None, serial_number: None, cum_operating_time: None, product_name: None, manufacturer: None,
            product: None, software_version: None, battery_voltage: None, ant_device_number: None, device_index: None, device_type: None,
            battery_status: None, hardware_version: None, descriptor: None, ant_transmission_type: None, source_type: None };

        for field in fields {
            match field.field_def {
                0 => { msg.device_index = Some(field.get_u8()); },
                1 => { msg.device_type = Some(field.get_u8()); },
                2 => { msg.manufacturer = Some(field.get_u16()); },
                3 => { msg.serial_number = Some(field.get_u32()); },
                4 => { msg.product = Some(field.get_u16()); },
                5 => { msg.software_version = Some(field.get_u16()); },
                6 => { msg.hardware_version = Some(field.get_u8()); },
                7 => { msg.cum_operating_time = Some(field.get_u32()); },
                10 => { msg.battery_voltage = Some(field.get_u16()); },
                11 => { msg.battery_status = Some(field.get_u8()); },
                16 => { msg.ant_device_number = Some(field.get_u16()); },
                19 => { msg.descriptor = Some(field.string); },
                21 => { msg.ant_device_number = Some(field.get_u16()); },
                25 => { msg.source_type = Some(field.get_u8()); },
                27 => { msg.product_name = Some(field.string); },
                _ => { panic!("Device Info field not implemented {:#x}", field.field_def); }
            }
        }
        msg
    }
}

pub struct FitRecordMsg {
    pub timestamp: Option<u32>, // 1 * s + 0,
    pub position_lat: Option<i32>, // 1 * semicircles + 0,
    pub position_long: Option<i32>, // 1 * semicircles + 0,
    pub distance: Option<u32>, // 100 * m + 0,
    pub time_from_course: Option<i32>, // 1000 * s + 0,
    pub total_cycles: Option<u32>, // 1 * cycles + 0,
    pub accumulated_power: Option<u32>, // 1 * watts + 0,
    pub enhanced_speed: Option<u32>, // 1000 * m/s + 0,
    pub enhanced_altitude: Option<u32>, // 5 * m + 500,
    pub altitude: Option<u16>, // 5 * m + 500,
    pub speed: Option<u16>, // 1000 * m/s + 0,
    pub power: Option<u16>, // 1 * watts + 0,
    pub grade: Option<i16>, // 100 * % + 0,
    pub compressed_accumulated_power: Option<u16>, // 1 * watts + 0,
    pub vertical_speed: Option<i16>, // 1000 * m/s + 0,
    pub calories: Option<u16>, // 1 * kcal + 0,
    pub vertical_oscillation: Option<u16>, // 10 * mm + 0,
    pub stance_time_percent: Option<u16>, // 100 * percent + 0,
    pub stance_time: Option<u16>, // 10 * ms + 0,
    pub ball_speed: Option<u16>, // 100 * m/s + 0,
    pub cadence256: Option<u16>, // 256 * rpm + 0, Log cadence and fractional cadence for backwards compatability
    pub total_hemoglobin_conc: Option<u16>, // 100 * g/dL + 0, Total saturated and unsaturated hemoglobin
    pub total_hemoglobin_conc_min: Option<u16>, // 100 * g/dL + 0, Min saturated and unsaturated hemoglobin
    pub total_hemoglobin_conc_max: Option<u16>, // 100 * g/dL + 0, Max saturated and unsaturated hemoglobin
    pub saturated_hemoglobin_percent: Option<u16>, // 10 * % + 0, Percentage of hemoglobin saturated with oxygen
    pub saturated_hemoglobin_percent_min: Option<u16>, // 10 * % + 0, Min percentage of hemoglobin saturated with oxygen
    pub saturated_hemoglobin_percent_max: Option<u16>, // 10 * % + 0, Max percentage of hemoglobin saturated with oxygen
    pub heart_rate: Option<u8>, // 1 * bpm + 0,
    pub cadence: Option<u8>, // 1 * rpm + 0,
    //FIT_BYTE compressed_speed_distance[FIT_RECORD_MESG_COMPRESSED_SPEED_DISTANCE_COUNT]: Option<u8>, //
    pub resistance: Option<u8>, // Relative. 0 is none  254 is Max.
    pub cycle_length: Option<u8>, // 100 * m + 0,
    pub temperature: Option<i8>, // 1 * C + 0,
    //FIT_UINT8 speed_1s[FIT_RECORD_MESG_SPEED_1S_COUNT], // 16 * m/s + 0, Speed at 1s intervals.  Timestamp field indicates time of last array element.
    pub cycles: Option<u8>, // 1 * cycles + 0,
    pub left_right_balance: Option<u8>,
    pub gps_accuracy: Option<u8>, // 1 * m + 0,
    pub activity_type: Option<u8>,
    pub left_torque_effectiveness: Option<u8>, // 2 * percent + 0,
    pub right_torque_effectiveness: Option<u8>, // 2 * percent + 0,
    pub left_pedal_smoothness: Option<u8>, // 2 * percent + 0,
    pub right_pedal_smoothness: Option<u8>, // 2 * percent + 0,
    pub combined_pedal_smoothness: Option<u8>, // 2 * percent + 0,
    pub time128: Option<u8>, // 128 * s + 0,
    pub stroke_type: Option<u8>,
    pub zone: Option<u8>,
    pub fractional_cadence: Option<u8>, // 128 * rpm + 0,
    //FIT_DEVICE_INDEX device_index;
    pub battery_soc: Option<u8>
}

impl FitRecordMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FieldValue>) -> Self {

        let mut msg = FitRecordMsg{ timestamp: None, position_lat: None, position_long: None, distance: None, time_from_course: None, total_cycles: None, accumulated_power: None,
            enhanced_speed: None, enhanced_altitude: None, altitude: None, speed: None, power: None, grade: None, compressed_accumulated_power: None, vertical_speed: None,
            calories: None, vertical_oscillation: None, stance_time_percent: None, stance_time: None, ball_speed: None, cadence256: None, total_hemoglobin_conc: None,
            total_hemoglobin_conc_min: None, total_hemoglobin_conc_max: None, saturated_hemoglobin_percent: None, saturated_hemoglobin_percent_min: None,
            saturated_hemoglobin_percent_max: None, heart_rate: None, cadence: None, resistance: None, cycle_length: None, temperature: None,
            cycles: None, left_right_balance: None, gps_accuracy: None, activity_type: None, left_torque_effectiveness: None, right_torque_effectiveness: None,
            left_pedal_smoothness: None, right_pedal_smoothness: None, combined_pedal_smoothness: None, time128: None, stroke_type: None, zone: None, fractional_cadence: None,
            battery_soc: None };

        for field in fields {
            match field.field_def {
                0 => { msg.position_lat = Some(field.get_i32()); },
                1 => { msg.position_long = Some(field.get_i32()); },
                2 => { msg.altitude = Some(field.get_u16()); },
                3 => { msg.heart_rate = Some(field.get_u8()); },
                4 => { msg.cadence = Some(field.get_u8()); },
                5 => { msg.distance = Some(field.get_u32()); },
                6 => { msg.speed = Some(field.get_u16()); },
                7 => { msg.power = Some(field.get_u16()); },
                9 => { msg.grade = Some(field.get_i16()); },
                13 => { msg.temperature = Some(field.get_i8()); },
                31 => { msg.gps_accuracy = Some(field.get_u8()); },
                43 => { msg.left_torque_effectiveness = Some(field.get_u8()); },
                44 => { msg.right_torque_effectiveness = Some(field.get_u8()); },
                45 => { msg.left_pedal_smoothness = Some(field.get_u8()); },
                46 => { msg.right_pedal_smoothness = Some(field.get_u8()); },
                81 => { msg.battery_soc = Some(field.get_u8()); },
                _ => { panic!("Record field not implemented {:#x}", field.field_def); }
            }
        }
        msg
    }
}

pub enum FieldType {
    FieldTypeNotSet, // Value not set
    FieldTypeUInt, // Value is an unsigned integer
    FieldTypeSInt, // Value is a signed integer
    FieldTypeFloat, // Value is a float
    FieldTypeByteArray, // Value is a byte array
    FieldTypeStr // Value is a tring
}

pub struct FieldValue {
    pub field_def: u8, // From the message definition
    pub field_type: FieldType, // Tells us which of the following to use
    pub num_uint: u64,
    pub num_sint: i64,
    pub num_float: f64,
    pub byte_array: Vec<u8>,
    pub string: String
}

impl FieldValue {
    pub fn new() -> Self {
        let state = FieldValue{ field_def: 0, field_type: FieldType::FieldTypeNotSet, num_uint: 0, num_sint: 0, num_float: 0.0, byte_array: Vec::<u8>::new(), string: String::new() };
        state
    }

    pub fn get_i8(&self) -> i8 {
        return self.num_sint as i8;
    }

    pub fn get_i16(&self) -> i16 {
        return self.num_sint as i16;
    }

    pub fn get_i32(&self) -> i32 {
        return self.num_sint as i32;
    }

    pub fn get_i64(&self) -> i64 {
        return self.num_sint as i64;
    }

    pub fn get_u8(&self) -> u8 {
        return self.num_uint as u8;
    }

    pub fn get_u16(&self) -> u16 {
        return self.num_uint as u16;
    }

    pub fn get_u32(&self) -> u32 {
        return self.num_uint as u32;
    }

    pub fn get_u64(&self) -> u64 {
        return self.num_uint as u64;
    }
}

/// Encapsulates a custom field definition, as described by definition messages and used by data messages.
#[derive(Copy, Clone, Debug, Default)]
pub struct FieldDefinition {
    pub field_def: u8,
    pub size: u8,
    pub base_type: u8
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

pub type FieldDefinitionList = Vec<FieldDefinition>;

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
    timestamp: u32 // For use with the compressed timestamp header
}

impl FitState {
    pub fn new() -> Self {
        let state = FitState{ is_big_endian: false, current_global_msg_num: 0, global_msg_map: HashMap::<u16, GlobalMessage>::new(), timestamp: 0 };
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

    /// Adds the given global message/local message combo to the hash map.
    fn insert_local_msg_def(&mut self, global_msg_num: u16, local_msg_type: u8, local_msg_def: FieldDefinitionList) {
        self.global_msg_map.entry(global_msg_num)
            .and_modify(|e| { e.insert_msg_def(local_msg_type, local_msg_def) })
            .or_insert(GlobalMessage::new());
    }

    /// Given a global message number and local message number, retrieves the corresonding field definitions.s
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
        state.is_big_endian = definition_header[DEF_MSG_ARCHITECTURE] == 1;
        let global_msg_num = byte_array_to_uint(definition_header[DEF_MSG_GLOBAL_MSG_NUM..(DEF_MSG_GLOBAL_MSG_NUM + 2)].to_vec(), 2, state.is_big_endian) as u16;
        state.current_global_msg_num = global_msg_num;

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

        // The timestamp may get updated.
        let mut new_timestamp = state.timestamp;

        // Retrieve the field definitions based on the message type.
        let field_defs = state.retrieve_msg_def(state.current_global_msg_num, local_msg_type);
        match field_defs {
            Some(field_defs) => {

                // Read data for each message definition.
                let mut fields = Vec::new();
                for def in field_defs.iter() {

                    let mut field = FieldValue::new();
                    field.field_def = def.field_def;

                    // Read the number of bytes prescribed by the field definition.
                    let data = read_n(reader, def.size as u64)?;

                    // Is this a special field, like a timestamp?
                    if def.field_def == FIELD_MSG_INDEX {
                        panic!("Message Index not implemented: global message num: {} local message type: {}.", state.current_global_msg_num, local_msg_type);
                    }
                    else if def.field_def == FIELD_TIMESTAMP {
                        new_timestamp = byte_array_to_uint(data, 4, state.is_big_endian) as u32;
                    }
                    else if def.field_def == FIELD_PART_INDEX {
                        panic!("Part Index not implemented: global message num: {} local message type: {}.", state.current_global_msg_num, local_msg_type);
                    }

                    // Normal field.
                    else {
                        match def.base_type {
                            0x00 => { field.num_uint = byte_array_to_uint8(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            0x01 => { field.num_sint = byte_array_to_sint8(data, state.is_big_endian) as i64; field.field_type = FieldType::FieldTypeSInt; fields.push(field); },
                            0x02 => { field.num_uint = byte_array_to_uint8(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            0x83 => { field.num_sint = byte_array_to_sint16(data, state.is_big_endian) as i64; field.field_type = FieldType::FieldTypeSInt; fields.push(field); },
                            0x84 => { field.num_uint = byte_array_to_uint16(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            0x85 => { field.num_sint = byte_array_to_sint32(data, state.is_big_endian) as i64; field.field_type = FieldType::FieldTypeSInt; fields.push(field); },
                            0x86 => { field.num_uint = byte_array_to_uint32(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            0x07 => { field.string = byte_array_to_string(data, def.size as usize); field.field_type = FieldType::FieldTypeStr; fields.push(field); },
                            0x88 => { field.num_float = byte_array_to_float(data, 4, state.is_big_endian); field.field_type = FieldType::FieldTypeFloat; fields.push(field); },
                            0x89 => { field.num_float = byte_array_to_float(data, 8, state.is_big_endian); field.field_type = FieldType::FieldTypeFloat; fields.push(field); },
                            0x0A => { field.num_uint = byte_array_to_uint8(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            0x8B => { field.num_uint = byte_array_to_uint16(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            0x8C => { field.num_uint = byte_array_to_uint32(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            0x0D => { field.byte_array = data; field.field_type = FieldType::FieldTypeByteArray; fields.push(field); },
                            0x8E => { field.num_sint = byte_array_to_sint64(data, state.is_big_endian) as i64; field.field_type = FieldType::FieldTypeSInt; fields.push(field); },
                            0x8F => { field.num_uint = byte_array_to_uint64(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            0x90 => { field.num_uint = byte_array_to_uint64(data, state.is_big_endian) as u64; field.field_type = FieldType::FieldTypeUInt; fields.push(field); },
                            _ => { panic!("Base Type not implemented {:#x}", def.base_type); }
                        }
                    }
                }

                // Tell the people.
                callback(state.timestamp, state.current_global_msg_num, local_msg_type, fields);
            },
            None    => {
                let e = Error::new(std::io::ErrorKind::Other, "Message definition not found.");
                return Err(e);
            },
        }

        // Store the (possibly) updated timestamp.
        state.timestamp = new_timestamp;

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the compressed timestamp message, reads the message.
    fn read_compressed_timestamp_message<R: Read>(&mut self, reader: &mut BufReader<R>, header_byte: u8, state: &mut FitState, callback: Callback) -> Result<()> {

        // Compressed Timestamp Header.
        let time_offset = (header_byte & 0x1f) as u32;
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
