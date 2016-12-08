// pub struct LogWriter {
// }
//
// impl LogWriter {
// pub fn new() -> Self {}
// }
//

use std::io;
use std::io::Write;
use std::mem::transmute;

pub struct WriteBatch {
    sequence: u64,
    count: u32,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl WriteBatch {}

pub fn emit(batch: &WriteBatch, dest: &mut Write) -> io::Result<()> {
    // TODO consider endianess
    let sequence = unsafe { transmute::<u64, [u8; 8]>(batch.sequence) };
    let count = unsafe { transmute::<u32, [u8; 4]>(batch.count) };
    let key_length = batch.key.len() as u8; // safe cast???
    let value_length = batch.value.len() as u8; // safe cast???
    try!(dest.write_all(&sequence));
    try!(dest.write_all(&count));
    try!(dest.write_all(&[1])); // kTypeValue
    try!(dest.write_all(&[key_length]));
    try!(dest.write_all(&batch.key[..]));
    try!(dest.write_all(&[value_length]));
    try!(dest.write_all(&batch.value[..]));
    return Ok(());
}
pub fn emit_with_crc(batch: &WriteBatch, dest: &mut Write) {}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut actual = Vec::new(); // Vec<u8>
        let batch = WriteBatch {
            sequence: 1,
            count: 1,
            key: String::from("hoge").into_bytes(),
            value: String::from("piyo").into_bytes(),
        };
        emit(&batch, &mut actual);
        assert_eq!(expected, actual);
    }
}
