#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

pub mod api;
pub mod common;
pub mod config;
pub mod ledger;
pub mod pool;
pub mod state_proof;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn dummy() {
        assert!(true, "Dummy check!");
    }
}
