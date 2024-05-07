#[repr(u8)]
enum InputCurrentLimit {
    MA0500 = 0,
    MA100 = 1,
    UNUSED2 = 2,
    UNUSED3 = 3,
    MA200 = 4,
    MA500 = 5,
    MA600 = 6,
    MA700 = 7,
    MA800 = 8,
    MA900 = 9,
    MA1000 = 10,
    MA1100 = 11,
    MA1200 = 12,
    MA1300 = 13,
    MA1400 = 14,
    MA1500 = 15,
}

impl InputCurrentLimit {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => InputCurrentLimit::MA0500,
            1 => InputCurrentLimit::MA100,
            2 => InputCurrentLimit::UNUSED2,
            3 => InputCurrentLimit::UNUSED3,
            4 => InputCurrentLimit::MA200,
            5 => InputCurrentLimit::MA500,
            6 => InputCurrentLimit::MA600,
            7 => InputCurrentLimit::MA700,
            8 => InputCurrentLimit::MA800,
            9 => InputCurrentLimit::MA900,
            10 => InputCurrentLimit::MA1000,
            11 => InputCurrentLimit::MA1100,
            12 => InputCurrentLimit::MA1200,
            13 => InputCurrentLimit::MA1300,
            14 => InputCurrentLimit::MA1400,
            15 => InputCurrentLimit::MA1500,
            _ => InputCurrentLimit::MA1500,
        }
    }
}

impl From<u8> for InputCurrentLimit {
    fn from(value: u8) -> Self {
        InputCurrentLimit::from_u8(value)
    }
}

#[repr(u8)]
enum UsbCDetect {
    NOCONNECTION = 0,
    DEFAULTUSB = 1,
    HIGHPOWER1A5 = 2,
    HIGHPOWER3A0 = 3,
    INVALID = 255,
}

impl UsbCDetect {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => UsbCDetect::NOCONNECTION,
            1 => UsbCDetect::DEFAULTUSB,
            2 => UsbCDetect::HIGHPOWER1A5,
            3 => UsbCDetect::HIGHPOWER3A0,
            _ => UsbCDetect::INVALID,
        }
    }
}

impl From<u8> for UsbCDetect {
    fn from(value: u8) -> Self {
        UsbCDetect::from_u8(value)
    }
}

struct USBCDetectStatus(u8);

impl USBCDetectStatus {
    pub fn usb_c_detect_cc1(&self) -> UsbCDetect {
        UsbCDetect::from_u8(self.0 & 0b0000_0011)
    }
    pub fn usb_c_detect_cc2(&self) -> UsbCDetect {
        UsbCDetect::from_u8((self.0 & 0b0000_1100) >> 2)
    }
}
struct VbusInputStatus(u8);

impl VbusInputStatus {
    pub fn present(self) -> bool {
        self.0 & 0b0000_0001 != 0
    }
    pub fn current_limit_detected(self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub fn overvoltage_protection_active(self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    pub fn undervoltage_protection_active(self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub fn suspendmode_active(self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    pub fn output_active(self) -> bool {
        self.0 & 0b0010_0000 != 0
    }
}

struct SuspendStatus(u8);

impl SuspendStatus {
    pub fn suspend_active(self) -> bool {
        self.0 & 0b0000_0001 != 0
    }
}

struct InputCurrentLimitStatus(u8);

impl InputCurrentLimitStatus {
    pub fn input_current_limit(&self) -> InputCurrentLimit {
        InputCurrentLimit::from_u8(self.0 & 0b0000_1111)
    }

    pub fn set_active(self) {
        todo!("write 0b0000_0001 into TASKUPDATEILIM register")
    }
}
