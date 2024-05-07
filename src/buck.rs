#[derive(PartialEq, Debug)]
struct Voltage(u8);

impl Voltage {
    pub fn from_millivolts(millivolts: u16) -> Self {
        if millivolts < 1100 {
            return Voltage(0u8);
        } else if millivolts > 3300 {
            return Voltage(24u8);
        }
        return Voltage((millivolts / 100 - 10) as u8);
    }

    pub fn to_millivolts(&self) -> u16 {
        if self.0 == 0 {
            return 0;
        } else if self.0 == 24 {
            return 3300;
        }
        return (self.0 as u16 + 10) * 100;
    }
}

pub enum ModeControl {
    Auto,
    PWM,
    PFM,
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
}

pub enum OnOffControl {
    Vset,
    Software,
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
}

pub enum RetentionControl {
    Off,
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
}

struct BuckConfig {
    pub enabled: bool,
    pub auto_cap_discharge: bool,
    pub output_voltage: Voltage,
    pub retention_voltage: Voltage,
    pub retention_control: RetentionControl,
    pub on_off_control: OnOffControl,
    pub mode_control: ModeControl,
}

impl Default for BuckConfig {
    fn default() -> Self {
        BuckConfig {
            enabled: false,
            auto_cap_discharge: false,
            output_voltage: Voltage(0),
            retention_voltage: Voltage(0),
            retention_control: RetentionControl::Off,
            on_off_control: OnOffControl::Vset,
            mode_control: ModeControl::Auto,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_millivolts() {
        assert_eq!(Voltage::from_millivolts(0), Voltage(0));
        assert_eq!(Voltage::from_millivolts(100), Voltage(0));
        assert_eq!(Voltage::from_millivolts(1000), Voltage(0));
        assert_eq!(Voltage::from_millivolts(1100), Voltage(1));
        assert_eq!(Voltage::from_millivolts(3000), Voltage(20));
        assert_eq!(Voltage::from_millivolts(3300), Voltage(23));
        assert_eq!(Voltage::from_millivolts(3400), Voltage(24));
        assert_eq!(Voltage::from_millivolts(25500), Voltage(24));
        assert_eq!(Voltage::from_millivolts(25600), Voltage(24));
    }

    #[test]
    fn test_to_millivolts() {
        assert_eq!(Voltage(0).to_millivolts(), 0);
        assert_eq!(Voltage(1).to_millivolts(), 1100);
        assert_eq!(Voltage(20).to_millivolts(), 3000);
        assert_eq!(Voltage(23).to_millivolts(), 3300);
        assert_eq!(Voltage(24).to_millivolts(), 3300);
    }
}
