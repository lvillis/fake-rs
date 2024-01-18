//! # Fake-RS Library
//!
//! `fake-rs` is a Rust library for generating fake data such as random user agents, names, phone numbers, and IP addresses.
//! It's designed to provide a simple and easy-to-use interface for developers to quickly generate mock data for testing,
//! simulation, and other scenarios where random data is needed.
//!
//! ## Features
//!
//! - **User Agent Generation**: Randomly select a user agent from a predefined list.
//! - **Name Generation**: Randomly generate names.
//! - **Phone Number Generation**: Generate properly formatted random phone numbers.
//! - **IP Address Generation**: Generate random IPv4 addresses.
//!
//! ## Usage
//!
//! Each module provides one or more functions to generate specific types of data. Here are some usage examples:
//!
//! ### User Agent Generation
//!
//! ```
//! use fake_rs::get_random_user_agent;
//! let user_agent = get_random_user_agent();
//! ```
//!
//! ### Name Generation
//!
//! ```
//! use fake_rs::generate_name;
//! let name = generate_name();
//! ```
//!
//! ### Phone Number Generation
//!
//! ```
//! use fake_rs::generate_phone;
//! let phone_number = generate_phone();
//! ```
//!
//! ### IP Address Generation
//!
//! ```
//! use fake_rs::generate_ip;
//! let ip_address = generate_ip();
//! ```

mod user_agent;
pub use user_agent::*;
mod user_agent_data;

#[cfg(feature = "name")]
mod name;
#[cfg(feature = "name")]
pub use name::*;

#[cfg(feature = "phone")]
mod phone;
#[cfg(feature = "phone")]
pub use phone::*;

#[cfg(feature = "ip")]
mod ip;

#[cfg(feature = "ip")]
pub use ip::*;
