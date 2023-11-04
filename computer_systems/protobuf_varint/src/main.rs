use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() {
    let second_file = "./varint/150.uint64";

    let result = read_file(second_file).unwrap();
    let uint = vec_to_decimal(result);

    println!("Number: {:?}. Binary {:b}", uint, uint);

    let result = encode(uint);
    println!("Encoded: {:?}", result);
    let decode = decode(b"\x96\x01".to_vec());
    println!("Decode: {:?}", decode);
}

fn read_file(filename: &str) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    Ok(buf)
}

fn vec_to_decimal(vec: Vec<u8>) -> usize {
    let mut sum = 0;

    for (i, value) in vec.iter().enumerate() {
        sum += (*value as usize) << 8 * (vec.len() - 1 - i);
    }

    sum
}

fn encode(mut num: usize) -> Vec<u8> {
    let mut bytes = vec![];

    while num > 0 {
        let mut part = num & 0b01111111;
        num >>= 7;
        // add continuation bit
        if num > 0 {
            part |= 0x80;
        }

        bytes.push(part);
    }

    bytes.into_iter().map(|v| v as u8).collect()
}

fn decode(bytes: Vec<u8>) -> u8 {
    let mut n = 0;

    for byte in bytes.iter().rev() {
        n <<= 7;
        n |= byte & 0b01111111;
    }

    n
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    #[test]
    fn it_should_encode() {
        let max = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 1];

        assert_eq!(encode(1), b"\x01");
        assert_eq!(encode(150), b"\x96\x01");
        assert_eq!(encode(18446744073709551615), max);
    }

    #[test]
    fn it_should_decode() {
        assert_eq!(decode(b"\x96\x01".to_vec()), 150);
    }
}
