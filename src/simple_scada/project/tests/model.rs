use std::time::{Duration, UNIX_EPOCH};
use crate::simple_scada::project::DelphiDateTime;

#[test]
fn test_delphi_date() {
    // little endian f64 -> Delphi TDateTime
    // 2026-08-11 13:52:17 UTC
    let bytes: [u8; 8] = [
        0x50, 0x95, 0xDC, 0x7E,
        0xB2, 0x94, 0xE6, 0x40,
    ];

    let raw = f64::from_le_bytes(bytes);

    let actual = DelphiDateTime::from(raw).to_system_time().unwrap();

    let expected = UNIX_EPOCH + Duration::from_secs(1_786_456_337);

    let actual_seconds = actual
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let expected_seconds = expected
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert_eq!(actual_seconds, expected_seconds);
}