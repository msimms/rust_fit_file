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
const HEADER_DATA_TYPE_0_OFFSET: usize = 7;
const HEADER_DATA_TYPE_1_OFFSET: usize = 8;
const HEADER_DATA_TYPE_2_OFFSET: usize = 9;
const HEADER_DATA_TYPE_3_OFFSET: usize = 10;
const HEADER_CRC_1_OFFSET: usize = 11;
const HEADER_CRC_2_OFFSET: usize = 12;

#[derive(Debug, Default)]
pub struct FitHeader {
    pub header_buf: [u8; 14]
}

impl FitHeader {
    pub fn new() -> Self {
        let header = FitHeader{ header_buf: [0u8; 14] };
        header
    }

    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) {
        reader.read_exact(&mut self.header_buf);
    }
}

#[derive(Debug, Default)]
pub struct FitRecord {

}

impl FitRecord {
    pub fn new() -> Self {
        let rec = FitRecord{};
        rec
    }

    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) {
    }
}

#[derive(Debug, Default)]
pub struct Fit {
    header: FitHeader,
    records: Vec<FitRecord>
}

impl Fit {
    pub fn new() -> Self {
        let fit = Fit{ header: FitHeader::new(), records: Vec::new() };
        fit
    }

    pub fn read<R: Read>(&mut self, reader: &mut BufReader<R>) {
        self.header.read(reader);
    }
}

pub fn read<R: Read>(reader: &mut BufReader<R>) -> Fit {
    let mut fit: Fit = Fit::new();
    fit.read(reader);
    fit
}
