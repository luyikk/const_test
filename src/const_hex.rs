pub const fn get_encode_hex_len(len: usize) -> usize {
    len * 2
}

pub const fn hex(buf: &'static [u8]) -> HexData {
    HexData {
        buf,
        len: get_encode_hex_len(buf.len()),
    }
}

pub struct HexData {
    pub buf: &'static [u8],
    pub len: usize,
}

impl HexData {
    pub const fn encode<const LEN: usize>(mut input: &[u8]) -> [u8; LEN] {
        let mut out = [0; LEN];
        let mut i = 0usize;
        let hex_char = b"0123456789ABCDEF";
        while let [c, ref next @ ..] = *input {
            out[i] = hex_char[(c >> 4) as usize];
            i += 1;
            out[i] = hex_char[(c & 0xF) as usize];
            i += 1;
            input = next;
        }
        out
    }
}
