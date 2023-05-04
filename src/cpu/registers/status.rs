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

    pub fn set_flag(&mut self, flag: char, setting: bool) {
        let shift = Self::flag_pos(flag);
        let mask = 0b0000_0001 << shift;

        if setting {
            self.status |= mask;
        } else {
            self.status &= !mask;
        }
    }

    pub fn get_flag(&self, flag: char) -> bool {
        let position = Self::flag_pos(flag);
        self.status.get_nth_bit(position)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_settings() {
        let mut status = Status::new();

        for flag in Status::FLAGS {
            status.reset_flags();

            status.set_flag(flag, true);
            assert!(status.get_flag(flag));

            status.set_flag(flag, false);
            assert!(!status.get_flag(flag));
        }
    }

    #[test]
    #[should_panic(expected = "Invalid status flag")]
    fn nonexistent_flag_setting() {
        let mut status = Status::new();

        // The 'A' status flag does not exist.
        status.set_flag('A', true);
    }
}
