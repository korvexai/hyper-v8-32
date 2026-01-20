# ================= IP PROTECTION APPLY SCRIPT =================

$projectRoot = Get-Location
$guid = [guid]::NewGuid().ToString()

Write-Host "Applying IP protection..."

# ================= WRITE LICENSE =================
$licenseText = @"
Personal Use License

"Personal Use" is strictly limited to use by a natural person
(physical individual) acting in a private, non-commercial,
non-professional capacity.

Personal Use does NOT include any form of use by:
- any legal entity (company, corporation, startup, NGO, institution),
- any organization or group,
- any professional or business activity,
- any internal company use, testing, benchmarking, study, or evaluation,
- any use related to products, services, infrastructure, or operations.

Any use by a legal entity, regardless of size, purpose, profit status,
or internal-only usage, is considered Commercial Use and requires
a paid commercial license from Korvex.

Building, wrapping, embedding, or providing services on top of this
engine without a commercial license is strictly prohibited.
"@

Set-Content -Path "$projectRoot\LICENSE" -Value $licenseText -Encoding UTF8

# ================= TRACKING HASH =================
$bytes = [System.Text.Encoding]::UTF8.GetBytes("HYPERV8-32-$guid")
$sha256 = [System.Security.Cryptography.SHA256]::Create()
$hash = ([BitConverter]::ToString($sha256.ComputeHash($bytes))).Replace("-", "").Substring(0,16)

$trackingRs = "pub const TRACKING_HASH: u64 = 0x$hash;"
Set-Content -Path "$projectRoot\src\tracking.rs" -Value $trackingRs -Encoding UTF8

# ================= SECURITY MODULE =================
$securityRs = @"
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
"@

Set-Content -Path "$projectRoot\src\security.rs" -Value $securityRs -Encoding UTF8

Write-Host "IP protection applied successfully."
Write-Host "Tracking ID: HYPERV8-32-$guid"
Write-Host "Tracking Hash: 0x$hash"
