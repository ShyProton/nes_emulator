use super::aliases::StatusFlagAlias;

pub struct Status {
    status: [bool; 8],
}

impl Status {
    pub const fn new() -> Self {
        Self { status: [false; 8] }
    }

    pub fn set_flag(&mut self, flag: StatusFlagAlias, setting: bool) -> &mut Self {
        self.status[flag.index()] = setting;
        self
    }

    pub const fn get_flag(&self, flag: StatusFlagAlias) -> bool {
        self.status[flag.index()]
    }

    pub fn set_byte(&mut self, byte: u8) {
        for i in 0..8 {
            self.status[i] = (byte & (0b0000_0001 << i)) != 0;
        }
    }

    pub fn get_byte(&self) -> u8 {
        let mut byte = 0;

        for (i, &bit) in self.status.iter().enumerate() {
            if bit {
                byte |= 0b0000_0001 << i;
            }
        }

        byte
    }

    pub fn reset_flags(&mut self) {
        self.status = [false; 8];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use StatusFlagAlias::{B, C, D, I, N, V};

    #[test]
    fn flag_settings() {
        let mut status = Status::new();

        let aliases = [C, N, I, D, B, V, N];

        for flag in aliases {
            status.reset_flags();

            status.set_flag(flag, true);
            assert!(status.get_flag(flag));

            status.set_flag(flag, false);
            assert!(!status.get_flag(flag));
        }
    }
}
