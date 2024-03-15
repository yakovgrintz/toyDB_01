pub(crate) fn murmur_hash3_32(key: &[u8], seed: u32) -> u32 {
    let mut hash = seed;
    let c1 = 0xcc9e2d51;
    let c2 = 0x1b873593;
    let r1 = 15;
    let r2 = 13;
    let m = 5;
    let n: u32 = 0xe6546b64;
    let mut len = key.len();
    for chunk in key.chunks(4) {
        let mut k = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            k |= (byte as u32) << (8 * i);
        }
        k = (k * c1) as u32;
        k = (k << r1) as u32 | (k >> (32 - r1)) as u32;
        k = (k * c2) as u32;
        hash = hash ^ k as u32;
        hash = (hash << r2) | (hash >> (32 - r2)) as u32;
        hash = hash * m + n as u32;
        len -= 4;
    }
    let remaining = len % 4;
    if remaining > 0 {
        let last_chunk = &key[len - remaining..];
        let mut k = 0;
        for (i, &byte) in last_chunk.iter().enumerate() {
            k |= (byte as u32) << (8 * i);
        }
        k = (k * c1);
        k = (k << r1) | (k >> (32 - r1)) as u32;
        k = k * c2 as u32;
        hash = hash ^ k as u32;
        hash = (hash << r2) | (hash >> (32 - r2)) as u32;
        hash = hash * m + n as u32;
    }
    hash = hash ^ (len as u32);
    hash = hash ^ (hash >> 16);
    hash = hash * 0x85ebca6b;
    hash = hash ^ (hash >> 13);
    hash = hash * 0xc2b2ae35;
    hash = hash ^ (hash >> 16);
    hash
}
