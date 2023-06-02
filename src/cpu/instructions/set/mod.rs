use super::{registers::aliases, AddressingMode, Cpu};

mod brk;
mod crement;
mod load;
mod store;
mod transfer;

#[cfg(test)]
mod tests;
