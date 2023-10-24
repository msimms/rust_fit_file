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

// Record message header byte offsets.
const RECORD_HDR_NORMAL: u8 = 0x80;
const RECORD_HDR_MSG_TYPE: u8 = 0x40;
const RECORD_HDR_MSG_TYPE_SPECIFIC: u8 = 0x20;
const RECORD_HDR_RESERVED: u8 = 0x10;
const RECORD_HDR_LOCAL_MSG_TYPE: u8 = 0x0f;
const RECORD_HDR_LOCAL_MSG_TYPE_COMPRESSED: u8 = 0x60;

// Global message numbers.
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

// File enumeration, used in the FileId message.
pub const FIT_FILE_DEVICE: u8 = 1;
pub const FIT_FILE_SETTINGS: u8 = 2;
pub const FIT_FILE_SPORT: u8 = 3;
pub const FIT_FILE_ACTIVITY: u8 = 4;
pub const FIT_FILE_WORKOUT: u8 = 5;
pub const FIT_FILE_COURSE: u8 = 6;
pub const FIT_FILE_SCHEDULES: u8 = 7;
pub const FIT_FILE_WEIGHT: u8 = 9;
pub const FIT_FILE_TOTALS: u8 = 10;
pub const FIT_FILE_GOALS: u8 = 11;
pub const FIT_FILE_BLOOD_PRESSURE: u8 = 14;
pub const FIT_FILE_MONITORING_A: u8 = 15;
pub const FIT_FILE_ACTIVITY_SUMMARY: u8 = 20;
pub const FIT_FILE_MONITORING_DAILY: u8 = 28;
pub const FIT_FILE_MONITORING_B: u8 = 32;
pub const FIT_FILE_SEGMENT: u8 = 34;
pub const FIT_FILE_SEGMENT_LIST: u8 = 35;
pub const FIT_FILE_EXD_CONFIGURATION: u8 = 50;
pub const FIT_FILE_MFG_RANGE_MIN: u8 = 0xF7;
pub const FIT_FILE_MFG_RANGE_MAX: u8 = 0xFE;

// Sport enumeration.
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

// Sub-sport enumeration
pub const FIT_SUB_SPORT_GENERIC : u8 = 0;
pub const FIT_SUB_SPORT_TREADMILL : u8 = 1;
pub const FIT_SUB_SPORT_STREET : u8 = 2;
pub const FIT_SUB_SPORT_TRAIL : u8 = 3;
pub const FIT_SUB_SPORT_TRACK : u8 = 4;
pub const FIT_SUB_SPORT_SPIN : u8 = 5;
pub const FIT_SUB_SPORT_INDOOR_CYCLING : u8 = 6;
pub const FIT_SUB_SPORT_ROAD : u8 = 7;
pub const FIT_SUB_SPORT_MOUNTAIN : u8 = 8;
pub const FIT_SUB_SPORT_DOWNHILL : u8 = 9;
pub const FIT_SUB_SPORT_RECUMBENT : u8 = 10;
pub const FIT_SUB_SPORT_CYCLOCROSS : u8 = 11;
pub const FIT_SUB_SPORT_HAND_CYCLING : u8 = 12;
pub const FIT_SUB_SPORT_TRACK_CYCLING : u8 = 13;
pub const FIT_SUB_SPORT_INDOOR_ROWING : u8 = 14;
pub const FIT_SUB_SPORT_ELLIPTICAL : u8 = 15;
pub const FIT_SUB_SPORT_STAIR_CLIMBING : u8 = 16;
pub const FIT_SUB_SPORT_LAP_SWIMMING : u8 = 17;
pub const FIT_SUB_SPORT_OPEN_WATER : u8 = 18;
pub const FIT_SUB_SPORT_FLEXIBILITY_TRAINING : u8 = 19;
pub const FIT_SUB_SPORT_STRENGTH_TRAINING : u8 = 20;
pub const FIT_SUB_SPORT_WARM_UP : u8 = 21;
pub const FIT_SUB_SPORT_MATCH : u8 = 22;
pub const FIT_SUB_SPORT_EXERCISE : u8 = 23;
pub const FIT_SUB_SPORT_CHALLENGE : u8 = 24;
pub const FIT_SUB_SPORT_INDOOR_SKIING : u8 = 25;
pub const FIT_SUB_SPORT_CARDIO_TRAINING : u8 = 26;
pub const FIT_SUB_SPORT_INDOOR_WALKING : u8 = 27;
pub const FIT_SUB_SPORT_E_BIKE_FITNESS : u8 = 28;
pub const FIT_SUB_SPORT_BMX : u8 = 29;
pub const FIT_SUB_SPORT_CASUAL_WALKING : u8 = 30;
pub const FIT_SUB_SPORT_SPEED_WALKING : u8 = 31;
pub const FIT_SUB_SPORT_BIKE_TO_RUN_TRANSITION : u8 = 32;
pub const FIT_SUB_SPORT_RUN_TO_BIKE_TRANSITION : u8 = 33;
pub const FIT_SUB_SPORT_SWIM_TO_BIKE_TRANSITION : u8 = 34;
pub const FIT_SUB_SPORT_ATV : u8 = 35;
pub const FIT_SUB_SPORT_MOTOCROSS : u8 = 36;
pub const FIT_SUB_SPORT_BACKCOUNTRY : u8 = 37;
pub const FIT_SUB_SPORT_RESORT : u8 = 38;
pub const FIT_SUB_SPORT_RC_DRONE : u8 = 39;
pub const FIT_SUB_SPORT_WINGSUIT : u8 = 40;
pub const FIT_SUB_SPORT_WHITEWATER : u8 = 41;
pub const FIT_SUB_SPORT_SKATE_SKIING : u8 = 42;
pub const FIT_SUB_SPORT_YOGA : u8 = 43;
pub const FIT_SUB_SPORT_PILATES : u8 = 44;
pub const FIT_SUB_SPORT_INDOOR_RUNNING : u8 = 45;
pub const FIT_SUB_SPORT_GRAVEL_CYCLING : u8 = 46;
pub const FIT_SUB_SPORT_E_BIKE_MOUNTAIN : u8 = 47;
pub const FIT_SUB_SPORT_COMMUTING : u8 = 48;
pub const FIT_SUB_SPORT_MIXED_SURFACE : u8 = 49;
pub const FIT_SUB_SPORT_NAVIGATE : u8 = 50;
pub const FIT_SUB_SPORT_TRACK_ME : u8 = 51;
pub const FIT_SUB_SPORT_MAP : u8 = 52;
pub const FIT_SUB_SPORT_SINGLE_GAS_DIVING : u8 = 53;
pub const FIT_SUB_SPORT_MULTI_GAS_DIVING : u8 = 54;
pub const FIT_SUB_SPORT_GAUGE_DIVING : u8 = 55;
pub const FIT_SUB_SPORT_APNEA_DIVING : u8 = 56;
pub const FIT_SUB_SPORT_APNEA_HUNTING : u8 = 57;
pub const FIT_SUB_SPORT_VIRTUAL_ACTIVITY : u8 = 58;
pub const FIT_SUB_SPORT_OBSTACLE : u8 = 59;
pub const FIT_SUB_SPORT_BREATHING : u8 = 62;
pub const FIT_SUB_SPORT_SAIL_RACE : u8 = 65;
pub const FIT_SUB_SPORT_ULTRA : u8 = 67;
pub const FIT_SUB_SPORT_INDOOR_CLIMBING : u8 = 68;
pub const FIT_SUB_SPORT_BOULDERING : u8 = 69;
pub const FIT_SUB_SPORT_HIIT : u8 = 70;
pub const FIT_SUB_SPORT_AMRAP : u8 = 73;
pub const FIT_SUB_SPORT_EMOM : u8 = 74;
pub const FIT_SUB_SPORT_TABATA : u8 = 75;
pub const FIT_SUB_SPORT_PICKLEBALL : u8 = 84;
pub const FIT_SUB_SPORT_PADEL : u8 = 85;
pub const FIT_SUB_SPORT_FLY_CANOPY : u8 = 110;
pub const FIT_SUB_SPORT_FLY_PARAGLIDE : u8 = 111;
pub const FIT_SUB_SPORT_FLY_PARAMOTOR : u8 = 112;
pub const FIT_SUB_SPORT_FLY_PRESSURIZED : u8 = 113;
pub const FIT_SUB_SPORT_FLY_NAVIGATE : u8 = 114;
pub const FIT_SUB_SPORT_FLY_TIMER : u8 = 115;
pub const FIT_SUB_SPORT_FLY_ALTIMETER : u8 = 116;
pub const FIT_SUB_SPORT_FLY_WX : u8 = 117;
pub const FIT_SUB_SPORT_FLY_VFR : u8 = 118;
pub const FIT_SUB_SPORT_FLY_IFR : u8 = 119;
pub const FIT_SUB_SPORT_ALL : u8 = 254;

// Swim stroke enumeration.
pub const FIT_ENUM_INVALID: u8 = 0xff;
pub const FIT_STROKE_TYPE_INVALID: u8 = FIT_ENUM_INVALID;
pub const FIT_STROKE_TYPE_NO_EVENT: u8 = 0;
pub const FIT_STROKE_TYPE_OTHER: u8 = 1; // stroke was detected but cannot be identified
pub const FIT_STROKE_TYPE_SERVE: u8 = 2;
pub const FIT_STROKE_TYPE_FOREHAND: u8 = 3;
pub const FIT_STROKE_TYPE_BACKHAND: u8 = 4;
pub const FIT_STROKE_TYPE_SMASH: u8 = 5;
pub const FIT_STROKE_TYPE_COUNT: u8 = 6;

// Lap trigger enumation.
pub const FIT_LAP_TRIGGER_MANUAL: u8 = 0;
pub const FIT_LAP_TRIGGER_TIME: u8 = 1;
pub const FIT_LAP_TRIGGER_DISTANCE: u8 = 2;
pub const FIT_LAP_TRIGGER_POSITION_START: u8 = 3;
pub const FIT_LAP_TRIGGER_POSITION_LAP: u8 = 4;
pub const FIT_LAP_TRIGGER_POSITION_WAYPOINT: u8 = 5;
pub const FIT_LAP_TRIGGER_POSITION_MARKED: u8 = 6;
pub const FIT_LAP_TRIGGER_SESSION_END: u8 = 7;
pub const FIT_LAP_TRIGGER_FITNESS_EQUIPMENT: u8 = 8;

// Base types.
pub const FIT_BASE_TYPE_ENUM: u8 = 0x00;
pub const FIT_BASE_TYPE_SINT8: u8 = 0x01;
pub const FIT_BASE_TYPE_UINT8: u8 = 0x02;
pub const FIT_BASE_TYPE_SINT16: u8 = 0x83;
pub const FIT_BASE_TYPE_UINT16: u8 = 0x84;
pub const FIT_BASE_TYPE_SINT32: u8 = 0x85;
pub const FIT_BASE_TYPE_UINT32: u8 = 0x86;
pub const FIT_BASE_TYPE_STRING: u8 = 0x07;
pub const FIT_BASE_TYPE_FLOAT32: u8 = 0x88;
pub const FIT_BASE_TYPE_FLOAT64: u8 = 0x89;
pub const FIT_BASE_TYPE_UINT8Z: u8 = 0x0A;
pub const FIT_BASE_TYPE_UINT16Z: u8 = 0x8B;
pub const FIT_BASE_TYPE_UINT32Z: u8 = 0x8C;
pub const FIT_BASE_TYPE_BYTE: u8 = 0x0D;
pub const FIT_BASE_TYPE_SINT64: u8 = 0x8E;
pub const FIT_BASE_TYPE_UINT64: u8 = 0x8F;
pub const FIT_BASE_TYPE_UINT64Z: u8 = 0x90;

pub const WORKOUT_CAPABILITIES_INTERVAL : u32 = 0x00000001;
pub const WORKOUT_CAPABILITIES_CUSTOM : u32 = 0x00000002;
pub const WORKOUT_CAPABILITIES_FITNESS_EQUIPMENT : u32 = 0x00000004;
pub const WORKOUT_CAPABILITIES_FIRSTBEAT : u32 = 0x00000008;
pub const WORKOUT_CAPABILITIES_NEW_LEAF : u32 = 0x00000010;
pub const WORKOUT_CAPABILITIES_TCX : u32 = 0x00000020;
/// Speed source required for workout step.
pub const WORKOUT_CAPABILITIES_SPEED : u32 = 0x00000080;
/// Heart rate source required for workout step.
pub const WORKOUT_CAPABILITIES_HEART_RATE : u32 = 0x00000100;
/// Distance source required for workout step.
pub const WORKOUT_CAPABILITIES_DISTANCE : u32 = 0x00000200;
/// Distance source required for workout step.
pub const WORKOUT_CAPABILITIES_CADENCE : u32 = 0x00000400;
/// Power source required for workout step.
pub const WORKOUT_CAPABILITIES_POWER : u32 = 0x00000800;
/// Grade source required for workout step.
pub const WORKOUT_CAPABILITIES_GRADE : u32 = 0x00001000;
/// Resistance source required for workout step.
pub const WORKOUT_CAPABILITIES_RESISTANCE : u32 = 0x00002000;
pub const WORKOUT_CAPABILITIES_PROTECTED : u32 = 0x00004000;

