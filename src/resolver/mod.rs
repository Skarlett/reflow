//! Proxy dns requests based on rules

mod handler;
pub mod client;
mod dnsclient;
mod serve;
mod lookup;

pub use self::serve::serve;
pub use self::lookup::create_resolver;