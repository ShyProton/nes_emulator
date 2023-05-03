use super::Register;

pub struct Status {
    status: u8,
}

impl Status {
    const FLAGS: [char; 8] = ['N', 'V', ' ', 'B', 'D', 'I', 'Z', 'C'];

    pub const fn new() -> Self {
        Self {
            status: 0b0000_0000,
        }
    }

    pub fn set(&mut self, flag: char, setting: bool) {
        let shift = Self::flag_pos(flag);
        let mask = 0b0000_0001 << shift;

        if setting {
            self.status |= mask;
        } else {
            self.status &= !mask;
        }
    }

    pub fn get(&self, flag: char) -> bool {
        let position = Self::flag_pos(flag);
        self.status.bit_is_set(position)
    }

    pub fn reset_flags(&mut self) {
        self.status = 0b0000_0000;
    }

    fn flag_pos(flag: char) -> usize {
        Self::FLAGS
            .iter()
            .rev()
            .position(|&val| val == flag)
            .map_or_else(|| panic!("Invalid status flag"), |pos| pos)
    }
}