pub const WORKOUT_EQUIPMENT_NONE : u8 = 0;
pub const WORKOUT_EQUIPMENT_SWIM_FINS : u8 = 1;
pub const WORKOUT_EQUIPMENT_SWIM_KICKBOARD : u8 = 2;
pub const WORKOUT_EQUIPMENT_SWIM_PADDLES : u8 = 3;
pub const WORKOUT_EQUIPMENT_PULL_BUOY : u8 = 4;
pub const WORKOUT_EQUIPMENT_SWIM_SNORKEL : u8 = 5;

pub const WORKOUT_STEP_DURATION_TIME : u8 = 0;
pub const WORKOUT_STEP_DURATION_DISTANCE : u8 = 1;
pub const WORKOUT_STEP_DURATION_HEART_RATE_LESS_THAN : u8 = 2;
pub const WORKOUT_STEP_DURATION_HEART_RATE_GREATER_THAN : u8 = 3;
pub const WORKOUT_STEP_DURATION_CALORIES : u8 = 4;
pub const WORKOUT_STEP_DURATION_OPEN : u8 = 5;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_STEPS_COMPLETE : u8 = 6;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_TIME : u8 = 7;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_DISTANCE : u8 = 8;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_CALORIES : u8 = 9;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_HEART_RATE_LESS_THAN : u8 = 10;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_HEART_RATE_GREATER_THAN : u8 = 11;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_POWER_LESS_THAN : u8 = 12;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_POWER_GREATER_THAN : u8 =  13;
pub const WORKOUT_STEP_DURATION_POWER_LESS_THAN : u8 = 14;
pub const WORKOUT_STEP_DURATION_POWER_GREATER_THAN : u8 = 15;
pub const WORKOUT_STEP_DURATION_TRAINING_PEAKS_TSS : u8 = 16;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_POWER_LAST_LAP_LESS_THAN : u8 = 17;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_MAX_POWER_LAST_LAP_LESS_THAN : u8 = 18;
pub const WORKOUT_STEP_DURATION_POWER3S_LESS_THAN : u8 = 19;
pub const WORKOUT_STEP_DURATION_POWER10S_LESS_THAN : u8 = 20;
pub const WORKOUT_STEP_DURATION_POWER30S_LESS_THAN : u8 = 21;
pub const WORKOUT_STEP_DURATION_POWER3S_GREATER_THAN : u8 = 22;
pub const WORKOUT_STEP_DURATION_POWER10S_GREATER_THAN : u8 = 23;
pub const WORKOUT_STEP_DURATION_POWER30S_GREATER_THAN : u8 = 24;
pub const WORKOUT_STEP_DURATION_POWER_LAP_LESS_THAN : u8 = 25;
pub const WORKOUT_STEP_DURATION_POWER_LAP_GREATER_THAN : u8 = 26;
pub const WORKOUT_STEP_DURATION_REPEAT_UNTIL_TRAINING_PEAKS_TSS : u8 = 27;
pub const WORKOUT_STEP_DURATION_REPETITION_TIME : u8 = 28;
pub const WORKOUT_STEP_DURATION_REPS : u8 = 29;
pub const WORKOUT_STEP_DURATION_TIME_ONLY : u8 = 31;

pub const WORKOUT_STEP_TARGET_SPEED : u8 = 0;
pub const WORKOUT_STEP_TARGET_HEART_RATE : u8 = 1;
pub const WORKOUT_STEP_TARGET_OPEN : u8 = 2;
pub const WORKOUT_STEP_TARGET_CADENCE : u8 = 3;
pub const WORKOUT_STEP_TARGET_POWER : u8 = 4;
pub const WORKOUT_STEP_TARGET_GRADE : u8 = 5;
pub const WORKOUT_STEP_TARGET_RESISTANCE : u8 = 6;
pub const WORKOUT_STEP_TARGET_POWER_3S : u8 = 7;
pub const WORKOUT_STEP_TARGET_POWER_10S : u8 = 8;
pub const WORKOUT_STEP_TARGET_POWER_30S : u8 = 9;
pub const WORKOUT_STEP_TARGET_POWER_LAP : u8 = 10;
pub const WORKOUT_STEP_TARGET_SWIM_STROKE : u8 = 11;
pub const WORKOUT_STEP_TARGET_SPEED_LAP : u8 = 12;
pub const WORKOUT_STEP_TARGET_HEART_RATE_LAP : u8 = 13;

pub const INTENSITY_ACTIVE : u8 = 0;
pub const INTENSITY_REST : u8 = 1;
pub const INTENSITY_WARM_UP : u8 = 2;
pub const INTENSITY_COOL_DOWN : u8 = 3;
pub const INTENSITY_RECOVERY : u8 = 4;
pub const INTENSITY_INTERVAL : u8 = 5;
pub const INTENSITY_OTHER : u8 = 6;

pub const DISPLAY_MEASURE_METRIC : u8 = 0;
pub const DISPLAY_MEASURE_STATUTE : u8 = 1;
pub const DISPLAY_MEASURE_NAUTICAL : u8 = 2;

type Callback<T> = fn(timestamp: u32, global_message_num: u16, local_message_type: u8, message_index: u16, data: Vec<FitFieldValue>, context: &mut T);

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
    global_msg_name_map.insert(GLOBAL_MSG_NUM_GPS_METADATA, "GPS Metadata".to_string());
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
    let num = byte_array_to_uint32(bytes, is_big_endian);

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
    result.trim_end_matches('\0').to_string()
}

/// Utility function for converting a byte array to an unsigned int of the given size.
fn byte_array_to_num(bytes: Vec<u8>, num_bytes: usize, is_big_endian: bool) -> u64 {

    let mut num: u64 = 0;

    if bytes.len() < num_bytes {
        panic!("Unexpected length; got {} when expecting {}", bytes.len(), num_bytes);
    }

    if is_big_endian {
        for i in 0..num_bytes {
            num = (num << 8) | (bytes[i] as u64);
        }
    }
    else {
        for i in 0..num_bytes {
            num = (num << 8) | (bytes[num_bytes - i - 1] as u64);
        }
    }

    num
}

/// Utility function for converting a byte array to an u64
fn byte_array_to_uint64(bytes: Vec<u8>, is_big_endian: bool) -> u64 {
    let temp = byte_array_to_num(bytes, 8, is_big_endian);
    temp
}

/// Utility function for converting a byte array to an u32
fn byte_array_to_uint32(bytes: Vec<u8>, is_big_endian: bool) -> u32 {
    let temp = byte_array_to_num(bytes, 4, is_big_endian) as u32;
    temp
}

/// Utility function for converting a byte array to an u16
fn byte_array_to_uint16(bytes: Vec<u8>, is_big_endian: bool) -> u16 {
    let temp = byte_array_to_num(bytes, 2, is_big_endian) as u16;
    temp
}

/// Utility function for converting a byte array to an u8
fn byte_array_to_uint8(bytes: Vec<u8>) -> u8 {
    bytes[0]
}

/// Utility function for converting a byte array to an i64
fn byte_array_to_sint64(bytes: Vec<u8>, is_big_endian: bool) -> i64 {
    let temp = byte_array_to_num(bytes, 8, is_big_endian) as i64;
    temp
}

/// Utility function for converting a byte array to an i32
fn byte_array_to_sint32(bytes: Vec<u8>, is_big_endian: bool) -> i32 {
    let temp = byte_array_to_num(bytes, 4, is_big_endian) as i32;
    temp
}

/// Utility function for converting a byte array to an i16
fn byte_array_to_sint16(bytes: Vec<u8>, is_big_endian: bool) -> i16 {
    let temp = byte_array_to_num(bytes, 2, is_big_endian) as i16;
    temp
}

/// Utility function for converting a byte array to an i8
fn byte_array_to_sint8(bytes: Vec<u8>) -> i8 {
    let temp = bytes[0] as i8;
    temp
}

/// Utility function for converting a byte array to either a 32 or 64-bit float.
fn byte_array_to_float(bytes: Vec<u8>, num_bytes: usize, _is_big_endian: bool) -> f64 {
    if num_bytes == 1 {
        return bytes[0] as f64;
    }
    else if num_bytes == 4 {
        let byte_array = bytes.try_into().unwrap_or_else(|bytes: Vec<u8>| panic!("Expected a Vec of length {} but it was {}.", 4, bytes.len()));
        return f32::from_bits(u32::from_be_bytes(byte_array)) as f64;
    }
    else if num_bytes == 8 {
        let byte_array = bytes.try_into().unwrap_or_else(|bytes: Vec<u8>| panic!("Expected a Vec of length {} but it was {}.", 8, bytes.len()));
        return f64::from_bits(u64::from_be_bytes(byte_array)) as f64;
    }

    0.0
}

/// Utility function for converting between semicircles and degrees.
pub fn semicircles_to_degrees(semicircles: i32) -> f64 {
    let degrees = (semicircles as f64) * 0.000000083819032; // (180.0 / f64::powf(2.0, 31.0));
    degrees
}

// Auto-generated by print_message_struct in lib.rs
pub struct FitFileIdMsg {
    pub manufacturer: Option<u8>,
    pub serial_number: Option<u32>,
    pub time_created: Option<u32>,
    pub product_name: Option<String>,
    pub number: Option<u16>,
    pub file_type: Option<u8>,
    pub product: Option<u16>,
}

impl FitFileIdMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FitFieldValue>) -> Self {
        let mut msg = FitFileIdMsg { manufacturer: None, 
            serial_number: None, time_created: None, product_name: None, 
            number: None, file_type: None, product: None, 
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    1 => { msg.manufacturer = Some(field.get_u8()); },
                    3 => { msg.serial_number = Some(field.get_u32()); },
                    4 => { msg.time_created = Some(field.get_u32()); },
                    8 => { msg.product_name = Some(field.value_string); },
                    5 => { msg.number = Some(field.get_u16()); },
                    0 => { msg.file_type = Some(field.get_u8()); },
                    2 => { msg.product = Some(field.get_u16()); },
                    _ => { panic!("FileId field not implemented {:#x}", field.field_def); }
                }
            }
        }
        msg
    }
}

// Auto-generated by print_message_struct in lib.rs
pub struct FitFileCreatorMsg {
    pub hardware_version: Option<u8>,
    pub software_version: Option<u16>,
}

impl FitFileCreatorMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FitFieldValue>) -> Self {
        let mut msg = FitFileCreatorMsg { hardware_version: None, 
            software_version: None, 
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    1 => { msg.hardware_version = Some(field.get_u8()); },
                    0 => { msg.software_version = Some(field.get_u16()); },
                    _ => { panic!("FileCreator field not implemented {:#x}", field.field_def); }
                }
            }
        }
        msg
    }
}

