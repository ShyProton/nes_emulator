use super::{aliases::StatusFlagAlias, RegisterByte};

pub struct Status {
    status: u8,
}

impl Status {
    pub const fn new() -> Self {
        Self {
            status: 0b0000_0000,
        }
    }

    pub fn set_flag(&mut self, flag: StatusFlagAlias, setting: bool) {
        let mask = 0b0000_0001 << flag.index();

        if setting {
            self.status |= mask;
        } else {
            self.status &= !mask;
        }
    }

    pub fn get_flag(&self, flag: StatusFlagAlias) -> bool {
        self.status.get_nth_bit(flag.index())
    }

    pub fn reset_flags(&mut self) {
        self.status = 0b0000_0000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use StatusFlagAlias::*;

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
