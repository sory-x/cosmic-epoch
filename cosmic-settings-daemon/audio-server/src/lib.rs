// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(not(target_os = "redox"))]
pub mod backend;

#[cfg(not(target_os = "redox"))]
pub mod context;
#[cfg(not(target_os = "redox"))]
pub use context::Context;

#[cfg(not(target_os = "redox"))]
pub mod config;

#[cfg(not(target_os = "redox"))]
pub mod codec;
#[cfg(not(target_os = "redox"))]
pub use codec::EventCodec;

#[cfg(not(target_os = "redox"))]
pub mod server;
#[cfg(not(target_os = "redox"))]
pub use server::Server;

#[cfg(target_os = "redox")]
mod stub;
#[cfg(target_os = "redox")]
pub use stub::{Context, EventCodec, Server};