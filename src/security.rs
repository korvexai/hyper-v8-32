pub struct LicenseKey { pub valid: bool }
pub struct AbuseKey { pub level: u8 }

pub struct HyperCore {
    pub identity_key: u64,
    pub license_key: LicenseKey,
    pub abuse_key: AbuseKey,
}

#[inline(always)]
pub fn watermark(id: u64, identity_key: u64) -> u64 {
    let mut x = id ^ identity_key;
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x
}

#[inline(always)]
pub fn license_bias(valid: bool) -> u64 {
    (!valid as u64).wrapping_mul(0xA5A5_A5A5_A5A5_A5A5)
}

#[inline(always)]
pub fn abuse_bias(level: u8) -> u64 {
    (level as u64).wrapping_mul(0x1111_1111_1111_1111)
}