// Auto-generated by print_message_struct in lib.rs
pub struct FitSessionMsg {
    pub total_cycles: Option<u32>,
    pub num_lengths: Option<u16>,
    pub total_distance: Option<u32>,
    pub avg_stance_time: Option<u16>,
    pub avg_right_pedal_smoothness: Option<u8>,
    pub total_moving_time: Option<u32>,
    pub avg_vertical_ratio: Option<u16>,
    pub best_lap_index: Option<u16>,
    pub timestamp: Option<u32>,
    pub avg_altitude: Option<u16>,
    pub swim_stroke: Option<u8>,
    pub total_fractional_descent: Option<u8>,
    pub max_neg_vertical_speed: Option<i16>,
    pub max_fractional_cadence: Option<u8>,
    pub intensity_factor: Option<u16>,
    pub max_ball_speed: Option<u16>,
    pub avg_lev_motor_power: Option<u16>,
    pub jump_count: Option<u16>,
    pub max_power: Option<u16>,
    pub num_active_lengths: Option<u16>,
    pub max_neg_grade: Option<i16>,
    pub training_stress_score: Option<u16>,
    pub enhanced_max_altitude: Option<u32>,
    pub max_cadence_position: Option<u8>,
    pub total_calories: Option<u16>,
    pub avg_pos_grade: Option<i16>,
    pub time_in_cadence_zone: Option<u32>,
    pub zone_count: Option<u16>,
    pub enhanced_max_speed: Option<u32>,
    pub avg_cadence: Option<u8>,
    pub total_fractional_ascent: Option<u8>,
    pub total_elapsed_time: Option<u32>,
    pub swc_long: Option<i32>,
    pub max_pos_vertical_speed: Option<i16>,
    pub avg_stance_time_balance: Option<u16>,
    pub max_saturated_hemoglobin_percent: Option<u16>,
    pub event_type: Option<u8>,
    pub first_lap_index: Option<u16>,
    pub enhanced_avg_speed: Option<u32>,
    pub avg_flow: Option<f32>,
    pub time_in_hr_zone: Option<u32>,
    pub pool_length_unit: Option<u8>,
    pub max_cadence: Option<u8>,
    pub event_group: Option<u8>,
    pub avg_cadence_position: Option<u8>,
    pub start_position_long: Option<i32>,
    pub total_timer_time: Option<u32>,
    pub trigger: Option<u8>,
    pub max_speed: Option<u16>,
    pub nec_lat: Option<i32>,
    pub total_work: Option<u32>,
    pub min_total_hemoglobin_conc: Option<u16>,
    pub min_saturated_hemoglobin_percent: Option<u16>,
    pub max_lev_motor_power: Option<u16>,
    pub swc_lat: Option<i32>,
    pub avg_left_power_phase: Option<u8>,
    pub avg_ball_speed: Option<u16>,
    pub stroke_count: Option<u16>,
    pub total_fractional_cycles: Option<u8>,
    pub enhanced_avg_altitude: Option<u32>,
    pub total_training_effect: Option<u8>,
    pub avg_neg_grade: Option<i16>,
    pub min_altitude: Option<u16>,
    pub max_pos_grade: Option<i16>,
    pub avg_right_power_phase_peak: Option<u8>,
    pub avg_right_torque_effectiveness: Option<u8>,
    pub avg_neg_vertical_speed: Option<i16>,
    pub max_total_hemoglobin_conc: Option<u16>,
    pub stand_count: Option<u16>,
    pub min_heart_rate: Option<u8>,
    pub sub_sport: Option<u8>,
    pub nec_long: Option<i32>,
    pub avg_total_hemoglobin_conc: Option<u16>,
    pub avg_power_position: Option<u16>,
    pub sport_index: Option<u8>,
    pub avg_fractional_cadence: Option<u8>,
    pub normalized_power: Option<u16>,
    pub avg_left_torque_effectiveness: Option<u8>,
    pub avg_left_pedal_smoothness: Option<u8>,
    pub total_descent: Option<u16>,
    pub total_grit: Option<f32>,
    pub total_flow: Option<f32>,
    pub left_right_balance: Option<u16>,
    pub start_position_lat: Option<i32>,
    pub opponent_name: Option<String>,
    pub max_temperature: Option<i8>,
    pub lev_battery_consumption: Option<u8>,
    pub avg_step_length: Option<u16>,
    pub num_laps: Option<u16>,
    pub avg_grade: Option<i16>,
    pub avg_stroke_count: Option<u32>,
    pub avg_left_power_phase_peak: Option<u8>,
    pub max_power_position: Option<u16>,
    pub total_fat_calories: Option<u16>,
    pub pool_length: Option<u16>,
    pub total_ascent: Option<u16>,
    pub gps_accuracy: Option<u8>,
    pub avg_lap_time: Option<u32>,
    pub max_altitude: Option<u16>,
    pub time_standing: Option<u32>,
    pub avg_left_pco: Option<i8>,
    pub time_in_power_zone: Option<u32>,
    pub time_in_speed_zone: Option<u32>,
    pub avg_power: Option<u16>,
    pub opponent_score: Option<u16>,
    pub avg_combined_pedal_smoothness: Option<u8>,
    pub avg_heart_rate: Option<u8>,
    pub enhanced_min_altitude: Option<u32>,
    pub total_anaerobic_training_effect: Option<u8>,
    pub threshold_power: Option<u16>,
    pub start_time: Option<u32>,
    pub avg_vertical_oscillation: Option<u16>,
    pub avg_saturated_hemoglobin_percent: Option<u16>,
    pub avg_right_pco: Option<i8>,
    pub sport: Option<u8>,
    pub avg_temperature: Option<i8>,
    pub avg_pos_vertical_speed: Option<i16>,
    pub message_index: Option<u16>,
    pub player_score: Option<u16>,
    pub avg_stance_time_percent: Option<u16>,
    pub avg_stroke_distance: Option<u16>,
    pub avg_right_power_phase: Option<u8>,
    pub avg_speed: Option<u16>,
    pub avg_vam: Option<u16>,
    pub max_heart_rate: Option<u8>,
    pub event: Option<u8>,
    pub avg_grit: Option<f32>,
}

impl FitSessionMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FitFieldValue>) -> Self {
        let mut msg = FitSessionMsg { total_cycles: None, 
            num_lengths: None, total_distance: None, avg_stance_time: None, 
            avg_right_pedal_smoothness: None, total_moving_time: None, avg_vertical_ratio: None, 
            best_lap_index: None, timestamp: None, avg_altitude: None, 
            swim_stroke: None, total_fractional_descent: None, max_neg_vertical_speed: None, 
            max_fractional_cadence: None, intensity_factor: None, max_ball_speed: None, 
            avg_lev_motor_power: None, jump_count: None, max_power: None, 
            num_active_lengths: None, max_neg_grade: None, training_stress_score: None, 
            enhanced_max_altitude: None, max_cadence_position: None, total_calories: None, 
            avg_pos_grade: None, time_in_cadence_zone: None, zone_count: None, 
            enhanced_max_speed: None, avg_cadence: None, total_fractional_ascent: None, 
            total_elapsed_time: None, swc_long: None, max_pos_vertical_speed: None, 
            avg_stance_time_balance: None, max_saturated_hemoglobin_percent: None, event_type: None, 
            first_lap_index: None, enhanced_avg_speed: None, avg_flow: None, 
            time_in_hr_zone: None, pool_length_unit: None, max_cadence: None, 
            event_group: None, avg_cadence_position: None, start_position_long: None, 
            total_timer_time: None, trigger: None, max_speed: None, 
            nec_lat: None, total_work: None, min_total_hemoglobin_conc: None, 
            min_saturated_hemoglobin_percent: None, max_lev_motor_power: None, swc_lat: None, 
            avg_left_power_phase: None, avg_ball_speed: None, stroke_count: None, 
            total_fractional_cycles: None, enhanced_avg_altitude: None, total_training_effect: None, 
            avg_neg_grade: None, min_altitude: None, max_pos_grade: None, 
            avg_right_power_phase_peak: None, avg_right_torque_effectiveness: None, avg_neg_vertical_speed: None, 
            max_total_hemoglobin_conc: None, stand_count: None, min_heart_rate: None, 
            sub_sport: None, nec_long: None, avg_total_hemoglobin_conc: None, 
            avg_power_position: None, sport_index: None, avg_fractional_cadence: None, 
            normalized_power: None, avg_left_torque_effectiveness: None, avg_left_pedal_smoothness: None, 
            total_descent: None, total_grit: None, total_flow: None, 
            left_right_balance: None, start_position_lat: None, opponent_name: None, 
            max_temperature: None, lev_battery_consumption: None, avg_step_length: None, 
            num_laps: None, avg_grade: None, avg_stroke_count: None, 
            avg_left_power_phase_peak: None, max_power_position: None, total_fat_calories: None, 
            pool_length: None, total_ascent: None, gps_accuracy: None, 
            avg_lap_time: None, max_altitude: None, time_standing: None, 
            avg_left_pco: None, time_in_power_zone: None, time_in_speed_zone: None, 
            avg_power: None, opponent_score: None, avg_combined_pedal_smoothness: None, 
            avg_heart_rate: None, enhanced_min_altitude: None, total_anaerobic_training_effect: None, 
            threshold_power: None, start_time: None, avg_vertical_oscillation: None, 
            avg_saturated_hemoglobin_percent: None, avg_right_pco: None, sport: None, 
            avg_temperature: None, avg_pos_vertical_speed: None, message_index: None, 
            player_score: None, avg_stance_time_percent: None, avg_stroke_distance: None, 
            avg_right_power_phase: None, avg_speed: None, avg_vam: None, 
            max_heart_rate: None, event: None, avg_grit: None, 
            
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    10 => { msg.total_cycles = Some(field.get_u32()); },
                    33 => { msg.num_lengths = Some(field.get_u16()); },
                    9 => { msg.total_distance = Some(field.get_u32()); },
                    91 => { msg.avg_stance_time = Some(field.get_u16()); },
                    104 => { msg.avg_right_pedal_smoothness = Some(field.get_u8()); },
                    59 => { msg.total_moving_time = Some(field.get_u32()); },
                    132 => { msg.avg_vertical_ratio = Some(field.get_u16()); },
                    70 => { msg.best_lap_index = Some(field.get_u16()); },
                    253 => { msg.timestamp = Some(field.get_u32()); },
                    49 => { msg.avg_altitude = Some(field.get_u16()); },
                    43 => { msg.swim_stroke = Some(field.get_u8()); },
                    200 => { msg.total_fractional_descent = Some(field.get_u8()); },
                    63 => { msg.max_neg_vertical_speed = Some(field.get_i16()); },
                    93 => { msg.max_fractional_cadence = Some(field.get_u8()); },
                    36 => { msg.intensity_factor = Some(field.get_u16()); },
                    87 => { msg.max_ball_speed = Some(field.get_u16()); },
                    129 => { msg.avg_lev_motor_power = Some(field.get_u16()); },
                    183 => { msg.jump_count = Some(field.get_u16()); },
                    21 => { msg.max_power = Some(field.get_u16()); },
                    47 => { msg.num_active_lengths = Some(field.get_u16()); },
                    56 => { msg.max_neg_grade = Some(field.get_i16()); },
                    35 => { msg.training_stress_score = Some(field.get_u16()); },
                    128 => { msg.enhanced_max_altitude = Some(field.get_u32()); },
                    123 => { msg.max_cadence_position = Some(field.get_u8()); },
                    11 => { msg.total_calories = Some(field.get_u16()); },
                    53 => { msg.avg_pos_grade = Some(field.get_i16()); },
                    67 => { msg.time_in_cadence_zone = Some(field.get_u32()); },
                    86 => { msg.zone_count = Some(field.get_u16()); },
                    125 => { msg.enhanced_max_speed = Some(field.get_u32()); },
                    18 => { msg.avg_cadence = Some(field.get_u8()); },
                    199 => { msg.total_fractional_ascent = Some(field.get_u8()); },
                    7 => { msg.total_elapsed_time = Some(field.get_u32()); },
                    32 => { msg.swc_long = Some(field.get_i32()); },
                    62 => { msg.max_pos_vertical_speed = Some(field.get_i16()); },
                    133 => { msg.avg_stance_time_balance = Some(field.get_u16()); },
                    100 => { msg.max_saturated_hemoglobin_percent = Some(field.get_u16()); },
                    1 => { msg.event_type = Some(field.get_u8()); },
                    25 => { msg.first_lap_index = Some(field.get_u16()); },
                    124 => { msg.enhanced_avg_speed = Some(field.get_u32()); },
                    187 => { msg.avg_flow = Some(field.get_f32()); },
                    65 => { msg.time_in_hr_zone = Some(field.get_u32()); },
                    46 => { msg.pool_length_unit = Some(field.get_u8()); },
                    19 => { msg.max_cadence = Some(field.get_u8()); },
                    27 => { msg.event_group = Some(field.get_u8()); },
                    122 => { msg.avg_cadence_position = Some(field.get_u8()); },
                    4 => { msg.start_position_long = Some(field.get_i32()); },
                    8 => { msg.total_timer_time = Some(field.get_u32()); },
                    28 => { msg.trigger = Some(field.get_u8()); },
                    15 => { msg.max_speed = Some(field.get_u16()); },
                    29 => { msg.nec_lat = Some(field.get_i32()); },
                    48 => { msg.total_work = Some(field.get_u32()); },
                    96 => { msg.min_total_hemoglobin_conc = Some(field.get_u16()); },
                    99 => { msg.min_saturated_hemoglobin_percent = Some(field.get_u16()); },
                    130 => { msg.max_lev_motor_power = Some(field.get_u16()); },
                    31 => { msg.swc_lat = Some(field.get_i32()); },
                    116 => { msg.avg_left_power_phase = Some(field.get_u8()); },
                    88 => { msg.avg_ball_speed = Some(field.get_u16()); },
                    85 => { msg.stroke_count = Some(field.get_u16()); },
                    94 => { msg.total_fractional_cycles = Some(field.get_u8()); },
                    126 => { msg.enhanced_avg_altitude = Some(field.get_u32()); },
                    24 => { msg.total_training_effect = Some(field.get_u8()); },
                    54 => { msg.avg_neg_grade = Some(field.get_i16()); },
                    71 => { msg.min_altitude = Some(field.get_u16()); },
                    55 => { msg.max_pos_grade = Some(field.get_i16()); },
                    119 => { msg.avg_right_power_phase_peak = Some(field.get_u8()); },
                    102 => { msg.avg_right_torque_effectiveness = Some(field.get_u8()); },
                    61 => { msg.avg_neg_vertical_speed = Some(field.get_i16()); },
                    97 => { msg.max_total_hemoglobin_conc = Some(field.get_u16()); },
                    113 => { msg.stand_count = Some(field.get_u16()); },
                    64 => { msg.min_heart_rate = Some(field.get_u8()); },
                    6 => { msg.sub_sport = Some(field.get_u8()); },
                    30 => { msg.nec_long = Some(field.get_i32()); },
                    95 => { msg.avg_total_hemoglobin_conc = Some(field.get_u16()); },
                    120 => { msg.avg_power_position = Some(field.get_u16()); },
                    111 => { msg.sport_index = Some(field.get_u8()); },
                    92 => { msg.avg_fractional_cadence = Some(field.get_u8()); },
                    34 => { msg.normalized_power = Some(field.get_u16()); },
                    101 => { msg.avg_left_torque_effectiveness = Some(field.get_u8()); },
                    103 => { msg.avg_left_pedal_smoothness = Some(field.get_u8()); },
                    23 => { msg.total_descent = Some(field.get_u16()); },
                    181 => { msg.total_grit = Some(field.get_f32()); },
                    182 => { msg.total_flow = Some(field.get_f32()); },
                    37 => { msg.left_right_balance = Some(field.get_u16()); },
                    3 => { msg.start_position_lat = Some(field.get_i32()); },
                    84 => { msg.opponent_name = Some(field.value_string); },
                    58 => { msg.max_temperature = Some(field.get_i8()); },
                    131 => { msg.lev_battery_consumption = Some(field.get_u8()); },
                    134 => { msg.avg_step_length = Some(field.get_u16()); },
                    26 => { msg.num_laps = Some(field.get_u16()); },
                    52 => { msg.avg_grade = Some(field.get_i16()); },
                    41 => { msg.avg_stroke_count = Some(field.get_u32()); },
                    117 => { msg.avg_left_power_phase_peak = Some(field.get_u8()); },
                    121 => { msg.max_power_position = Some(field.get_u16()); },
                    13 => { msg.total_fat_calories = Some(field.get_u16()); },
                    44 => { msg.pool_length = Some(field.get_u16()); },
                    22 => { msg.total_ascent = Some(field.get_u16()); },
                    51 => { msg.gps_accuracy = Some(field.get_u8()); },
                    69 => { msg.avg_lap_time = Some(field.get_u32()); },
                    50 => { msg.max_altitude = Some(field.get_u16()); },
                    112 => { msg.time_standing = Some(field.get_u32()); },
                    114 => { msg.avg_left_pco = Some(field.get_i8()); },
                    68 => { msg.time_in_power_zone = Some(field.get_u32()); },
                    66 => { msg.time_in_speed_zone = Some(field.get_u32()); },
                    20 => { msg.avg_power = Some(field.get_u16()); },
                    83 => { msg.opponent_score = Some(field.get_u16()); },
                    105 => { msg.avg_combined_pedal_smoothness = Some(field.get_u8()); },
                    16 => { msg.avg_heart_rate = Some(field.get_u8()); },
                    127 => { msg.enhanced_min_altitude = Some(field.get_u32()); },
                    137 => { msg.total_anaerobic_training_effect = Some(field.get_u8()); },
                    45 => { msg.threshold_power = Some(field.get_u16()); },
                    2 => { msg.start_time = Some(field.get_u32()); },
                    89 => { msg.avg_vertical_oscillation = Some(field.get_u16()); },
                    98 => { msg.avg_saturated_hemoglobin_percent = Some(field.get_u16()); },
                    115 => { msg.avg_right_pco = Some(field.get_i8()); },
                    5 => { msg.sport = Some(field.get_u8()); },
                    57 => { msg.avg_temperature = Some(field.get_i8()); },
                    60 => { msg.avg_pos_vertical_speed = Some(field.get_i16()); },
                    254 => { msg.message_index = Some(field.get_u16()); },
                    82 => { msg.player_score = Some(field.get_u16()); },
                    90 => { msg.avg_stance_time_percent = Some(field.get_u16()); },
                    42 => { msg.avg_stroke_distance = Some(field.get_u16()); },
                    118 => { msg.avg_right_power_phase = Some(field.get_u8()); },
                    14 => { msg.avg_speed = Some(field.get_u16()); },
                    139 => { msg.avg_vam = Some(field.get_u16()); },
                    17 => { msg.max_heart_rate = Some(field.get_u8()); },
                    0 => { msg.event = Some(field.get_u8()); },
                    186 => { msg.avg_grit = Some(field.get_f32()); },
                    _ => { /* panic!("Session field not implemented {:#x}", field.field_def); */ }
                }
            }
        }
        msg
    }
}

