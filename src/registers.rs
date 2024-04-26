#[repr(u16)]
enum Instances {
    MAIN = 0x0000,
    VBUSIN = 0x0200,
    BCHARGER = 0x0300,
    BUCK = 0x0400,
    ADC = 0x0500,
    GPIOS = 0x0600,
    TIMER = 0x0700,
    LDSW = 0x0800,
    POF = 0x0900,
    LEDDRV = 0x0A00,
    SHIP = 0x0B00,
    ERRLOG = 0x0E00,
}

#[repr(u16)]
pub enum SysregRegs {
    TASKUPDATEILIMSW = Instances::VBUSIN as u16 + 0x00,
    VBUSINILIM0 = Instances::VBUSIN as u16 + 0x01,
    VBUSSUSPEND = Instances::VBUSIN as u16 + 0x03,
    USBCDETECTSTATUS = Instances::VBUSIN as u16 + 0x05,
    VBUSINSTATUS = Instances::VBUSIN as u16 + 0x07,
}

#[repr(u16)]
pub enum ChargerRegs {
    TASKRELEASEERR = Instances::BCHARGER as u16 + 0x00,
    TASKCLEARCHGERR = Instances::BCHARGER as u16 + 0x01,
    TASKCLEARSAFETYTIMER = Instances::BCHARGER as u16 + 0x02,
    BCHGENABLESET = Instances::BCHARGER as u16 + 0x04,
    BCHGENABLECLR = Instances::BCHARGER as u16 + 0x05,
    BCHGDISABLESET = Instances::BCHARGER as u16 + 0x06,
    BCHGDISABLECLR = Instances::BCHARGER as u16 + 0x07,
    BCHGISETMSB = Instances::BCHARGER as u16 + 0x08,
    BCHGISETLSB = Instances::BCHARGER as u16 + 0x0009,
    BCHGISETDISCHARGEMSB = Instances::BCHARGER as u16 + 0x0A,
    BCHGISETDISCHARGELSB = Instances::BCHARGER as u16 + 0x0B,
    BCHGVTERM = Instances::BCHARGER as u16 + 0x0C,
    BCHGVTERMR = Instances::BCHARGER as u16 + 0x0D,
    BCHGVTRICKLESEL = Instances::BCHARGER as u16 + 0x0E,
    BCHGITERMSEL = Instances::BCHARGER as u16 + 0x0F,
    NTCCOLD = Instances::BCHARGER as u16 + 0x10,
    NTCCOLDLSB = Instances::BCHARGER as u16 + 0x11,
    NTCCOOL = Instances::BCHARGER as u16 + 0x12,
    NTCCOOLLSB = Instances::BCHARGER as u16 + 0x13,
    NTCWARM = Instances::BCHARGER as u16 + 0x14,
    NTCWARMLSB = Instances::BCHARGER as u16 + 0x15,
    NTCHOT = Instances::BCHARGER as u16 + 0x16,
    NTCHOTLSB = Instances::BCHARGER as u16 + 0x17,
    DIETEMPSTOP = Instances::BCHARGER as u16 + 0x18,
    DIETEMPSTOPLSB = Instances::BCHARGER as u16 + 0x19,
    DIETEMPRESUME = Instances::BCHARGER as u16 + 0x1A,
    DIETEMPRESUMELSB = Instances::BCHARGER as u16 + 0x1B,
    BCHGILIMSTATUS = Instances::BCHARGER as u16 + 0x2D,
    NTCSTATUS = Instances::BCHARGER as u16 + 0x32,
    DIETEMPSTATUS = Instances::BCHARGER as u16 + 0x33,
    BCHGCHARGESTATUS = Instances::BCHARGER as u16 + 0x34,
    BCHGERRREASON = Instances::BCHARGER as u16 + 0x36,
    BCHGERRSENSOR = Instances::BCHARGER as u16 + 0x37,
    BCHGCONFIG = Instances::BCHARGER as u16 + 0x3C,
}

