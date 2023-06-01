use super::aliases::StatusFlagAlias;

pub struct Status {
    status: [bool; 8],
}

impl Status {
    pub const fn new() -> Self {
        Self { status: [false; 8] }
    }

    pub fn set_flag(&mut self, flag: StatusFlagAlias, setting: bool) {
        self.status[flag.index()] = setting;
    }

    pub const fn get_flag(&self, flag: StatusFlagAlias) -> bool {
        self.status[flag.index()]
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