// Auto-generated by print_message_struct in lib.rs
pub struct FitDeviceInfoMsg {
    pub battery_voltage: Option<u16>,
    pub cum_operating_time: Option<u32>,
    pub serial_number: Option<u32>,
    pub product: Option<u16>,
    pub timestamp: Option<u32>,
    pub sensor_position: Option<u8>,
    pub source_type: Option<u8>,
    pub software_version: Option<u16>,
    pub ant_transmission_type: Option<u8>,
    pub ant_device_number: Option<u16>,
    pub descriptor: Option<String>,
    pub device_type: Option<u8>,
    pub ant_network: Option<u8>,
    pub product_name: Option<String>,
    pub device_index: Option<u8>,
    pub hardware_version: Option<u8>,
    pub battery_status: Option<u8>,
    pub manufacturer: Option<u16>,
}

impl FitDeviceInfoMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FitFieldValue>) -> Self {
        let mut msg = FitDeviceInfoMsg { battery_voltage: None, 
            cum_operating_time: None, serial_number: None, product: None,
            timestamp: None, sensor_position: None, source_type: None, 
            software_version: None, ant_transmission_type: None, ant_device_number: None, 
            descriptor: None, device_type: None, ant_network: None, 
            product_name: None, device_index: None, hardware_version: None, 
            battery_status: None, manufacturer: None, 
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    10 => { msg.battery_voltage = Some(field.get_u16()); },
                    7 => { msg.cum_operating_time = Some(field.get_u32()); },
                    3 => { msg.serial_number = Some(field.get_u32()); },
                    4 => { msg.product = Some(field.get_u16()); },
                    253 => { msg.timestamp = Some(field.get_u32()); },
                    18 => { msg.sensor_position = Some(field.get_u8()); },
                    25 => { msg.source_type = Some(field.get_u8()); },
                    5 => { msg.software_version = Some(field.get_u16()); },
                    20 => { msg.ant_transmission_type = Some(field.get_u8()); },
                    21 => { msg.ant_device_number = Some(field.get_u16()); },
                    19 => { msg.descriptor = Some(field.value_string); },
                    1 => { msg.device_type = Some(field.get_u8()); },
                    22 => { msg.ant_network = Some(field.get_u8()); },
                    27 => { msg.product_name = Some(field.value_string); },
                    0 => { msg.device_index = Some(field.get_u8()); },
                    6 => { msg.hardware_version = Some(field.get_u8()); },
                    11 => { msg.battery_status = Some(field.get_u8()); },
                    2 => { msg.manufacturer = Some(field.get_u16()); },
                    _ => { /* panic!("Device Info field not implemented {:#x}", field.field_def); */ }
                }
            }
        }
        msg
    }
}

// Auto-generated by print_message_struct in lib.rs
pub struct FitLapMsg {
    pub opponent_score: Option<u16>,
    pub avg_vertical_ratio: Option<u16>,
    pub avg_saturated_hemoglobin_percent: Option<u16>,
    pub num_lengths: Option<u16>,
    pub max_temperature: Option<i8>,
    pub sport: Option<u8>,
    pub min_altitude: Option<u16>,
    pub avg_right_torque_effectiveness: Option<u8>,
    pub normalized_power: Option<u16>,
    pub time_in_hr_zone: Option<u32>,
    pub left_right_balance: Option<u16>,
    pub avg_right_power_phase: Option<u8>,
    pub event_group: Option<u8>,
    pub avg_left_power_phase: Option<u8>,
    pub avg_cadence: Option<u8>,
    pub start_time: Option<u32>,
    pub avg_power: Option<u16>,
    pub enhanced_min_altitude: Option<u32>,
    pub avg_neg_vertical_speed: Option<i16>,
    pub total_elapsed_time: Option<u32>,
    pub max_pos_grade: Option<i16>,
    pub max_cadence_position: Option<u8>,
    pub event_type: Option<u8>,
    pub start_position_long: Option<i32>,
    pub total_descent: Option<u16>,
    pub min_heart_rate: Option<u8>,
    pub wkt_step_index: Option<u16>,
    pub enhanced_avg_altitude: Option<u32>,
    pub max_lev_motor_power: Option<u16>,
    pub num_active_lengths: Option<u16>,
    pub sub_sport: Option<u8>,
    pub time_standing: Option<u32>,
    pub first_length_index: Option<u16>,
    pub total_cycles: Option<u32>,
    pub avg_cadence_position: Option<u8>,
    pub lev_battery_consumption: Option<u8>,
    pub avg_temperature: Option<i8>,
    pub max_pos_vertical_speed: Option<i16>,
    pub timestamp: Option<u32>,
    pub enhanced_max_speed: Option<u32>,
    pub zone_count: Option<u16>,
    pub avg_pos_grade: Option<i16>,
    pub avg_lev_motor_power: Option<u16>,
    pub total_timer_time: Option<u32>,
    pub avg_pos_vertical_speed: Option<i16>,
    pub total_work: Option<u32>,
    pub time_in_speed_zone: Option<u32>,
    pub stroke_count: Option<u16>,
    pub avg_vertical_oscillation: Option<u16>,
    pub avg_fractional_cadence: Option<u8>,
    pub jump_count: Option<u16>,
    pub event: Option<u8>,
    pub avg_step_length: Option<u16>,
    pub total_fractional_descent: Option<u8>,
    pub avg_combined_pedal_smoothness: Option<u8>,
    pub max_power_position: Option<u16>,
    pub total_flow: Option<f32>,
    pub avg_stroke_distance: Option<u16>,
    pub max_neg_grade: Option<i16>,
    pub swim_stroke: Option<u8>,
    pub avg_left_torque_effectiveness: Option<u8>,
    pub enhanced_max_altitude: Option<u32>,
    pub avg_vam: Option<u16>,
    pub avg_right_pedal_smoothness: Option<u8>,
    pub avg_stance_time_balance: Option<u16>,
    pub avg_neg_grade: Option<i16>,
    pub lap_trigger: Option<u8>,
    pub max_saturated_hemoglobin_percent: Option<u16>,
    pub max_heart_rate: Option<u8>,
    pub total_fractional_cycles: Option<u8>,
    pub avg_left_power_phase_peak: Option<u8>,
    pub total_moving_time: Option<u32>,
    pub avg_stance_time: Option<u16>,
    pub total_calories: Option<u16>,
    pub max_speed: Option<u16>,
    pub enhanced_avg_speed: Option<u32>,
    pub stand_count: Option<u16>,
    pub max_altitude: Option<u16>,
    pub avg_heart_rate: Option<u8>,
    pub avg_altitude: Option<u16>,
    pub total_fractional_ascent: Option<u8>,
    pub end_position_lat: Option<i32>,
    pub total_distance: Option<u32>,
    pub avg_power_position: Option<u16>,
    pub time_in_power_zone: Option<u32>,
    pub max_total_hemoglobin_conc: Option<u16>,
    pub avg_stance_time_percent: Option<u16>,
    pub start_position_lat: Option<i32>,
    pub gps_accuracy: Option<u8>,
    pub repetition_num: Option<u16>,
    pub message_index: Option<u16>,
    pub max_power: Option<u16>,
    pub player_score: Option<u16>,
    pub total_fat_calories: Option<u16>,
    pub avg_left_pco: Option<i8>,
    pub avg_right_pco: Option<i8>,
    pub avg_right_power_phase_peak: Option<u8>,
    pub end_position_long: Option<i32>,
    pub avg_left_pedal_smoothness: Option<u8>,
    pub avg_speed: Option<u16>,
    pub avg_total_hemoglobin_conc: Option<u16>,
    pub total_grit: Option<f32>,
    pub intensity: Option<u8>,
    pub avg_grit: Option<f32>,
    pub max_fractional_cadence: Option<u8>,
    pub min_total_hemoglobin_conc: Option<u16>,
    pub avg_flow: Option<f32>,
    pub max_cadence: Option<u8>,
    pub avg_grade: Option<i16>,
    pub min_saturated_hemoglobin_percent: Option<u16>,
    pub total_ascent: Option<u16>,
    pub max_neg_vertical_speed: Option<i16>,
    pub time_in_cadence_zone: Option<u32>,
}

