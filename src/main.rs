mod const_hex;
use crate::const_hex::*;

const DATA:&'static [u8]= {
    const HEX:HexData = hex(b"niahcisa");
    const LEN:usize = HEX.len;
    &HexData::encode::<LEN>(HEX.buf)
};


macro_rules! hex_encode {
    ($buf:expr) => ({
        const _HEX:HexData = hex($buf);
        const _LEN:usize=_HEX.len;
        const _DATA:&'static [u8]= &HexData::encode::<_LEN>(_HEX.buf);
        _DATA
    });
}

const MD:&'static [u8]=hex_encode!(b"aabxcsadf");
const HD:&'static [u8]=hex_encode!(b"aabxcsadf");

fn main() {
    println!("{:?}",DATA);
    unsafe {
        let x = std::str::from_utf8_unchecked(MD);
        println!("{:?}", x);
    }

    unsafe {
        let x = std::str::from_utf8_unchecked(HD);
        println!("{:?}", x);
    }

    let xx=hex_encode!("123123123".as_bytes());
    println!("{:?}",xx);
    let xx=hex_encode!("333333312".as_bytes());
    println!("{:?}",xx);
}
