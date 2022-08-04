mod const_hex;
use const_hex::*;

macro_rules! hex_encode {
    ($buf:expr) => {{
        const _LEN: usize = get_encode_hex_len($buf.len());
        const _DATA: &'static [u8] = &encode::<_LEN>($buf);
        _DATA
    }};
}

const MD: &'static [u8] = hex_encode!(b"aabxcsadf");
const HD: &'static [u8] = hex_encode!(b"aabxcsadf");

fn main() {
    unsafe {
        let x = std::str::from_utf8_unchecked(MD);
        println!("{:?}", x);
    }

    unsafe {
        let x = std::str::from_utf8_unchecked(HD);
        println!("{:?}", x);
    }

    let xx = hex_encode!("123123123".as_bytes());
    println!("{:?}", xx);
    let xx = hex_encode!("333333312".as_bytes());
    println!("{:?}", xx);
}