impl FitLapMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FitFieldValue>) -> Self {
        let mut msg = FitLapMsg { opponent_score: None, 
            avg_vertical_ratio: None, avg_saturated_hemoglobin_percent: None, num_lengths: None, 
            max_temperature: None, sport: None, min_altitude: None, 
            avg_right_torque_effectiveness: None, normalized_power: None, time_in_hr_zone: None, 
            left_right_balance: None, avg_right_power_phase: None, event_group: None, 
            avg_left_power_phase: None, avg_cadence: None, start_time: None, 
            avg_power: None, enhanced_min_altitude: None, avg_neg_vertical_speed: None, 
            total_elapsed_time: None, max_pos_grade: None, max_cadence_position: None, 
            event_type: None, start_position_long: None, total_descent: None, 
            min_heart_rate: None, wkt_step_index: None, enhanced_avg_altitude: None, 
            max_lev_motor_power: None, num_active_lengths: None, sub_sport: None, 
            time_standing: None, first_length_index: None, total_cycles: None, 
            avg_cadence_position: None, lev_battery_consumption: None, avg_temperature: None, 
            max_pos_vertical_speed: None, timestamp: None, enhanced_max_speed: None, 
            zone_count: None, avg_pos_grade: None, avg_lev_motor_power: None, 
            total_timer_time: None, avg_pos_vertical_speed: None, total_work: None, 
            time_in_speed_zone: None, stroke_count: None, avg_vertical_oscillation: None, 
            avg_fractional_cadence: None, jump_count: None, event: None, 
            avg_step_length: None, total_fractional_descent: None, avg_combined_pedal_smoothness: None, 
            max_power_position: None, total_flow: None, avg_stroke_distance: None, 
            max_neg_grade: None, swim_stroke: None, avg_left_torque_effectiveness: None, 
            enhanced_max_altitude: None, avg_vam: None, avg_right_pedal_smoothness: None, 
            avg_stance_time_balance: None, avg_neg_grade: None, lap_trigger: None, 
            max_saturated_hemoglobin_percent: None, max_heart_rate: None, total_fractional_cycles: None, 
            avg_left_power_phase_peak: None, total_moving_time: None, avg_stance_time: None, 
            total_calories: None, max_speed: None, enhanced_avg_speed: None, 
            stand_count: None, max_altitude: None, avg_heart_rate: None, 
            avg_altitude: None, total_fractional_ascent: None, end_position_lat: None, 
            total_distance: None, avg_power_position: None, time_in_power_zone: None, 
            max_total_hemoglobin_conc: None, avg_stance_time_percent: None, start_position_lat: None, 
            gps_accuracy: None, repetition_num: None, message_index: None, 
            max_power: None, player_score: None, total_fat_calories: None, 
            avg_left_pco: None, avg_right_pco: None, avg_right_power_phase_peak: None, 
            end_position_long: None, avg_left_pedal_smoothness: None, avg_speed: None, 
            avg_total_hemoglobin_conc: None, total_grit: None, intensity: None, 
            avg_grit: None, max_fractional_cadence: None, min_total_hemoglobin_conc: None, 
            avg_flow: None, max_cadence: None, avg_grade: None, 
            min_saturated_hemoglobin_percent: None, total_ascent: None, max_neg_vertical_speed: None, 
            time_in_cadence_zone: None, 
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    74 => { msg.opponent_score = Some(field.get_u16()); },
                    118 => { msg.avg_vertical_ratio = Some(field.get_u16()); },
                    87 => { msg.avg_saturated_hemoglobin_percent = Some(field.get_u16()); },
                    32 => { msg.num_lengths = Some(field.get_u16()); },
                    51 => { msg.max_temperature = Some(field.get_i8()); },
                    25 => { msg.sport = Some(field.get_u8()); },
                    62 => { msg.min_altitude = Some(field.get_u16()); },
                    92 => { msg.avg_right_torque_effectiveness = Some(field.get_u8()); },
                    33 => { msg.normalized_power = Some(field.get_u16()); },
                    57 => { msg.time_in_hr_zone = Some(field.get_u32()); },
                    34 => { msg.left_right_balance = Some(field.get_u16()); },
                    104 => { msg.avg_right_power_phase = Some(field.get_u8()); },
                    26 => { msg.event_group = Some(field.get_u8()); },
                    102 => { msg.avg_left_power_phase = Some(field.get_u8()); },
                    17 => { msg.avg_cadence = Some(field.get_u8()); },
                    2 => { msg.start_time = Some(field.get_u32()); },
                    19 => { msg.avg_power = Some(field.get_u16()); },
                    113 => { msg.enhanced_min_altitude = Some(field.get_u32()); },
                    54 => { msg.avg_neg_vertical_speed = Some(field.get_i16()); },
                    7 => { msg.total_elapsed_time = Some(field.get_u32()); },
                    48 => { msg.max_pos_grade = Some(field.get_i16()); },
                    109 => { msg.max_cadence_position = Some(field.get_u8()); },
                    1 => { msg.event_type = Some(field.get_u8()); },
                    4 => { msg.start_position_long = Some(field.get_i32()); },
                    22 => { msg.total_descent = Some(field.get_u16()); },
                    63 => { msg.min_heart_rate = Some(field.get_u8()); },
                    71 => { msg.wkt_step_index = Some(field.get_u16()); },
                    112 => { msg.enhanced_avg_altitude = Some(field.get_u32()); },
                    116 => { msg.max_lev_motor_power = Some(field.get_u16()); },
                    40 => { msg.num_active_lengths = Some(field.get_u16()); },
                    39 => { msg.sub_sport = Some(field.get_u8()); },
                    98 => { msg.time_standing = Some(field.get_u32()); },
                    35 => { msg.first_length_index = Some(field.get_u16()); },
                    10 => { msg.total_cycles = Some(field.get_u32()); },
                    108 => { msg.avg_cadence_position = Some(field.get_u8()); },
                    117 => { msg.lev_battery_consumption = Some(field.get_u8()); },
                    50 => { msg.avg_temperature = Some(field.get_i8()); },
                    55 => { msg.max_pos_vertical_speed = Some(field.get_i16()); },
                    253 => { msg.timestamp = Some(field.get_u32()); },
                    111 => { msg.enhanced_max_speed = Some(field.get_u32()); },
                    76 => { msg.zone_count = Some(field.get_u16()); },
                    46 => { msg.avg_pos_grade = Some(field.get_i16()); },
                    115 => { msg.avg_lev_motor_power = Some(field.get_u16()); },
                    8 => { msg.total_timer_time = Some(field.get_u32()); },
                    53 => { msg.avg_pos_vertical_speed = Some(field.get_i16()); },
                    41 => { msg.total_work = Some(field.get_u32()); },
                    58 => { msg.time_in_speed_zone = Some(field.get_u32()); },
                    75 => { msg.stroke_count = Some(field.get_u16()); },
                    77 => { msg.avg_vertical_oscillation = Some(field.get_u16()); },
                    80 => { msg.avg_fractional_cadence = Some(field.get_u8()); },
                    151 => { msg.jump_count = Some(field.get_u16()); },
                    0 => { msg.event = Some(field.get_u8()); },
                    120 => { msg.avg_step_length = Some(field.get_u16()); },
                    157 => { msg.total_fractional_descent = Some(field.get_u8()); },
                    95 => { msg.avg_combined_pedal_smoothness = Some(field.get_u8()); },
                    107 => { msg.max_power_position = Some(field.get_u16()); },
                    150 => { msg.total_flow = Some(field.get_f32()); },
                    37 => { msg.avg_stroke_distance = Some(field.get_u16()); },
                    49 => { msg.max_neg_grade = Some(field.get_i16()); },
                    38 => { msg.swim_stroke = Some(field.get_u8()); },
                    91 => { msg.avg_left_torque_effectiveness = Some(field.get_u8()); },
                    114 => { msg.enhanced_max_altitude = Some(field.get_u32()); },
                    121 => { msg.avg_vam = Some(field.get_u16()); },
                    94 => { msg.avg_right_pedal_smoothness = Some(field.get_u8()); },
                    119 => { msg.avg_stance_time_balance = Some(field.get_u16()); },
                    47 => { msg.avg_neg_grade = Some(field.get_i16()); },
                    24 => { msg.lap_trigger = Some(field.get_u8()); },
                    89 => { msg.max_saturated_hemoglobin_percent = Some(field.get_u16()); },
                    16 => { msg.max_heart_rate = Some(field.get_u8()); },
                    82 => { msg.total_fractional_cycles = Some(field.get_u8()); },
                    103 => { msg.avg_left_power_phase_peak = Some(field.get_u8()); },
                    52 => { msg.total_moving_time = Some(field.get_u32()); },
                    79 => { msg.avg_stance_time = Some(field.get_u16()); },
                    11 => { msg.total_calories = Some(field.get_u16()); },
                    14 => { msg.max_speed = Some(field.get_u16()); },
                    110 => { msg.enhanced_avg_speed = Some(field.get_u32()); },
                    99 => { msg.stand_count = Some(field.get_u16()); },
                    43 => { msg.max_altitude = Some(field.get_u16()); },
                    15 => { msg.avg_heart_rate = Some(field.get_u8()); },
                    42 => { msg.avg_altitude = Some(field.get_u16()); },
                    156 => { msg.total_fractional_ascent = Some(field.get_u8()); },
                    5 => { msg.end_position_lat = Some(field.get_i32()); },
                    9 => { msg.total_distance = Some(field.get_u32()); },
                    106 => { msg.avg_power_position = Some(field.get_u16()); },
                    60 => { msg.time_in_power_zone = Some(field.get_u32()); },
                    86 => { msg.max_total_hemoglobin_conc = Some(field.get_u16()); },
                    78 => { msg.avg_stance_time_percent = Some(field.get_u16()); },
                    3 => { msg.start_position_lat = Some(field.get_i32()); },
                    44 => { msg.gps_accuracy = Some(field.get_u8()); },
                    61 => { msg.repetition_num = Some(field.get_u16()); },
                    254 => { msg.message_index = Some(field.get_u16()); },
                    20 => { msg.max_power = Some(field.get_u16()); },
                    83 => { msg.player_score = Some(field.get_u16()); },
                    12 => { msg.total_fat_calories = Some(field.get_u16()); },
                    100 => { msg.avg_left_pco = Some(field.get_i8()); },
                    101 => { msg.avg_right_pco = Some(field.get_i8()); },
                    105 => { msg.avg_right_power_phase_peak = Some(field.get_u8()); },
                    6 => { msg.end_position_long = Some(field.get_i32()); },
                    93 => { msg.avg_left_pedal_smoothness = Some(field.get_u8()); },
                    13 => { msg.avg_speed = Some(field.get_u16()); },
                    84 => { msg.avg_total_hemoglobin_conc = Some(field.get_u16()); },
                    149 => { msg.total_grit = Some(field.get_f32()); },
                    23 => { msg.intensity = Some(field.get_u8()); },
                    153 => { msg.avg_grit = Some(field.get_f32()); },
                    81 => { msg.max_fractional_cadence = Some(field.get_u8()); },
                    85 => { msg.min_total_hemoglobin_conc = Some(field.get_u16()); },
                    154 => { msg.avg_flow = Some(field.get_f32()); },
                    18 => { msg.max_cadence = Some(field.get_u8()); },
                    45 => { msg.avg_grade = Some(field.get_i16()); },
                    88 => { msg.min_saturated_hemoglobin_percent = Some(field.get_u16()); },
                    21 => { msg.total_ascent = Some(field.get_u16()); },
                    56 => { msg.max_neg_vertical_speed = Some(field.get_i16()); },
                    59 => { msg.time_in_cadence_zone = Some(field.get_u32()); },
                    _ => { /* panic!("Lap field not implemented {:#x}", field.field_def); */ }
                }
            }
        }
        msg
    }
}

// Auto-generated by print_message_struct in lib.rs
pub struct FitLengthMsg {
    pub opponent_score: Option<u16>,
    pub stroke_count: Option<u16>,
    pub zone_count: Option<u16>,
    pub length_type: Option<u8>,
    pub total_elapsed_time: Option<u32>,
    pub player_score: Option<u16>,
    pub timestamp: Option<u32>,
    pub swim_stroke: Option<u8>,
    pub total_timer_time: Option<u32>,
    pub total_calories: Option<u16>,
    pub start_time: Option<u32>,
    pub message_index: Option<u16>,
    pub event: Option<u8>,
    pub total_strokes: Option<u16>,
    pub event_type: Option<u8>,
    pub avg_swimming_cadence: Option<u8>,
    pub event_group: Option<u8>,
    pub avg_speed: Option<u16>,
}

