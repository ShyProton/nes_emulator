use super::{registers::aliases, AddressingMode, Cpu};

mod brk;
mod compare;
mod crement;
mod flags;
mod load;
mod store;
mod transfer;

#[cfg(test)]
mod tests;
