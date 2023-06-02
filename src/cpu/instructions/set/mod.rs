use super::{registers::aliases, AddressingMode, Cpu};

mod brk;
mod increment;
mod load;
mod store;
mod transfer;

#[cfg(test)]
mod tests;
