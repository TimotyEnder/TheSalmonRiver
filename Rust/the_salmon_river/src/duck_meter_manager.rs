pub struct DuckMeterManager {
    duck_meter: u8,
    duck_meter_max: u8,
    duck_meter_gain: u8,
    duck_jump_cost: u8,
    duck_meter_loss: u8,
    update_rate_ms: u64,
    next_update_time_ms: u64,
    meter_gain_reset_flag: bool,
    duck_jump_reset_flag: bool,
    duck_reset_flag: bool,
}
impl DuckMeterManager {
    pub fn get_duck_meter(&self) -> u8 {
        self.duck_meter
    }
    pub fn get_max_duck_meter(&self) -> u8 {
        self.duck_meter_max
    }
    pub fn new(max: u8, gain: u8, loss: u8, duck_jump_cost: u8) -> Self {
        Self {
            duck_meter: max,
            duck_meter_gain: gain,
            duck_jump_cost: duck_jump_cost,
            duck_meter_loss: loss,
            update_rate_ms: 100,
            next_update_time_ms: 0,
            duck_meter_max: max,
            duck_jump_reset_flag: false,
            meter_gain_reset_flag: false,
            duck_reset_flag: false,
        }
    }
    pub fn can_duck_jump(&self) -> bool {
        self.duck_jump_cost <= self.duck_meter
    }
    pub fn can_duck(&mut self) -> bool {
        if self.duck_meter == 0 {
            self.duck_reset_flag = true;
        }
        if self.duck_meter > 15 {
            self.duck_reset_flag = false;
        }
        self.duck_meter > 0 && !self.duck_reset_flag
    }
    pub fn get_duck_reset_flag(&self) -> bool {
        self.duck_reset_flag
    }
    pub fn duck_meter_update(&mut self, duck_jumped: bool, ducked: bool, current_time: u64) {
        let mut gain_meter = true;
        if duck_jumped && !self.duck_jump_reset_flag {
            self.duck_meter -= self.duck_jump_cost;
            self.duck_jump_reset_flag = true;
            gain_meter = false;
        }
        if !duck_jumped && self.duck_jump_reset_flag {
            self.duck_jump_reset_flag = false;
        }
        if ducked {
            gain_meter = false;
            if self.next_update_time_ms < current_time {
                self.next_update_time_ms = current_time + self.update_rate_ms;
                self.duck_meter = self.duck_meter.saturating_sub(self.duck_meter_loss);
            }
        }
        if gain_meter {
            if !self.meter_gain_reset_flag {
                self.meter_gain_reset_flag = true;
                self.next_update_time_ms = current_time + self.update_rate_ms;
            }
            if self.next_update_time_ms < current_time {
                self.next_update_time_ms = current_time + self.update_rate_ms;
                self.duck_meter += self.duck_meter_gain;
            }
        } else if !gain_meter {
            self.meter_gain_reset_flag = false;
        }
        self.duck_meter = self.duck_meter.min(self.duck_meter_max);
        self.duck_meter = self.duck_meter.max(0);
    }
}