impl FitLengthMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FitFieldValue>) -> Self {
        let mut msg = FitLengthMsg { opponent_score: None, 
            stroke_count: None, zone_count: None, length_type: None, 
            total_elapsed_time: None, player_score: None, timestamp: None, 
            swim_stroke: None, total_timer_time: None, total_calories: None, 
            start_time: None, message_index: None, event: None, 
            total_strokes: None, event_type: None, avg_swimming_cadence: None, 
            event_group: None, avg_speed: None, 
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    19 => { msg.opponent_score = Some(field.get_u16()); },
                    20 => { msg.stroke_count = Some(field.get_u16()); },
                    21 => { msg.zone_count = Some(field.get_u16()); },
                    12 => { msg.length_type = Some(field.get_u8()); },
                    3 => { msg.total_elapsed_time = Some(field.get_u32()); },
                    18 => { msg.player_score = Some(field.get_u16()); },
                    253 => { msg.timestamp = Some(field.get_u32()); },
                    7 => { msg.swim_stroke = Some(field.get_u8()); },
                    4 => { msg.total_timer_time = Some(field.get_u32()); },
                    11 => { msg.total_calories = Some(field.get_u16()); },
                    2 => { msg.start_time = Some(field.get_u32()); },
                    254 => { msg.message_index = Some(field.get_u16()); },
                    0 => { msg.event = Some(field.get_u8()); },
                    5 => { msg.total_strokes = Some(field.get_u16()); },
                    1 => { msg.event_type = Some(field.get_u8()); },
                    9 => { msg.avg_swimming_cadence = Some(field.get_u8()); },
                    10 => { msg.event_group = Some(field.get_u8()); },
                    6 => { msg.avg_speed = Some(field.get_u16()); },
                    _ => { /* panic!("Length field not implemented {:#x}", field.field_def); */ }
                }
            }
        }
        msg
    }
}

// Auto-generated by print_message_struct in lib.rs
pub struct FitRecordMsg {
    pub step_length: Option<u16>,
    pub resistance: Option<u8>,
    pub speed: Option<u16>,
    pub accumulated_power: Option<u32>,
    pub next_stop_depth: Option<u32>,
    pub stroke_type: Option<u8>,
    pub heart_rate: Option<u8>,
    pub cycles: Option<u8>,
    pub total_hemoglobin_conc: Option<u16>,
    pub time_from_course: Option<i32>,
    pub saturated_hemoglobin_percent: Option<u16>,
    pub cadence256: Option<u16>,
    pub ndl_time: Option<u32>,
    pub next_stop_time: Option<u32>,
    pub time128: Option<u8>,
    pub left_right_balance: Option<u8>,
    pub temperature: Option<i8>,
    pub position_long: Option<i32>,
    pub motor_power: Option<u16>,
    pub vertical_ratio: Option<u16>,
    pub grit: Option<f32>,
    pub compressed_accumulated_power: Option<u16>,
    pub position_lat: Option<i32>,
    pub ebike_assist_mode: Option<u8>,
    pub n2_load: Option<u16>,
    pub grade: Option<i16>,
    pub left_power_phase: Option<u8>,
    pub power: Option<u16>,
    pub timestamp: Option<u32>,
    pub vertical_oscillation: Option<u16>,
    pub fractional_cadence: Option<u8>,
    pub saturated_hemoglobin_percent_min: Option<u16>,
    pub cycle_length: Option<u8>,
    pub right_pedal_smoothness: Option<u8>,
    pub total_hemoglobin_conc_max: Option<u16>,
    pub stance_time_percent: Option<u16>,
    pub enhanced_altitude: Option<u32>,
    pub stance_time_balance: Option<u16>,
    pub speed_1s: Option<u8>,
    pub battery_soc: Option<u8>,
    pub total_hemoglobin_conc_min: Option<u16>,
    pub cns_load: Option<u8>,
    pub distance: Option<u32>,
    pub zone: Option<u8>,
    pub ball_speed: Option<u16>,
    pub gps_accuracy: Option<u8>,
    pub absolute_pressure: Option<u32>,
    pub calories: Option<u16>,
    pub stance_time: Option<u16>,
    pub right_pco: Option<i8>,
    pub ebike_travel_range: Option<u16>,
    pub left_torque_effectiveness: Option<u8>,
    pub activity_type: Option<u8>,
    pub depth: Option<u32>,
    pub enhanced_speed: Option<u32>,
    pub total_cycles: Option<u32>,
    pub vertical_speed: Option<i16>,
    pub combined_pedal_smoothness: Option<u8>,
    pub saturated_hemoglobin_percent_max: Option<u16>,
    pub altitude: Option<u16>,
    pub left_pco: Option<i8>,
    pub left_power_phase_peak: Option<u8>,
    pub flow: Option<f32>,
    pub device_index: Option<u8>,
    pub cadence: Option<u8>,
    pub ebike_assist_level_percent: Option<u8>,
    pub right_power_phase_peak: Option<u8>,
    pub ebike_battery_level: Option<u8>,
    pub compressed_speed_distance: Option<u8>,
    pub left_pedal_smoothness: Option<u8>,
    pub right_power_phase: Option<u8>,
    pub right_torque_effectiveness: Option<u8>,
    pub time_to_surface: Option<u32>,
}

impl FitRecordMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FitFieldValue>) -> Self {
        let mut msg = FitRecordMsg { step_length: None, 
            resistance: None, speed: None, accumulated_power: None, 
            next_stop_depth: None, stroke_type: None, heart_rate: None, 
            cycles: None, total_hemoglobin_conc: None, time_from_course: None, 
            saturated_hemoglobin_percent: None, cadence256: None, ndl_time: None, 
            next_stop_time: None, time128: None, left_right_balance: None, 
            temperature: None, position_long: None, motor_power: None, 
            vertical_ratio: None, grit: None, compressed_accumulated_power: None, 
            position_lat: None, ebike_assist_mode: None, n2_load: None, 
            grade: None, left_power_phase: None, power: None, 
            timestamp: None, vertical_oscillation: None, fractional_cadence: None, 
            saturated_hemoglobin_percent_min: None, cycle_length: None, right_pedal_smoothness: None, 
            total_hemoglobin_conc_max: None, stance_time_percent: None, enhanced_altitude: None, 
            stance_time_balance: None, speed_1s: None, battery_soc: None, 
            total_hemoglobin_conc_min: None, cns_load: None, distance: None, 
            zone: None, ball_speed: None, gps_accuracy: None, 
            absolute_pressure: None, calories: None, stance_time: None, 
            right_pco: None, ebike_travel_range: None, left_torque_effectiveness: None, 
            activity_type: None, depth: None, enhanced_speed: None, 
            total_cycles: None, vertical_speed: None, combined_pedal_smoothness: None, 
            saturated_hemoglobin_percent_max: None, altitude: None, left_pco: None, 
            left_power_phase_peak: None, flow: None, device_index: None, 
            cadence: None, ebike_assist_level_percent: None, right_power_phase_peak: None, 
            ebike_battery_level: None, compressed_speed_distance: None, left_pedal_smoothness: None, 
            right_power_phase: None, right_torque_effectiveness: None, time_to_surface: None, 
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    85 => { msg.step_length = Some(field.get_u16()); },
                    10 => { msg.resistance = Some(field.get_u8()); },
                    6 => { msg.speed = Some(field.get_u16()); },
                    29 => { msg.accumulated_power = Some(field.get_u32()); },
                    93 => { msg.next_stop_depth = Some(field.get_u32()); },
                    49 => { msg.stroke_type = Some(field.get_u8()); },
                    3 => { msg.heart_rate = Some(field.get_u8()); },
                    18 => { msg.cycles = Some(field.get_u8()); },
                    54 => { msg.total_hemoglobin_conc = Some(field.get_u16()); },
                    11 => { msg.time_from_course = Some(field.get_i32()); },
                    57 => { msg.saturated_hemoglobin_percent = Some(field.get_u16()); },
                    52 => { msg.cadence256 = Some(field.get_u16()); },
                    96 => { msg.ndl_time = Some(field.get_u32()); },
                    94 => { msg.next_stop_time = Some(field.get_u32()); },
                    48 => { msg.time128 = Some(field.get_u8()); },
                    30 => { msg.left_right_balance = Some(field.get_u8()); },
                    13 => { msg.temperature = Some(field.get_i8()); },
                    1 => { msg.position_long = Some(field.get_i32()); },
                    82 => { msg.motor_power = Some(field.get_u16()); },
                    83 => { msg.vertical_ratio = Some(field.get_u16()); },
                    114 => { msg.grit = Some(field.get_f32()); },
                    28 => { msg.compressed_accumulated_power = Some(field.get_u16()); },
                    0 => { msg.position_lat = Some(field.get_i32()); },
                    119 => { msg.ebike_assist_mode = Some(field.get_u8()); },
                    98 => { msg.n2_load = Some(field.get_u16()); },
                    9 => { msg.grade = Some(field.get_i16()); },
                    69 => { msg.left_power_phase = Some(field.get_u8()); },
                    7 => { msg.power = Some(field.get_u16()); },
                    253 => { msg.timestamp = Some(field.get_u32()); },
                    39 => { msg.vertical_oscillation = Some(field.get_u16()); },
                    53 => { msg.fractional_cadence = Some(field.get_u8()); },
                    58 => { msg.saturated_hemoglobin_percent_min = Some(field.get_u16()); },
                    12 => { msg.cycle_length = Some(field.get_u8()); },
                    46 => { msg.right_pedal_smoothness = Some(field.get_u8()); },
                    56 => { msg.total_hemoglobin_conc_max = Some(field.get_u16()); },
                    40 => { msg.stance_time_percent = Some(field.get_u16()); },
                    78 => { msg.enhanced_altitude = Some(field.get_u32()); },
                    84 => { msg.stance_time_balance = Some(field.get_u16()); },
                    17 => { msg.speed_1s = Some(field.get_u8()); },
                    81 => { msg.battery_soc = Some(field.get_u8()); },
                    55 => { msg.total_hemoglobin_conc_min = Some(field.get_u16()); },
                    97 => { msg.cns_load = Some(field.get_u8()); },
                    5 => { msg.distance = Some(field.get_u32()); },
                    50 => { msg.zone = Some(field.get_u8()); },
                    51 => { msg.ball_speed = Some(field.get_u16()); },
                    31 => { msg.gps_accuracy = Some(field.get_u8()); },
                    91 => { msg.absolute_pressure = Some(field.get_u32()); },
                    33 => { msg.calories = Some(field.get_u16()); },
                    41 => { msg.stance_time = Some(field.get_u16()); },
                    68 => { msg.right_pco = Some(field.get_i8()); },
                    117 => { msg.ebike_travel_range = Some(field.get_u16()); },
                    43 => { msg.left_torque_effectiveness = Some(field.get_u8()); },
                    42 => { msg.activity_type = Some(field.get_u8()); },
                    92 => { msg.depth = Some(field.get_u32()); },
                    73 => { msg.enhanced_speed = Some(field.get_u32()); },
                    19 => { msg.total_cycles = Some(field.get_u32()); },
                    32 => { msg.vertical_speed = Some(field.get_i16()); },
                    47 => { msg.combined_pedal_smoothness = Some(field.get_u8()); },
                    59 => { msg.saturated_hemoglobin_percent_max = Some(field.get_u16()); },
                    2 => { msg.altitude = Some(field.get_u16()); },
                    67 => { msg.left_pco = Some(field.get_i8()); },
                    70 => { msg.left_power_phase_peak = Some(field.get_u8()); },
                    115 => { msg.flow = Some(field.get_f32()); },
                    62 => { msg.device_index = Some(field.get_u8()); },
                    4 => { msg.cadence = Some(field.get_u8()); },
                    120 => { msg.ebike_assist_level_percent = Some(field.get_u8()); },
                    72 => { msg.right_power_phase_peak = Some(field.get_u8()); },
                    118 => { msg.ebike_battery_level = Some(field.get_u8()); },
                    8 => { msg.compressed_speed_distance = Some(field.get_u8()); },
                    45 => { msg.left_pedal_smoothness = Some(field.get_u8()); },
                    71 => { msg.right_power_phase = Some(field.get_u8()); },
                    44 => { msg.right_torque_effectiveness = Some(field.get_u8()); },
                    95 => { msg.time_to_surface = Some(field.get_u32()); },
                    87 => { }, // Can't find a definition for these.
                    88 => { },
                    108 => { },
                    _ => { /* panic!("Record field not implemented {:#x}", field.field_def); */ }
                }
            }
        }
        msg
    }
}

