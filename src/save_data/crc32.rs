fn generate_table(poly: u32) -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut value = i << 24;
        for _ in (0..8).rev() {
            value = if (value & 0x80000000) != 0 { (value << 1) ^ poly } else { value << 1 };
        }
        table[i as usize] = value;
    }
    table
}

// Reversed CCITT32 CRC
pub fn compute(bytes: &[u8]) -> u32 {
    let table = generate_table(0x04c11db7);

    let mut crc = u32::MAX;
    for &byte in bytes {
        let index = (crc >> 24) ^ (byte as u32);
        crc = (crc << 8) ^ table[index as usize];
    }
    !crc
}
