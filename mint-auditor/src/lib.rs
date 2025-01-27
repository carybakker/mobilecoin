// Copyright (c) 2018-2022 The MobileCoin Foundation

//! A utility for keeping track of token minting and burning.

#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_docs)]

pub mod counters;
pub mod db;
pub mod gnosis;
pub mod http_api;

mod convert;
mod error;
mod mint_tx_nonce;
mod service;

pub use crate::{error::Error, mint_tx_nonce::MintTxNonce, service::MintAuditorService};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