pub struct FitEventMsg {
    pub event_group: Option<u8>,
    pub rear_gear: Option<u8>,
    pub data16: Option<u16>,
    pub event: Option<u8>,
    pub rear_gear_num: Option<u8>,
    pub score: Option<u16>,
    pub data: Option<u32>,
    pub radar_threat_level_max: Option<u8>,
    pub front_gear: Option<u8>,
    pub device_index: Option<u8>,
    pub opponent_score: Option<u16>,
    pub timestamp: Option<u32>,
    pub event_type: Option<u8>,
    pub radar_threat_count: Option<u8>,
    pub front_gear_num: Option<u8>,
}

impl FitEventMsg {

    /// Constructor: Takes the fields that were read by the file parser and puts them into a structure.
    pub fn new(fields: Vec<FitFieldValue>) -> Self {
        let mut msg = FitEventMsg { event_group: None,
            rear_gear: None, data16: None, event: None,
            rear_gear_num: None, score: None, data: None,
            radar_threat_level_max: None, front_gear: None, device_index: None,
            opponent_score: None, timestamp: None, event_type: None,
            radar_threat_count: None, front_gear_num: None,
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    4 => { msg.event_group = Some(field.get_u8()); },
                    12 => { msg.rear_gear = Some(field.get_u8()); },
                    2 => { msg.data16 = Some(field.get_u16()); },
                    0 => { msg.event = Some(field.get_u8()); },
                    11 => { msg.rear_gear_num = Some(field.get_u8()); },
                    7 => { msg.score = Some(field.get_u16()); },
                    3 => { msg.data = Some(field.get_u32()); },
                    21 => { msg.radar_threat_level_max = Some(field.get_u8()); },
                    10 => { msg.front_gear = Some(field.get_u8()); },
                    13 => { msg.device_index = Some(field.get_u8()); },
                    8 => { msg.opponent_score = Some(field.get_u16()); },
                    253 => { msg.timestamp = Some(field.get_u32()); },
                    1 => { msg.event_type = Some(field.get_u8()); },
                    22 => { msg.radar_threat_count = Some(field.get_u8()); },
                    9 => { msg.front_gear_num = Some(field.get_u8()); },
                    _ => { /* panic!("Record field not implemented {:#x}", field.field_def); */ }
                }
            }
        }
        msg
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FitWorkoutMsg {
    pub message_index: Option<u16>,
    /// enum with values defined in constants `FIT_SPORT_*`
    pub sport: Option<u8>,
    /// bitmask with flags defined in constants `WORKOUT_CAPABILITIES_*`
    pub capabilities: Option<u32>,
    /// number of workout steps included in the file
    pub num_valid_steps: Option<u16>,
    /// nul-terminated string with the workout name
    pub workout_name: Option<String>,
    /// enum with values defined in constants `FIT_SUB_SPORT_*`
    pub sub_sport: Option<u8>,
    /// pool length measured in meters
    pub pool_length: Option<u16>,
    /// pool length display unit
    /// enum with values defined in constants `DISPLAY_MEASURE_*`
    pub pool_length_unit: Option<u8>,
}

impl FitWorkoutMsg {
    pub fn new(fields: Vec<FitFieldValue>) -> FitWorkoutMsg {
        let mut msg = FitWorkoutMsg {
            message_index: None,
            sport: None,
            num_valid_steps: None,
            workout_name: None,
            sub_sport: None,
            capabilities: None,
            pool_length: None,
            pool_length_unit: None,
        };

        for field in fields.iter() {
            if !field.is_dev_field {
                match field.field_def {
                    4 => msg.sport = Some(field.get_u8()),
                    5 => msg.capabilities = Some(field.get_u32()),
                    6 => msg.num_valid_steps = Some(field.get_u16()),
                    8 => msg.workout_name = Some(field.value_string.clone()),
                    11 => msg.sub_sport = Some(field.get_u8()),
                    14 => msg.pool_length = Some(field.get_u16()),
                    15 => msg.pool_length_unit = Some(field.get_u8()),
                    254 => msg.message_index = Some(field.get_u16()),
                    _ => {}
                }
            }
        }

        msg
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FitWorkoutStepMsg {
    pub message_index: u16,
    pub step_name: Option<String>,
    /// enum values defined in constants `WORKOUT_STEP_DURATION_*`
    pub duration_type: Option<u8>,
    pub duration_value: Option<u32>,
    /// enum values defined in constants `WORKOUT_STEP_TARGET_*`
    pub target_type: Option<u8>,
    /// If `target_type` indicates speed, heart rate, cadence or power, then this indicates zones
    /// with value >0  or `target_value` = 0 for custom values defined with `custom_target_low` and
    /// `custom_target_high`
    pub target_value: Option<u32>,
    /// If `target_type` indicates power, then values 0-1000 are to be interpreted as percentage of
    /// user FTP (0%-1000%), values greater than 1000 are to be interpreted as 
    /// (1000 + absolute power). E.g. 1250 would be absolute power of 250W.
    ///
    /// If `target_type` indicates heart rate, then values 0-100 are to be interpreted as 
    /// percentage of use max heart rate (0%-100%), values greater than 100 are to be interpreted
    /// as (100 + absolute heart rate). E.g. 250 would be absolute HR of 150BPM.
    pub custom_target_low: Option<u32>,
    /// See `custom_target_low` for more details of the content
    pub custom_target_high: Option<u32>,
    /// enum values defined in constants `INTENSITY_*`
    pub intensity: Option<u8>,
    pub notes: Option<String>,
    /// enum values defined in constants `WORKOUT_EQUIPMENT_*`
    pub equipment: Option<u8>,
    /// enum values defined in constants `WORKOUT_STEP_TARGET_*`
    pub secondary_target_type: Option<u8>,
    /// See `target_value` for more details of the content
    pub secondary_target_value: Option<u32>,
    /// See `custom_target_low` for more details for the content
    pub secondary_custom_target_low: Option<u32>,
    /// See `custom_target_low` for more details for the content
    pub secondary_custom_target_high: Option<u32>,
}

impl FitWorkoutStepMsg {
    pub fn new(message_index: u16, fields: Vec<FitFieldValue>) -> FitWorkoutStepMsg {
        let mut msg = FitWorkoutStepMsg{
            message_index,
            step_name: None,
            duration_type: None,
            duration_value: None,
            target_type: None,
            target_value: None,
            custom_target_low: None,
            custom_target_high: None,
            intensity: None,
            notes: None,
            equipment: None,
            secondary_target_type: None,
            secondary_target_value: None,
            secondary_custom_target_low: None,
            secondary_custom_target_high: None,
        };

        for field in fields {
            if !field.is_dev_field {
                match field.field_def {
                    0 => msg.step_name = Some(field.value_string),
                    1 => msg.duration_type = Some(field.get_u8()),
                    2 => msg.duration_value = Some(field.get_u32()),
                    3 => msg.target_type = Some(field.get_u8()),
                    4 => msg.target_value = Some(field.get_u32()),
                    5 => msg.custom_target_low = Some(field.get_u32()),
                    6 => msg.custom_target_high = Some(field.get_u32()),
                    7 => msg.intensity = Some(field.get_u8()),
                    8 => msg.notes = Some(field.value_string),
                    9 => msg.equipment = Some(field.get_u8()),
                    19 => msg.secondary_target_type = Some(field.get_u8()),
                    20 => msg.secondary_target_value = Some(field.get_u32()),
                    21 => msg.secondary_custom_target_low = Some(field.get_u32()),
                    22 => msg.secondary_custom_target_high = Some(field.get_u32()),
                    _ => {}
                }
            }
        }

        msg
    }
}

#[derive(Clone, Debug)]
pub enum FieldType {
    FieldTypeNotSet, // Value not set
    FieldTypeUInt, // Value is an unsigned integer
    FieldTypeSInt, // Value is a signed integer
    FieldTypeFloat, // Value is a float
    FieldTypeByteArray, // Value is a byte array
    FieldTypeStr // Value is a tring
}

#[derive(Clone, Debug)]
pub struct FitFieldValue {
    pub field_def: u8, // From the message definition
    pub base_type: u8, // Base type, from the SDK
    pub type_enum: FieldType, // Tells us which of the following to use
    pub value_uint: u64,
    pub value_sint: i64,
    pub value_float: f64,
    pub value_byte_array: Vec<u8>,
    pub value_string: String,
    pub is_dev_field: bool
}

impl FitFieldValue {
    pub fn new() -> Self {
        let state = FitFieldValue{ field_def: 0, type_enum: FieldType::FieldTypeNotSet, base_type: 0, value_uint: 0, value_sint: 0, value_float: 0.0, value_byte_array: Vec::<u8>::new(), value_string: String::new(), is_dev_field: false };
        state
    }

    pub fn get_i8(&self) -> i8 {
        return self.value_sint as i8;
    }

    pub fn get_i16(&self) -> i16 {
        return self.value_sint as i16;
    }

    pub fn get_i32(&self) -> i32 {
        return self.value_sint as i32;
    }

    pub fn get_i64(&self) -> i64 {
        return self.value_sint as i64;
    }

    pub fn get_u8(&self) -> u8 {
        return self.value_uint as u8;
    }

    pub fn get_u16(&self) -> u16 {
        return self.value_uint as u16;
    }

    pub fn get_u32(&self) -> u32 {
        return self.value_uint as u32;
    }

    pub fn get_u64(&self) -> u64 {
        return self.value_uint as u64;
    }

    pub fn get_f32(&self) -> f32 {
        return self.value_float as f32;
    }

    pub fn get_f64(&self) -> f64 {
        return self.value_float as f64;
    }
}

/// Encapsulates a custom field definition, as described by definition messages and used by data messages.
#[derive(Copy, Clone, Debug, Default)]
pub struct FieldDefinition {
    pub field_def: u8, // Field definition number
    pub size: u8, // Number of bytes
    pub base_type: u8, // Base type (from the SDK)
    pub is_dev_field: bool // Set if this represents a developer defined field
}

impl Ord for FieldDefinition {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.field_def, &self.size, &self.base_type, &self.is_dev_field).cmp(&(other.field_def, &other.size, &other.base_type, &other.is_dev_field))
    }
}

impl PartialOrd for FieldDefinition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FieldDefinition {
    fn eq(&self, other: &Self) -> bool {
        (self.field_def, &self.size, &self.is_dev_field) == (other.field_def, &other.size, &other.is_dev_field)
    }
}

impl Eq for FieldDefinition { }

pub type FieldDefinitionList = Vec<FieldDefinition>;

/// Contains everything we need to remember about the state of the file parsing operation.
#[derive(Debug, Default)]
struct FitState {
    endianness_map: HashMap<u8, bool>, // true = messages of the given local message type are in big endian format
    global_msg_map: HashMap<u8, u16>, // Associates local message types with global message numbers
    field_defs: HashMap<u8, FieldDefinitionList>, // Describes the format of local messages, key is the local message type
    timestamp: u32, // Current timestamp, listed here as it may be updated by a compressed timestamp header
    bytes_read: u64 // Number of bytes read so far
}

impl FitState {
    pub fn new() -> Self {
        let state = FitState{ endianness_map: HashMap::<u8, bool>::new(), global_msg_map: HashMap::<u8, u16>::new(), field_defs: HashMap::<u8, FieldDefinitionList>::new(), timestamp: 0, bytes_read: 0 };
        state
    }

    /// For debugging purposes.
    fn print(&self) {
        println!("----------------------------------------");

        for (local_msg_type, local_msg_def) in &self.field_defs {
            println!("Local Msg Type {}:", local_msg_type);
            for field_def in local_msg_def.iter() {
                println!("   Field Def {} Size {} Base Type {:#x}", field_def.field_def, field_def.size, field_def.base_type);
            }
        }

        println!("----------------------------------------");
    }

    /// Adds the given global message/local message combo to the hash map.
    fn insert_local_msg_def(&mut self, local_msg_type: u8, is_big_endian: bool, global_msg_num: u16, field_defs: FieldDefinitionList) {

        // Update the endianness map.
        if self.endianness_map.contains_key(&local_msg_type) {
            self.endianness_map.remove(&local_msg_type);
        }
        self.endianness_map.insert(local_msg_type, is_big_endian);

        // Update the global message number association.
        if self.global_msg_map.contains_key(&local_msg_type) {
            self.global_msg_map.remove(&local_msg_type);
        }
        self.global_msg_map.insert(local_msg_type, global_msg_num);

        // Update the message definition.
        if self.field_defs.contains_key(&local_msg_type) {
            self.field_defs.remove(&local_msg_type);
        }
        self.field_defs.insert(local_msg_type, field_defs);
    }
}