#[repr(u16)]
pub enum BuckRegs {
    BUCK1ENASET = Instances::BUCK as u16 + 0x0,
    BUCK1ENACLR = Instances::BUCK as u16 + 0x1,
    BUCK2ENASET = Instances::BUCK as u16 + 0x2,
    BUCK2ENACLR = Instances::BUCK as u16 + 0x3,
    BUCK1PWMSET = Instances::BUCK as u16 + 0x4,
    BUCK1PWMCLR = Instances::BUCK as u16 + 0x5,
    BUCK2PWMSET = Instances::BUCK as u16 + 0x6,
    BUCK2PWMCLR = Instances::BUCK as u16 + 0x7,
    BUCK1NORMVOUT = Instances::BUCK as u16 + 0x8,
    BUCK1RETVOUT = Instances::BUCK as u16 + 0x9,
    BUCK2NORMVOUT = Instances::BUCK as u16 + 0xA,
    BUCK2RETVOUT = Instances::BUCK as u16 + 0xB,
    BUCKENCTRL = Instances::BUCK as u16 + 0xC,
    BUCKVRETCTRL = Instances::BUCK as u16 + 0xD,
    BUCKPWMCTRL = Instances::BUCK as u16 + 0xE,
    BUCKSWCTRLSEL = Instances::BUCK as u16 + 0xF,
    BUCK1VOUTSTATUS = Instances::BUCK as u16 + 0x10,
    BUCK2VOUTSTATUS = Instances::BUCK as u16 + 0x11,
    BUCKCTRL0 = Instances::BUCK as u16 + 0x15,
    BUCKSTATUS = Instances::BUCK as u16 + 0x34,
}

#[repr(u16)]
pub enum LdswRegs {
    TASKLDSW1SET = Instances::LDSW as u16 + 0x0,
    TASKLDSW1CLR = Instances::LDSW as u16 + 0x1,
    TASKLDSW2SET = Instances::LDSW as u16 + 0x2,
    TASKLDSW2CLR = Instances::LDSW as u16 + 0x3,
    LDSWSTATUS = Instances::LDSW as u16 + 0x4,
    LDSW1GPISEL = Instances::LDSW as u16 + 0x5,
    LDSW2GPISEL = Instances::LDSW as u16 + 0x6,
    LDSWCONFIG = Instances::LDSW as u16 + 0x7,
    LDSW1LDOSEL = Instances::LDSW as u16 + 0x8,
    LDSW2LDOSEL = Instances::LDSW as u16 + 0x9,
    LDSW1VOUTSEL = Instances::LDSW as u16 + 0xC,
    LDSW2VOUTSEL = Instances::LDSW as u16 + 0xD,
}

#[repr(u16)]
pub enum LedDrvRegs {
    LEDDRV0MODESEL = Instances::LEDDRV as u16 + 0x0,
    LEDDRV1MODESEL = Instances::LEDDRV as u16 + 0x1,
    LEDDRV2MODESEL = Instances::LEDDRV as u16 + 0x2,
    LEDDRV0SET = Instances::LEDDRV as u16 + 0x3,
    LEDDRV0CLR = Instances::LEDDRV as u16 + 0x4,
    LEDDRV1SET = Instances::LEDDRV as u16 + 0x5,
    LEDDRV1CLR = Instances::LEDDRV as u16 + 0x6,
    LEDDRV2SET = Instances::LEDDRV as u16 + 0x7,
    LEDDRV2CLR = Instances::LEDDRV as u16 + 0x8,
}

