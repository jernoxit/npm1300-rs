#[derive(PartialEq, Debug)]
struct ChargeCurrent(u16);

impl ChargeCurrent {
    pub fn from_milliamps(current: u16) -> Self {
        let mut c: u16 = current;
        if c < 32 {
            c = 32;
        } else if c > 800 {
            c = 800;
        }
        let msb: u8 = (c / 4) as u8;
        let lsb: u8 = (c / 2) as u8 & 0b0000_0001;

        ChargeCurrent((msb as u16) << 8 | lsb as u16)
    }

    pub fn to_milliamps(&self) -> u16 {
        let msb = self.0 >> 8;
        let lsb = self.0 & 0x0001;
        msb * 4 + lsb * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charge_current() {
        let c = ChargeCurrent::from_milliamps(32);
        assert_eq!(c, ChargeCurrent(2048));
        assert_eq!(c.to_milliamps(), 32);

        let c = ChargeCurrent::from_milliamps(34);
        assert_eq!(c, ChargeCurrent(2049));
        assert_eq!(c.to_milliamps(), 34);

        let c = ChargeCurrent::from_milliamps(800);
        assert_eq!(c.to_milliamps(), 800);

        let c = ChargeCurrent::from_milliamps(33);
        assert_eq!(c.to_milliamps(), 32);

        let c = ChargeCurrent::from_milliamps(799);
        assert_eq!(c.to_milliamps(), 798);
    }
}
