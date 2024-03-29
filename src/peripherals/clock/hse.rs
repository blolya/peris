use super::super::super::core::rcc::Rcc;

pub struct Hse {
    rcc: Rcc,
    input_frequency: u32,
}
impl Hse {
    pub fn new() -> Hse {
        Hse {
            rcc: Rcc::new(),
            input_frequency: 8,
        }
    }
    pub fn enable(&self) {
        self.rcc.enable_hse();
        let mut hse_ready_status = self.rcc.get_hse_ready_status();

        let mut cycles = 0;
        while hse_ready_status == 0 {
            hse_ready_status = self.rcc.get_hse_ready_status();
    
            cycles += 1;
            if cycles > 100 {
                panic!("Can't enable Hse");
            }
        }
    } 
    pub fn disable(&self) {
        self.rcc.disable_hse();
        let mut hse_ready_status = self.rcc.get_hse_ready_status();

        let mut cycles = 0;
        while hse_ready_status == 1 {
            hse_ready_status = self.rcc.get_hse_ready_status();

            cycles += 1;
            if cycles > 100 {
                panic!("Can't disable Hse");
            }
        }
    }
    pub fn get_input_frequency(&self) -> u32 {
        self.input_frequency
    }
    pub fn get_output_frequency(&self) -> u32 {
        self.input_frequency
    }
}