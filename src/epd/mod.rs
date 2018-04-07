use bcm2835::*;
use bcm2835::gpio::*;
use bcm2835::spio::*;

pub mod doodle;

pub const RST_PIN: Pin = Pin::PI_GPIO_P1_11;
pub const DC_PIN: Pin = Pin::PI_GPIO_P1_22;
pub const CS_PIN: Pin = Pin::PI_GPIO_P1_24;
pub const BUSY_PIN: Pin = Pin::PI_GPIO_P1_18;

pub const WIDTH: usize = 104;
pub const HEIGHT: usize = 212;
pub const FRAME_SIZE: usize = ((WIDTH as usize * HEIGHT as usize) / 8) as usize;

pub struct Frame {
    buffer_black: [u8; FRAME_SIZE],
    buffer_red: [u8; FRAME_SIZE],
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            buffer_black: [0xFF; FRAME_SIZE],
            buffer_red:   [0xFF; FRAME_SIZE],
            }
    }

    pub fn clear(&mut self) {
        self.clear_black();
        self.clear_red();
    }

    pub fn clear_black(&mut self) {
        for byte in self.buffer_black.iter_mut() {
            *byte = 0xFF;
        }
    }

    pub fn clear_red(&mut self) {
        for byte in self.buffer_red.iter_mut() {
            *byte = 0xFF;
        }
    }
}

pub enum Command {
    PanelSetting,
    PowerSetting,
    PowerOff,
    PowerOffSequenceSetting,
    PowerOn,
    PowerOnMeasure,
    BoosterSoftStart,
    DeepSleep,
    DataStartTransmission1,
    DataStop,
    DisplayRefresh,
    DataStartTransmission2,
    VcomLut,
    W2WLut,
    B2WLut,
    W2BLut,
    B2BLut,
    PllControl,
    TemperatureSensorCalibration,
    TemperatureSensorSelection,
    TemperatureSensorWrite,
    TemperatureSensorRead,
    VcomAndDataIntervalSetting,
    LowPowerDetection,
    TconSetting,
    ReselutionSetting,
    GetStatus,
    AutoMeasureVcom,
    ReadVcomValue,
    VcmDcSetting,
    PartialWindow,
    PartialIn,
    PartialOut,
    ProgramMode,
    ActiveProgram,
    ReadOtpData,
    PowerSaving,
}

impl Command {
    fn to_u8(&self) -> u8 {
        match *self {
            Command::PanelSetting => 0x00,
            Command::PowerSetting => 0x01,
            Command::PowerOff => 0x02,
            Command::PowerOffSequenceSetting => 0x03,
            Command::PowerOn => 0x04,
            Command::PowerOnMeasure => 0x05,
            Command::BoosterSoftStart => 0x06,
            Command::DeepSleep => 0x07,
            Command::DataStartTransmission1 => 0x10,
            Command::DataStop => 0x11,
            Command::DisplayRefresh => 0x12,
            Command::DataStartTransmission2 => 0x13,
            Command::VcomLut => 0x20,
            Command::W2WLut => 0x21,
            Command::B2WLut => 0x22,
            Command::W2BLut => 0x23,
            Command::B2BLut => 0x24,
            Command::PllControl => 0x30,
            Command::TemperatureSensorCalibration => 0x40,
            Command::TemperatureSensorSelection => 0x41,
            Command::TemperatureSensorWrite => 0x42,
            Command::TemperatureSensorRead => 0x43,
            Command::VcomAndDataIntervalSetting => 0x50,
            Command::LowPowerDetection => 0x51,
            Command::TconSetting => 0x60,
            Command::ReselutionSetting => 0x61,
            Command::GetStatus => 0x71,
            Command::AutoMeasureVcom => 0x80,
            Command::ReadVcomValue => 0x81,
            Command::VcmDcSetting => 0x82,
            Command::PartialWindow => 0x90,
            Command::PartialIn => 0x91,
            Command::PartialOut => 0x92,
            Command::ProgramMode => 0xA0,
            Command::ActiveProgram => 0xA1,
            Command::ReadOtpData => 0xA2,
            Command::PowerSaving => 0xE3,
        }
    }
}

pub fn epd_init() {
    init();
    function_select(RST_PIN, FunctionSelect::Output);
    function_select(DC_PIN, FunctionSelect::Output);
    function_select(CS_PIN, FunctionSelect::Input);
    
    if let Some(err) = begin().err() {
        panic!(format!("{} {}", err.0, err.1));
    }
    set_bitorder(BitOrder::MSBFirst);
    set_data_mode(Mode::Mode0);
    set_clock_divider(ClockDivider::Divider128);
    chip_select(ChipSelect::CS0);
    set_chip_selct_polarity(ChipSelect::CS0, PinLevel::LOW);

    reset();
    send_command(Command::BoosterSoftStart);
    send_data(0x17);
    send_data(0x17);
    send_data(0x17);
    send_command(Command::PowerOn);
    wait_until_idle();
    send_command(Command::PanelSetting);
    send_data(0x8f);
    send_command(Command::VcomAndDataIntervalSetting);
    send_data(0x37);
    send_command(Command::ReselutionSetting);
    send_data(WIDTH as u8);
    send_data(0x00);
    send_data(HEIGHT as u8);
}

pub fn send_command(com: Command) {
    write(DC_PIN, PinLevel::LOW);
    transfer(com.to_u8());
}

pub fn send_data(data: u8) {
    write(DC_PIN, PinLevel::HIGH);
    transfer(data);
}

pub fn wait_until_idle() {
    while read(BUSY_PIN) == PinLevel::LOW {
        delay(100);
    }
}

pub fn reset() {
    write(RST_PIN, PinLevel::LOW);
    delay(200);
    write(RST_PIN, PinLevel::HIGH);
    delay(200);
}

pub fn sleep() {
    send_command(Command::PowerOff);
    wait_until_idle();
    send_command(Command::DeepSleep);
    send_data(0xA5);
}

pub fn display_color_frame(frame_buffer: &Frame) {
    send_command(Command::DataStartTransmission1);
    delay(2);
    for byte in frame_buffer.buffer_black.iter() {
        send_data(*byte);
    }
    delay(2);
    send_command(Command::DataStartTransmission2);
    delay(2);
    for byte in frame_buffer.buffer_red.iter() {
        send_data(*byte);
    }
    delay(2);
    send_command(Command::DisplayRefresh);
    wait_until_idle();
}

pub fn display_bw_frame(frame_buffer: &Frame) {
        send_command(Command::DataStartTransmission1);
    delay(2);
    for byte in frame_buffer.buffer_black.iter() {
        send_data(*byte);
    }
    delay(2);
    send_command(Command::DisplayRefresh);
    wait_until_idle();
}