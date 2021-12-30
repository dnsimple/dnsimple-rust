//! Rust DNSimple API client
//!
//! This crate contains the Rust client for the DNSimple API.
//! You will find all the needed endpoints of the v2 of the
//! DNSimple API covered in here.
//!
//! # Layout
//! At the top level we have the `Client` struct which will be
//! your entrypoint to the rest of the API as documented in each
//! of the endpoints.
pub mod dnsimple;
pub mod errors;
