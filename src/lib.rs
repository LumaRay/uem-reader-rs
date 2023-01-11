#![doc = include_str!("../README.md")]

#![crate_type = "lib"]
#![crate_name = "uem_reader"]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod errors;
mod helpers;
pub mod reader;
pub mod commands;
