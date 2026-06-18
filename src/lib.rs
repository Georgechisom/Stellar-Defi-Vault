#![no_std]

mod admin;
mod balance;
mod storage;
mod vault;
mod errors;
mod events;

pub use vault::VaultContract;

#[cfg(test)]
mod test;
