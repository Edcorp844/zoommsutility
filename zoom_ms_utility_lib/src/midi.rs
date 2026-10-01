/* midi.rs - MIDI transport layer for Zoom MS Utility
 *
 * Copyright 2026 Frost
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use midir::{
    MidiInput, MidiInputConnection, MidiInputPort, MidiOutput, MidiOutputConnection, MidiOutputPort,
};
use std::error::Error;
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct MidiTransport {
    _input_conn: Option<MidiInputConnection<()>>,
    output_conn: Option<MidiOutputConnection>,
    rx: Receiver<Vec<u8>>,
    tx: Sender<Vec<u8>>,
    connected_port_name: Option<String>,
}

impl Default for MidiTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl MidiTransport {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            _input_conn: None,
            output_conn: None,
            rx,
            tx,
            connected_port_name: None,
        }
    }

    /// Returns available MIDI input and output port names.
    pub fn list_ports() -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
        let midi_in = MidiInput::new("zoomms-proto-scan-in")?;
        let midi_out = MidiOutput::new("zoomms-proto-scan-out")?;

        let in_ports = midi_in
            .ports()
            .iter()
            .filter_map(|p| midi_in.port_name(p).ok())
            .collect();

        let out_ports = midi_out
            .ports()
            .iter()
            .filter_map(|p| midi_out.port_name(p).ok())
            .collect();

        Ok((in_ports, out_ports))
    }

    /// Attempts to automatically connect to the first available Zoom device.
    pub fn try_connect(&mut self) -> Result<bool, Box<dyn Error>> {
        if self.is_connected() {
            return Ok(true);
        }

        let is_zoom = |name: &str| name.to_ascii_lowercase().contains("zoom");
        match self.connect_by_predicate(is_zoom, "No Zoom MIDI port found") {
            Ok(()) => Ok(true),
            Err(e) if e.to_string().contains("No Zoom MIDI") => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Connects to a specific port by exact name.
    pub fn connect_to_port(&mut self, port_name: &str) -> Result<(), Box<dyn Error>> {
        self.connect_by_predicate(
            |name| name == port_name,
            &format!("MIDI port matching '{}' not found", port_name),
        )
    }

    /// Helper method that handles port discovery, connection, and state updates.
    fn connect_by_predicate<F>(
        &mut self,
        mut predicate: F,
        not_found_msg: &str,
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&str) -> bool,
    {
        self.disconnect();

        let midi_in = MidiInput::new("zoomms-proto-in")?;
        let midi_out = MidiOutput::new("zoomms-proto-out")?;

        // Find input port
        let in_port: MidiInputPort = midi_in
            .ports()
            .into_iter()
            .find(|p| midi_in.port_name(p).map(|n| predicate(&n)).unwrap_or(false))
            .ok_or_else(|| format!("Input {}", not_found_msg))?;

        // Find output port
        let out_port: MidiOutputPort = midi_out
            .ports()
            .into_iter()
            .find(|p| {
                midi_out
                    .port_name(p)
                    .map(|n| predicate(&n))
                    .unwrap_or(false)
            })
            .ok_or_else(|| format!("Output {}", not_found_msg))?;

        let raw_name = midi_in.port_name(&in_port).unwrap_or_default();
        let tx = self.tx.clone();

        let input_conn = midi_in.connect(
            &in_port,
            "zoom-sysex-in",
            move |_timestamp, message, _| {
                let _ = tx.send(message.to_vec());
            },
            (),
        )?;

        let output_conn = midi_out.connect(&out_port, "zoom-sysex-out")?;

        self._input_conn = Some(input_conn);
        self.output_conn = Some(output_conn);
        self.connected_port_name = Some(raw_name);

        Ok(())
    }

    pub fn disconnect(&mut self) {
        self._input_conn = None;
        self.output_conn = None;
        self.connected_port_name = None;
    }

    #[inline]
    pub fn is_connected(&self) -> bool {
        self.output_conn.is_some()
    }

    pub fn connected_port_name(&self) -> Option<&str> {
        self.connected_port_name.as_deref()
    }

    pub fn send(&mut self, sysex_bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        let conn = self
            .output_conn
            .as_mut()
            .ok_or("Not connected to MIDI device")?;
        conn.send(sysex_bytes)?;
        Ok(())
    }

    pub fn recv(&self) -> Option<Vec<u8>> {
        self.rx.try_recv().ok()
    }

    pub fn flush(&mut self) {
        while self.recv().is_some() {}
    }
}
