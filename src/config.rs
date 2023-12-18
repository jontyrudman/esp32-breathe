use crate::constants;

pub struct Config {
    pub items: [ConfigItem; 5],
    current_item_idx: usize,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Self {
        use SettingName::*;
        Config {
            items: [
                ConfigItem {
                    setting: InhaleTimeMs,
                    value: constants::MIN_INHALE_TIME_MS,
                },
                ConfigItem {
                    setting: ExhaleTimeMs,
                    value: constants::MIN_EXHALE_TIME_MS,
                },
                ConfigItem {
                    setting: HoldTimeMs,
                    value: constants::MIN_HOLD_TIME_MS,
                },
                ConfigItem {
                    setting: AirlessTimeMs,
                    value: constants::MIN_AIRLESS_TIME_MS,
                },
                ConfigItem {
                    setting: BrightnessPct,
                    value: 100,
                },
            ],
            current_item_idx: 0,
        }
    }

    pub fn adjust_setting(&mut self, setting: SettingName, value: u8) {
        for item in &mut self.items {
            if item.setting == setting {
                item.adjust(value);
            }
        }
    }

    pub fn adjust_setting_by_index(&mut self, index: usize, value: u8) {
        self.items[index].adjust(value);
    }

    pub fn adjust_current_setting(&mut self, value: u8) {
        self.items[self.current_item_idx].adjust(value);
    }

    pub fn next_item(&mut self) {
        self.current_item_idx = (self.current_item_idx + 1) % self.items.len();
    }

    pub fn current_item(&self) -> ConfigItem {
        self.items[self.current_item_idx]
    }

    pub fn get(&self, setting: SettingName) -> Option<u16> {
        for item in self.items {
            if item.setting == setting {
                return Some(item.value);
            }
        }
        return None;
    }
}

#[derive(Copy, Clone)]
pub struct ConfigItem {
    pub setting: SettingName,
    pub value: u16,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SettingName {
    InhaleTimeMs,
    ExhaleTimeMs,
    HoldTimeMs,
    AirlessTimeMs,
    BrightnessPct,
}

impl SettingName {
    pub fn as_str<'a>(&self) -> &'a str {
        use SettingName::*;
        match self {
            InhaleTimeMs => return "Inhale Time MS",
            ExhaleTimeMs => return "Exhale Time MS",
            HoldTimeMs => return "Hold Time MS",
            AirlessTimeMs => return "Airless Time MS",
            BrightnessPct => return "Brightness Pct",
        }
    }
}

fn segment_to_value(
    segment: u8,
    segment_min: u8,
    segment_max: u8,
    value_min: u16,
    value_max: u16,
) -> u16 {
    let adjusted_segment = segment - segment_min;
    let segment_div = adjusted_segment as f32 / (segment_max - segment_min) as f32;
    let value = value_min as f32 + (segment_div as f32 * (value_max - value_min) as f32);
    value as u16
}

impl ConfigItem {
    pub fn adjust(&mut self, segment: u8) {
        use SettingName::*;
        match self.setting {
            InhaleTimeMs => {
                self.value = segment_to_value(
                    segment,
                    constants::SEGMENT_MIN,
                    constants::SEGMENT_MAX,
                    constants::MIN_INHALE_TIME_MS,
                    constants::MAX_INHALE_TIME_MS,
                );
            }
            ExhaleTimeMs => {
                self.value = segment_to_value(
                    segment,
                    constants::SEGMENT_MIN,
                    constants::SEGMENT_MAX,
                    constants::MIN_EXHALE_TIME_MS,
                    constants::MAX_EXHALE_TIME_MS,
                );
            }
            HoldTimeMs => {
                self.value = segment_to_value(
                    segment,
                    constants::SEGMENT_MIN,
                    constants::SEGMENT_MAX,
                    constants::MIN_HOLD_TIME_MS,
                    constants::MAX_HOLD_TIME_MS,
                );
            }
            AirlessTimeMs => {
                self.value = segment_to_value(
                    segment,
                    constants::SEGMENT_MIN,
                    constants::SEGMENT_MAX,
                    constants::MIN_AIRLESS_TIME_MS,
                    constants::MAX_AIRLESS_TIME_MS,
                );
            }
            BrightnessPct => {
                self.value = segment_to_value(
                    segment,
                    constants::SEGMENT_MIN,
                    constants::SEGMENT_MAX,
                    0,
                    100,
                );
            }
        }
    }
}
