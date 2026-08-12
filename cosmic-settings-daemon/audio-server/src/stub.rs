// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Redox stub for the audio server.
//!
//! PipeWire is not available on Redox (its bindings require Linux headers and
//! sized unions that Redox does not provide). This module keeps the exact same
//! public API as the PipeWire-backed implementation so that the varlink server
//! and the settings daemon compile unchanged, but every operation reports that
//! no active audio device exists.

use cosmic_settings_audio_core::{Error, Mute, Node, Volume};
use std::os::fd::OwnedFd;
use tokio::sync::mpsc;

/// Internal message type that keeps the `Context::new` / `Context::run`
/// signatures identical to the PipeWire implementation.
pub enum Message {}

#[derive(Clone)]
pub struct Context {
    sender: mpsc::UnboundedSender<Message>,
}

impl Context {
    pub async fn new() -> (Self, mpsc::UnboundedReceiver<Message>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Context { sender: tx }, rx)
    }

    pub async fn run(self, mut rx: mpsc::UnboundedReceiver<Message>) {
        while rx.recv().await.is_some() {}
        futures_util::future::pending().await
    }
}

pub struct Server {
    _context: Context,
}

impl Server {
    pub async fn new(context: Context) -> Self {
        Server { _context: context }
    }

    pub async fn recv_events(&mut self) -> Result<OwnedFd, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn default_sink(&self) -> Option<Node> {
        None
    }

    pub async fn default_source(&self) -> Option<Node> {
        None
    }

    pub async fn set_default(&mut self, _node_id: u32, _save: bool) -> Result<(), Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn select_headphone_profile(&mut self, _device_id: u32) -> Result<(), Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn select_headset_profile(&mut self, _device_id: u32) -> Result<(), Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn set_profile(
        &mut self,
        _device_id: u32,
        _profile_index: u32,
        _save: bool,
    ) -> Result<(), Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn set_route(
        &mut self,
        _device_id: u32,
        _card_profile_device: u32,
        _route_index: u32,
        _save: bool,
    ) -> Result<(), Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn set_mono(&mut self, _enabled: bool) -> Result<(), Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn source_mute_toggle(&mut self) -> Result<Mute, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn source_volume_lower(&mut self) -> Result<Volume, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn source_volume_raise(&mut self) -> Result<Volume, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn sink_mute_toggle(&mut self) -> Result<Mute, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn sink_volume_lower(&mut self) -> Result<Volume, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn sink_volume_raise(&mut self) -> Result<Volume, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn set_sink_volume(&mut self, _volume: u32) -> Result<Volume, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn set_source_volume(&mut self, _volume: u32) -> Result<Volume, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn set_node_mute(&mut self, _node_id: u32, _mute: bool) -> Result<Mute, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn set_node_volume(&mut self, _node_id: u32, _volume: u32) -> Result<Volume, Error> {
        Err(Error::NoActiveSink)
    }

    pub async fn set_node_volume_balance(
        &mut self,
        _node_id: u32,
        _balance: Option<f32>,
    ) -> Result<Volume, Error> {
        Err(Error::NoActiveSink)
    }
}

/// Codec stub. Nothing outside of this crate references it, its presence only
/// preserves the module surface of the PipeWire build.
pub struct EventCodec;