/// Parses and validates the FIT file header.
#[derive(Debug, Default)]
pub struct FitHeader {
    pub header: Vec<u8>,
    pub header_buf2: [u8; 2], // Additional information introduced with the 14 byte header
    pub header_len: u8 // Number of bytes in the header
}

impl FitHeader {
    pub fn new() -> Self {
        let header = FitHeader{ header: Vec::new(), header_buf2: [0u8; 2], header_len: 0 };
        header
    }

    /// Reads the FIT File Header from the buffer.
    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) -> Result<()> {

        // Reads first 12 bytes of the header (12 bytes is the minimum header size for a valid FIT file).
        self.header = read_n(reader, 12)?;
        self.header_len = 12;

        // Does this file use the newer, 14 byte header?
        if self.header[HEADER_FILE_SIZE_OFFSET] == 14 {
            let mut additional_bytes = read_n(reader, 2)?;

            self.header.append(&mut additional_bytes);
            self.header_len = self.header_len + 2;
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
    pub header_byte: u8
}

impl FitRecord {
    pub fn new() -> Self {
        let rec = FitRecord{ header_byte: 0 };
        rec
    }

    /// Assumes the buffer is pointing to the beginning of the definition message, reads the message, and updates the field definitions.
    fn read_definition_message<R: Read>(&mut self, reader: &mut BufReader<R>, state: &mut FitState) -> Result<()> {
        println!("Definition Msg");

        // Local message type.
        let local_msg_type = self.header_byte & RECORD_HDR_LOCAL_MSG_TYPE;

        // Definition message (5 bytes).
        // 0: Reserved
        // 1: Architecture
        // 2-3: Global Message Number
        // 4: Number of Fields
        let mut definition_header: [u8; 5] = [0; 5];
        reader.read_exact(&mut definition_header)?;
        state.bytes_read = state.bytes_read + 5;

        // Make a note of the Architecture and Global Message Number.
        let is_big_endian = definition_header[DEF_MSG_ARCHITECTURE] == 1;
        let global_msg_num = byte_array_to_uint16(definition_header[DEF_MSG_GLOBAL_MSG_NUM..(DEF_MSG_GLOBAL_MSG_NUM + 2)].to_vec(), is_big_endian);

        // Read each field.
        let mut field_defs: FieldDefinitionList = FieldDefinitionList::new();
        let num_fields = definition_header[DEF_MSG_NUM_FIELDS];
        for _i in 0..num_fields {

            // Read the field definition (3 bytes).
            let field_num = read_byte(reader)?;
            let field_bytes = read_byte(reader)?;
            let base_type = read_byte(reader)?;
            state.bytes_read = state.bytes_read + 3;

            // Add the definition.
            let field_def = FieldDefinition { field_def:field_num, size:field_bytes, base_type:base_type, is_dev_field:false };
            field_defs.push(field_def);
        }

        // Is there any developer information in this record?
        if self.header_byte & RECORD_HDR_MSG_TYPE_SPECIFIC != 0 {

            // Read the number of developer fields (1 byte).
            let num_dev_fields = read_byte(reader)?;

            // Read each developer field.
            for _i in 0..num_dev_fields {

                // Read the field definition (3 bytes).
                let field_num = read_byte(reader)?;
                let field_bytes = read_byte(reader)?;
                let base_type = read_byte(reader)?;
                state.bytes_read = state.bytes_read + 3;

                // Add the definition.
                let field_def = FieldDefinition { field_def:field_num, size:field_bytes, base_type:base_type, is_dev_field:true };
                field_defs.push(field_def);
            }
        }

        // Associate the field definitions with the local message type.
        state.insert_local_msg_def(local_msg_type, is_big_endian, global_msg_num, field_defs);

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the data message, reads the message.
    fn read_data_message<C, R: Read>(&mut self, reader: &mut BufReader<R>, state: &mut FitState, callback: Callback<C>, context: &mut C) -> Result<()> {
        println!("Data Msg");

        // Local message type. The local message type is stored differently for compressed data headers.
        let local_msg_type;
        if self.header_byte & RECORD_HDR_NORMAL != 0 {
            local_msg_type = (self.header_byte & RECORD_HDR_LOCAL_MSG_TYPE_COMPRESSED) >> 5;
        }
        else {
            local_msg_type = self.header_byte & RECORD_HDR_LOCAL_MSG_TYPE;
        }

        // Do we know about this message type?
        if !state.endianness_map.contains_key(&local_msg_type) {
            let e = Error::new(std::io::ErrorKind::NotFound, "Field definition not found.");
            return Err(e);
        }
        if !state.global_msg_map.contains_key(&local_msg_type) {
            let e = Error::new(std::io::ErrorKind::NotFound, "Field definition not found.");
            return Err(e);
        }
        if !state.field_defs.contains_key(&local_msg_type) {
            let e = Error::new(std::io::ErrorKind::NotFound, "Field definition not found.");
            return Err(e);
        }

        // The timestamp may get updated.
        let mut new_timestamp = state.timestamp;

        // Retrieve the field definitions based on the message type.
        let is_big_endian = *state.endianness_map.get(&local_msg_type).unwrap();
        let global_msg_num = *state.global_msg_map.get(&local_msg_type).unwrap();
        let field_defs = state.field_defs.get(&local_msg_type).unwrap();

        // Read data for each message definition.
        let mut fields = Vec::new();
        let mut message_index: u16 = 0;
        let mut bytes_read = 0;
        for def in field_defs.iter() {

            let mut field = FitFieldValue::new();
            field.field_def = def.field_def;
            field.is_dev_field = def.is_dev_field;

            // Read the number of bytes prescribed by the field definition.
            let data = read_n(reader, def.size as u64)?;
            let num_bytes_read = data.len();
            if num_bytes_read != def.size as usize {
                let e = Error::new(std::io::ErrorKind::InvalidData, "Failed to read the required number of bytes.");
                return Err(e);
            }
            bytes_read = bytes_read + num_bytes_read as u64;

            // Is this a special field, like a timestamp?
            if def.field_def == FIELD_MSG_INDEX {
                message_index = byte_array_to_sint16(data, is_big_endian) as u16;
            }
            else if def.field_def == FIELD_TIMESTAMP {
                new_timestamp = byte_array_to_uint32(data, is_big_endian);
            }
            else if def.field_def == FIELD_PART_INDEX {
                panic!("Part Index not implemented: Local Message Type: {}.", local_msg_type);
            }

            // Normal field.
            else {
                match def.base_type {
                    0x00 => { field.value_uint = byte_array_to_uint8(data) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    0x01 => { field.value_sint = byte_array_to_sint8(data) as i64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeSInt; },
                    0x02 => { field.value_uint = byte_array_to_uint8(data) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    0x83 => { field.value_sint = byte_array_to_sint16(data, is_big_endian) as i64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeSInt; },
                    0x84 => { field.value_uint = byte_array_to_uint16(data, is_big_endian) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    0x85 => { field.value_sint = byte_array_to_sint32(data, is_big_endian) as i64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeSInt; },
                    0x86 => { field.value_uint = byte_array_to_uint32(data, is_big_endian) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    0x07 => { field.value_string = byte_array_to_string(data, def.size as usize); field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeStr; },
                    0x88 => { field.value_float = byte_array_to_float(data, 4, is_big_endian); field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeFloat; },
                    0x89 => { field.value_float = byte_array_to_float(data, 8, is_big_endian); field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeFloat; },
                    0x0A => { field.value_uint = byte_array_to_uint8(data) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    0x8B => { field.value_uint = byte_array_to_uint16(data, is_big_endian) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    0x8C => { field.value_uint = byte_array_to_uint32(data, is_big_endian) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    0x0D => { field.value_byte_array = data; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeByteArray; },
                    0x8E => { field.value_sint = byte_array_to_sint64(data, is_big_endian) as i64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeSInt; },
                    0x8F => { field.value_uint = byte_array_to_uint64(data, is_big_endian) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    0x90 => { field.value_uint = byte_array_to_uint64(data, is_big_endian) as u64; field.base_type = def.base_type; field.type_enum = FieldType::FieldTypeUInt; },
                    _ => { if !def.is_dev_field {
                            panic!("Base type {:#x} not implemented for field {:#x} and local message type {}. Bytes read so far {:#x}.", def.base_type, def.field_def, local_msg_type, state.bytes_read + bytes_read as u64);
                        }
                    }
                }
                fields.push(field);
            }
        }

        // Update the bytes_read state. Have to do this outside of the loop to make rust happy.
        state.bytes_read = state.bytes_read + bytes_read;

        // Convert the FIT timestamp to UNIX. FIT timestamps are seconds since UTC 00:00:00 Dec 31 1989.
        let mut display_timestamp = 0;
        if new_timestamp > 0 {
            display_timestamp = 631065600 + new_timestamp;
        }

        // Tell the people.
        callback(display_timestamp, global_msg_num, local_msg_type, message_index, fields, context);

        // Store the (possibly) updated timestamp.
        state.timestamp = new_timestamp;

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the compressed timestamp message, reads the message.
    fn read_compressed_timestamp_message<C, R: Read>(&mut self, reader: &mut BufReader<R>, state: &mut FitState, callback: Callback<C>, context: &mut C) -> Result<()> {
        // Compressed Timestamp Header.
        let time_offset = (self.header_byte & 0x1f) as u32;
        if time_offset >= state.timestamp & 0x0000001F { // offset value is greater than least significant 5 bits of previous timestamp
            state.timestamp = (state.timestamp & 0xFFFFFFE0) + time_offset;
        }
        else {
            state.timestamp = (state.timestamp & 0xFFFFFFE0) + time_offset + 0x00000020;
        }

        // Read the data fields that follow.
        self.read_data_message(reader, state, callback, context)?;

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the normal message, reads the message.
    fn read_normal_message<C, R: Read>(&mut self, reader: &mut BufReader<R>, state: &mut FitState, callback: Callback<C>, context: &mut C) -> Result<()> {
        // Reserve bit should be zero in normal messages.
        if self.header_byte & RECORD_HDR_RESERVED != 0 {
            let e = Error::new(std::io::ErrorKind::InvalidData, "Reserve bit set.");
            return Err(e);
        }

        // Data or definition message?
        // A value of zero indicates a data message.
        if self.header_byte & RECORD_HDR_MSG_TYPE != 0 {
            self.read_definition_message(reader, state)?;
        }
        else {
            self.read_data_message(reader, state, callback, context)?;
        }

        Ok(())
    }

    /// Assumes the buffer is pointing to the beginning of the next record message, reads the message.
    fn read<C, R: Read>(&mut self, reader: &mut BufReader<R>, state: &mut FitState, callback: Callback<C>, context: &mut C) -> Result<()> {
        // The first byte is a bit field that tells us more about the record.
        self.header_byte = read_byte(reader)?;
        state.bytes_read = state.bytes_read + 1;
        //println!("header_byte {:#04x} bytes_read {}", self.header_byte, state.bytes_read);

        // Normal header or compressed timestamp header?
        // A value of zero indicates a normal header.
        if self.header_byte & RECORD_HDR_NORMAL != 0 {
            self.read_compressed_timestamp_message(reader, state, callback, context)?;
        }
        else {
            self.read_normal_message(reader, state, callback, context)?;
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

    /// CRC validation function.
    fn check_crc(&self, crc: u16, byte: u8) -> u16{
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

        crc2
    }

    /// Reads the FIT data from the buffer.
    pub fn read<C, R: Read>(&mut self, reader: &mut BufReader<R>, callback: Callback<C>, context: &mut C) -> Result<()> {
        let mut state = FitState::new();

        // Read the file header.
        self.header.read(reader)?;
        state.bytes_read = self.header.header_len as u64;

        // Make sure the header is valid.
        if self.header.validate() {

            let mut error = false;

            // Bytes to read is specified in the header as being the number of bytes after the header.
            // We also need to subtract the two bytes for the CRC.
            let bytes_to_read = self.header.header_len as u64 + self.header.data_size() as u64 - 2;

            // Read each record.
            while !error && state.bytes_read < bytes_to_read {

                let mut record = FitRecord::new();
                let result = record.read(reader, &mut state, callback, context);

                match result {
                    Ok(_result) => {
                    }
                    Err(e) => {
                        println!("Error: {} Bytes Read: {}", e, state.bytes_read);
                        error = true;
                    }
                }
            }

            // Read the CRC.
            //self.check_crc();
        }

        Ok(())
    }
}

pub fn read<C, R: Read>(reader: &mut BufReader<R>, callback: Callback<C>, context: &mut C) -> Result<Fit> {
    let mut fit: Fit = Fit::new();
    fit.read(reader, callback, context)?;

    Ok(fit)
}
