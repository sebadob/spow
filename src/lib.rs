// Copyright 2023 Sebastian Dobe <sebastiandobe@mailbox.org>

#![doc = include_str!("../README.md")]

#[cfg(not(target_arch = "wasm32"))]
pub mod pow;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub type BitValue<'a> = (&'a [u8], usize);
