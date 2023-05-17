use super::{registers::aliases, AddressingMode, Cpu};

mod brk;
mod inx;
mod load;
mod store;
mod transfer;

#[cfg(test)]
mod tests;