#[repr(u16)]
pub enum GpioRegs {
    GPIOMODE0 = Instances::GPIOS as u16 + 0x0,
    GPIOMODE1 = Instances::GPIOS as u16 + 0x1,
    GPIOMODE2 = Instances::GPIOS as u16 + 0x2,
    GPIOMODE3 = Instances::GPIOS as u16 + 0x3,
    GPIOMODE4 = Instances::GPIOS as u16 + 0x4,
    GPIODRIVE0 = Instances::GPIOS as u16 + 0x5,
    GPIODRIVE1 = Instances::GPIOS as u16 + 0x6,
    GPIODRIVE2 = Instances::GPIOS as u16 + 0x7,
    GPIODRIVE3 = Instances::GPIOS as u16 + 0x8,
    GPIODRIVE4 = Instances::GPIOS as u16 + 0x9,
    GPIOPUEN0 = Instances::GPIOS as u16 + 0xA,
    GPIOPUEN1 = Instances::GPIOS as u16 + 0xB,
    GPIOPUEN2 = Instances::GPIOS as u16 + 0xC,
    GPIOPUEN3 = Instances::GPIOS as u16 + 0xD,
    GPIOPUEN4 = Instances::GPIOS as u16 + 0xE,
    GPIOPDEN0 = Instances::GPIOS as u16 + 0xF,
    GPIOPDEN1 = Instances::GPIOS as u16 + 0x10,
    GPIOPDEN2 = Instances::GPIOS as u16 + 0x11,
    GPIOPDEN3 = Instances::GPIOS as u16 + 0x12,
    GPIOPDEN4 = Instances::GPIOS as u16 + 0x13,
    GPIOOPENDRAIN0 = Instances::GPIOS as u16 + 0x14,
    GPIOOPENDRAIN1 = Instances::GPIOS as u16 + 0x15,
    GPIOOPENDRAIN2 = Instances::GPIOS as u16 + 0x16,
    GPIOOPENDRAIN3 = Instances::GPIOS as u16 + 0x17,
    GPIOOPENDRAIN4 = Instances::GPIOS as u16 + 0x18,
    GPIODEBOUNCE0 = Instances::GPIOS as u16 + 0x19,
    GPIODEBOUNCE1 = Instances::GPIOS as u16 + 0x1A,
    GPIODEBOUNCE2 = Instances::GPIOS as u16 + 0x1B,
    GPIODEBOUNCE3 = Instances::GPIOS as u16 + 0x1C,
    GPIODEBOUNCE4 = Instances::GPIOS as u16 + 0x1D,
    GPIOSTATUS = Instances::GPIOS as u16 + 0x1E,
}

#[repr(u16)]
pub enum AdcRegs {
    TASKVBATMEASURE = Instances::ADC as u16 + 0x0,
    TASKNTCMEASURE = Instances::ADC as u16 + 0x1,
    TASKTEMPMEASURE = Instances::ADC as u16 + 0x2,
    TASKVSYSMEASURE = Instances::ADC as u16 + 0x3,
    TASKIBATMEASURE = Instances::ADC as u16 + 0x6,
    TASKVBUS7MEASURE = Instances::ADC as u16 + 0x7,
    TASKDELAYEDVBATMEASURE = Instances::ADC as u16 + 0x8,
    ADCCONFIG = Instances::ADC as u16 + 0x9,
    ADCNTCRSEL = Instances::ADC as u16 + 0xA,
    ADCAUTOTIMCONF = Instances::ADC as u16 + 0xB,
    TASKAUTOTIMUPDATE = Instances::ADC as u16 + 0xC,
    ADCDELTIMCONF = Instances::ADC as u16 + 0xD,
    ADCIBATMEASSTATUS = Instances::ADC as u16 + 0x10,
    ADCVBATRESULTMSB = Instances::ADC as u16 + 0x11,
    ADCNTCRESULTMSB = Instances::ADC as u16 + 0x12,
    ADCTEMPRESULTMSB = Instances::ADC as u16 + 0x13,
    ADCVSYSRESULTMSB = Instances::ADC as u16 + 0x14,
    ADCGP0RESULTLSBS = Instances::ADC as u16 + 0x15,
    ADCVBAT0RESULTMSB = Instances::ADC as u16 + 0x16,
    ADCVBAT1RESULTMSB = Instances::ADC as u16 + 0x17,
    ADCVBAT2RESULTMSB = Instances::ADC as u16 + 0x18,
    ADCVBAT3RESULTMSB = Instances::ADC as u16 + 0x19,
    ADCGP1RESULTLSBS = Instances::ADC as u16 + 0x1A,
    ADCIBATMEASEN = Instances::ADC as u16 + 0x24,
}

#[repr(u16)]
pub enum PofRegs {
    POFCONFIG = Instances::POF as u16 + 0x00,
}

#[repr(u16)]
pub enum TimerRegs {
    TIMERSET = Instances::TIMER as u16 + 0x0,
    TIMERCLR = Instances::TIMER as u16 + 0x1,
    TIMERTARGETSTROBE = Instances::TIMER as u16 + 0x3,
    WATCHDOGKICK = Instances::TIMER as u16 + 0x4,
    TIMERCONFIG = Instances::TIMER as u16 + 0x5,
    TIMERSTATUS = Instances::TIMER as u16 + 0x6,
    TIMERHIBYTE = Instances::TIMER as u16 + 0x8,
    TIMERMIDBYTE = Instances::TIMER as u16 + 0x9,
    TIMERLOBYTE = Instances::TIMER as u16 + 0xA,
}

