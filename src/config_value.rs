use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    cpu: Cpu,
    threashold_cpu_temperature_range: ThreasholdCpuTemperatureRange,
    gpu: Gpu,
    threashold_gpu_temperature_range: ThreasholdGpuTemperatureRange,
    coolerboost: Coolerboost,
    battery_charging_threshold: BatteryChargingThreshold,
    fan_mode : FanMode,
    fan_preset: FanPreset,
}

impl Config {
    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }
    pub fn threashold_cpu_temperature_range(&self) -> &ThreasholdCpuTemperatureRange {
        &self.threashold_cpu_temperature_range
    }
    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
    pub fn threashold_gpu_temperature_range(&self) -> &ThreasholdGpuTemperatureRange {
        &self.threashold_gpu_temperature_range
    }
    pub fn coolerboost(&self) -> &Coolerboost {
        &self.coolerboost
    }
    pub fn battery_charging_threshold(&self) -> &BatteryChargingThreshold {
        &self.battery_charging_threshold
    }
    pub fn fan_mode(&self) -> &FanMode {
        &self.fan_mode
    }
    pub fn fan_preset(&self) -> &FanPreset {
        &self.fan_preset
    }
}

#[derive(Deserialize)]
pub struct Cpu {
    realtime_cpu_temperature: u8,
    realtime_cpu_fanspeed: u8,
    realtime_cpu_rpm: u8,
}

impl Cpu {
    pub fn realtime_cpu_temperature(&self) -> u8 {
        self.realtime_cpu_temperature
    }
    pub fn realtime_cpu_fanspeed(&self) -> u8 {
        self.realtime_cpu_fanspeed
    }
    pub fn realtime_cpu_rpm(&self) -> u8 {
        self.realtime_cpu_rpm
    }
}

#[derive(Deserialize)]
pub struct ThreasholdCpuTemperatureRange {
    address_1: u8,
    address_2: u8,
    address_3: u8,
    address_4: u8,
    address_5: u8,
    address_6: u8,
}

impl ThreasholdCpuTemperatureRange {
    pub fn address_1(&self) -> u8 {
        self.address_1
    }
    pub fn address_2(&self) -> u8 {
        self.address_2
    }
    pub fn address_3(&self) -> u8 {
        self.address_3
    }
    pub fn address_4(&self) -> u8 {
        self.address_4
    }
    pub fn address_5(&self) -> u8 {
        self.address_5
    }
    pub fn address_6(&self) -> u8 {
        self.address_6
    }
}

#[derive(Deserialize)]
pub struct Gpu {
    realtime_gpu_temperature: u8,
    realtime_gpu_fanspeed: u8,
    realtime_gpu_rpm: u8,
}

impl Gpu {
    pub fn realtime_gpu_temperature(&self) -> u8 {
        self.realtime_gpu_temperature
    }
    pub fn realtime_gpu_fanspeed(&self) -> u8 {
        self.realtime_gpu_fanspeed
    }
    pub fn realtime_gpu_rpm(&self) -> u8 {
        self.realtime_gpu_rpm
    }
}

#[derive(Deserialize)]
pub struct ThreasholdGpuTemperatureRange {
    address_1: u8,
    address_2: u8,
    address_3: u8,
    address_4: u8,
    address_5: u8,
    address_6: u8,
}

impl ThreasholdGpuTemperatureRange {
    pub fn address_1(&self) -> u8 {
        self.address_1
    }
    pub fn address_2(&self) -> u8 {
        self.address_2
    }
    pub fn address_3(&self) -> u8 {
        self.address_3
    }
    pub fn address_4(&self) -> u8 {
        self.address_4
    }
    pub fn address_5(&self) -> u8 {
        self.address_5
    }
    pub fn address_6(&self) -> u8 {
        self.address_6
    }
}

#[derive(Deserialize)]
pub struct Coolerboost {
    address: u8,
    hex_jump: u8,
}

impl Coolerboost {
    pub fn address(&self) -> u8 {
        self.address
    }
    pub fn hex_jump(&self) -> u8 {
        self.hex_jump
    }
}

#[derive(Deserialize)]
pub struct BatteryChargingThreshold {
    address: u8,
    hex_jump: u8, // battery_saver = BC; balanced = D0; mobility = 128;
}

impl BatteryChargingThreshold {
    pub fn address(&self) -> u8 {
        self.address
    }
    pub fn hex_jump(&self) -> u8 {
        self.hex_jump
    }
}

#[derive(Deserialize)]
pub struct FanPreset {
    address: u8,
    hex_jump: u8, // Advanced or auto
}

impl FanPreset {
    pub fn address(&self) -> u8 {
        self.address
    }
    pub fn hex_jump(&self) -> u8 {
        self.hex_jump
    }
}

#[derive(Deserialize)]
pub struct FanMode {
    address: u8,
    hex_jump: u8, // Low, medium, high
}

impl FanMode {
    pub fn address(&self) -> u8 {
        self.address
    }
    pub fn hex_jump(&self) -> u8 {
        self.hex_jump
    }
}

#[derive(Deserialize)]
pub struct SwitchFnWindows {
    address: u8,
    hex_jump: u8, // 10 yes; 0 no
}

impl SwitchFnWindows {
    pub fn address(&self) -> u8 {
        self.address
    }
    pub fn hex_jump(&self) -> u8 {
        self.hex_jump
    }
}