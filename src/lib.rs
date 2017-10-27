//! API bindings for Faktory workers and job producers.
//!
//! This crate provides API bindings for the language-agnostic
//! [Faktory](https://github.com/contribsys/faktory) work server.
//!
//! # Producing jobs
//!
//! If you want to **submit** jobs to Faktory, use `Producer`.
//!
//! ```no_run
//! # use faktory::{Producer, Job};
//! use std::net::TcpStream;
//! let mut p = Producer::connect_env::<TcpStream>().unwrap();
//! p.enqueue(Job::new("foobar", vec!["z"])).unwrap();
//! ```
//!
//! # Consuming jobs (i.e., workers)
//!
//! If you want to **accept** jobs from Faktory, use `Consumer`.
//!
//! ```no_run
//! # use faktory::ConsumerBuilder;
//! use std::io;
//! use std::net::TcpStream;
//! let mut c = ConsumerBuilder::default().connect_env::<TcpStream, _>().unwrap();
//! c.register("foobar", |job| -> io::Result<()> {
//!     println!("{:?}", job);
//!     Ok(())
//! });
//! if let Err(e) = c.run(&["default"]) {
//!     println!("worker failed: {}", e);
//! }
//! ```
#![deny(missing_docs)]

extern crate atomic_option;
extern crate bufstream;
extern crate chrono;
extern crate hostname;
extern crate libc;
extern crate native_tls;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;
extern crate url;

mod producer;
mod consumer;
mod proto;

pub use consumer::{Consumer, ConsumerBuilder};
pub use producer::Producer;
pub use proto::Job;
pub use proto::{FromUrl, StreamConnector};
