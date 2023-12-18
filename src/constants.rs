// Pin numbers, where possible (won't help for button)
pub const POT_PIN_NUM: u8 = 34;
pub const LED_PIN_NUM: u8 = 22;


// Potentiometer consts
pub const POT_READ_COUNT: u16 = 5;
pub const POT_MIN: u16 = 430;
pub const POT_MAX: u16 = 3410;
// Artificially hit the max and min segments later by expanding the deadzone
pub const POT_DEADZONE: u16 = 200;
pub const POT_SEGMENTS: u16 = 10;


// Config init values
pub const SEGMENT_MIN: u8 = 0;
pub const SEGMENT_MAX: u8 = 10;

pub const MIN_INHALE_TIME_MS: u16 = 3000u16;
pub const MAX_INHALE_TIME_MS: u16 = 10000u16;

pub const MIN_EXHALE_TIME_MS: u16 = 3000u16;
pub const MAX_EXHALE_TIME_MS: u16 = 10000u16;

pub const MIN_HOLD_TIME_MS: u16 = 1000u16;
pub const MAX_HOLD_TIME_MS: u16 = 10000u16;

pub const MIN_AIRLESS_TIME_MS: u16 = 500u16;
pub const MAX_AIRLESS_TIME_MS: u16 = 10000u16;
