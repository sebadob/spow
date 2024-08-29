// Copyright 2023 Sebastian Dobe <sebastiandobe@mailbox.org>

#![doc = include_str!("../README.md")]

#[cfg(feature = "server")]
pub mod pow;
#[cfg(feature = "client")]
pub mod wasm;

pub type BitValue<'a> = (&'a [u8], usize);
