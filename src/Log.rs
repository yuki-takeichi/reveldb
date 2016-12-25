// pub struct LogWriter {
// }
//
// impl LogWriter {
// pub fn new() -> Self {}
// }
//

use crc::crc32;
use std::num::Wrapping;

use std::mem::transmute;

pub struct WriteBatch {
    sequence: u64,
    count: u32,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl WriteBatch {
    fn repr(&self) -> Vec<u8> {
        let mut dest = Vec::new();
        let sequence = unsafe { transmute::<u64, [u8; 8]>(self.sequence) };
        let count = unsafe { transmute::<u32, [u8; 4]>(self.count) };
        let key_length = self.key.len() as u8; // safe cast???
        let value_length = self.value.len() as u8; // safe cast???
        dest.extend_from_slice(&sequence);
        dest.extend_from_slice(&count);
        dest.extend_from_slice(&[1]); // kTypeValue
        dest.extend_from_slice(&[key_length]);
        dest.extend_from_slice(&self.key[..]);
        dest.extend_from_slice(&[value_length]);
        dest.extend_from_slice(&self.value[..]);
        return dest;
    }

    fn repr_with_header(&self) -> Vec<u8> {
        let mut dest = Vec::new();

        let body = self.repr();
        let mut hoge = Vec::new();
        hoge.push(0x01); // kFullType
        hoge.extend_from_slice(&body[..]);
        let crc = crc32::checksum_castagnoli(&hoge[..]);
        let mask_delta = 0xa282ead8;
        let _crc = ((crc >> 15) | (crc << 17)).wrapping_add(mask_delta);
        let masked_crc = unsafe { transmute::<u32, [u8; 4]>(_crc) };

        let len = unsafe { transmute::<u16, [u8; 2]>(body.len() as u16) };
        dest.extend_from_slice(&masked_crc);
        dest.extend_from_slice(&len);
        dest.extend_from_slice(&[1]); // kFullType
        dest.extend_from_slice(&body);
        return dest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crc::crc32;

    #[test]
    fn crc_for_kFullType() {
        let crc = crc32::checksum_castagnoli(&[0x01]);
        assert_eq!(0xa016d052, crc);
    }

    #[test]
    fn simple() {
        let mut expected = vec![];
        expected.extend_from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // sequence : fixed64
        expected.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // count : fixed32
        expected.push(0x01); // kTypeValue
        expected.push(0x04); // len : variant32
        expected.extend_from_slice(&[0x68, 0x6f, 0x67, 0x65]); // "hoge"
        expected.push(0x04); // len : variant32
        expected.extend_from_slice(&[0x70, 0x69, 0x79, 0x6f]); // "piyo"

        let batch = WriteBatch {
            sequence: 1,
            count: 1,
            key: b"hoge".to_vec(),
            value: b"piyo".to_vec(),
        };
        assert_eq!(expected, batch.repr());
    }

    #[test]
    fn with_header() {
        let mut expected = vec![];
        expected.extend_from_slice(&[0x20, 0xdc, 0xa6, 0x8a]); // crc
        expected.extend_from_slice(&[0x17, 0x00]); // size (little endian)
        expected.extend_from_slice(&[0x01]); // kFullType
        expected.extend_from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // sequence : fixed64
        expected.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // count : fixed32
        expected.push(0x01); // kTypeValue
        expected.push(0x04); // len : variant32
        expected.extend_from_slice(&[0x68, 0x6f, 0x67, 0x65]); // "hoge"
        expected.push(0x04); // len : variant32
        expected.extend_from_slice(&[0x70, 0x69, 0x79, 0x6f]); // "piyo"

        let batch = WriteBatch {
            sequence: 1,
            count: 1,
            key: b"hoge".to_vec(),
            value: b"piyo".to_vec(),
        };
        assert_eq!(expected, batch.repr_with_header());
    }
}