#[repr(u16)]
pub enum ShipRegs {
    TASKENTERHIBERNATE = Instances::SHIP as u16 + 0x0,
    TASKSHPHLDCFGSTROBE = Instances::SHIP as u16 + 0x1,
    TASKENTERSHIPMODE = Instances::SHIP as u16 + 0x2,
    TASKRESETCFG = Instances::SHIP as u16 + 0x3,
    SHPHLDCONFIG = Instances::SHIP as u16 + 0x4,
    SHPHLDSTATUS = Instances::SHIP as u16 + 0x5,
    LPRESETCONFIG = Instances::SHIP as u16 + 0x6,
}

#[repr(u16)]
pub enum MainRegs {
    TASKSWRESET = Instances::MAIN as u16 + 0x1,
    EVENTSADCSET = Instances::MAIN as u16 + 0x2,
    EVENTSADCCLR = Instances::MAIN as u16 + 0x3,
    INTENEVENTSADCSET = Instances::MAIN as u16 + 0x4,
    INTENEVENTSADCCLR = Instances::MAIN as u16 + 0x5,
    EVENTSBCHARGER0SET = Instances::MAIN as u16 + 0x6,
    EVENTSBCHARGER0CLR = Instances::MAIN as u16 + 0x7,
    INTENEVENTSBCHARGER0SET = Instances::MAIN as u16 + 0x8,
    INTENEVENTSBCHARGER0CLR = Instances::MAIN as u16 + 0x9,
    EVENTSBCHARGER1SET = Instances::MAIN as u16 + 0xA,
    EVENTSBCHARGER1CLR = Instances::MAIN as u16 + 0xB,
    INTENEVENTSBCHARGER1SET = Instances::MAIN as u16 + 0xC,
    INTENEVENTSBCHARGER1CLR = Instances::MAIN as u16 + 0xD,
    EVENTSBCHARGER2SET = Instances::MAIN as u16 + 0xE,
    EVENTSBCHARGER2CLR = Instances::MAIN as u16 + 0xF,
    INTENEVENTSBCHARGER2SET = Instances::MAIN as u16 + 0x10,
    INTENEVENTSBCHARGER2CLR = Instances::MAIN as u16 + 0x11,
    EVENTSSHPHLDSET = Instances::MAIN as u16 + 0x12,
    EVENTSSHPHLDCLR = Instances::MAIN as u16 + 0x13,
    INTENEVENTSSHPHLDSET = Instances::MAIN as u16 + 0x14,
    INTENEVENTSSHPHLDCLR = Instances::MAIN as u16 + 0x15,
    EVENTSVBUSIN0SET = Instances::MAIN as u16 + 0x16,
    EVENTSVBUSIN0CLR = Instances::MAIN as u16 + 0x17,
    INTENEVENTSVBUSIN0SET = Instances::MAIN as u16 + 0x18,
    INTENEVENTSVBUSIN0CLR = Instances::MAIN as u16 + 0x19,
    EVENTSVBUSIN1SET = Instances::MAIN as u16 + 0x1A,
    EVENTSVBUSIN1CLR = Instances::MAIN as u16 + 0x1B,
    INTENEVENTSVBUSIN1SET = Instances::MAIN as u16 + 0x1C,
    INTENEVENTSVBUSIN1CLR = Instances::MAIN as u16 + 0x1D,
    EVENTSGPIOSET = Instances::MAIN as u16 + 0x22,
    EVENTSGPIOCLR = Instances::MAIN as u16 + 0x23,
    INTENEVENTSGPIOSET = Instances::MAIN as u16 + 0x24,
    INTENEVENTSGPIOCLR = Instances::MAIN as u16 + 0x25,
}

#[repr(u16)]
pub enum ErrlogReg {
    TASKCLRERRLOG = Instances::ERRLOG as u16 + 0x0,
    SCRATCH0 = Instances::ERRLOG as u16 + 0x1,
    SCRATCH1 = Instances::ERRLOG as u16 + 0x2,
    RSTCAUSE = Instances::ERRLOG as u16 + 0x3,
    CHARGERERRREASON = Instances::ERRLOG as u16 + 0x4,
    CHARGERERRSENSOR = Instances::ERRLOG as u16 + 0x5,
}
