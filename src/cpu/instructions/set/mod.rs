use super::{registers::aliases, AddressingMode, Cpu};

mod arithmetic;
mod bit;
mod brk;
mod compare;
mod crement;
mod flags;
mod load;
mod logical;
mod shift;
mod stack;
mod store;
mod transfer;

#[cfg(test)]
mod test_preperations;
