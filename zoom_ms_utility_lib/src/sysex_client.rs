/* sysex_client.rs
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

use std::{
    error::Error,
    time::{Duration, Instant},
};

use crate::midi::MidiTransport;

pub struct SysExClient {
    pub transport: MidiTransport,
}

impl SysExClient {
    pub fn new(transport: MidiTransport) -> Self {
        Self { transport }
    }

    pub fn flush(&mut self) {
        while self.transport.recv().is_some() {}
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.transport.send(data)?;
        Ok(())
    }

    pub fn recv(&mut self) -> Option<Vec<u8>> {
        self.transport.recv()
    }

    /// Send a command and wait for a response with a specific command code
    /// Send a command and wait for a response that matches a predicate
    pub fn send_and_wait<F>(
        &mut self,
        request: &[u8],
        timeout_ms: u64,
        response_matcher: F,
    ) -> Result<Vec<u8>, Box<dyn Error>>
    where
        F: Fn(&[u8]) -> bool,
    {
        self.flush();
        self.send(request)?;

        let start = Instant::now();
        let mut buf = Vec::new();

        while start.elapsed() < Duration::from_millis(timeout_ms) {
            if let Some(msg) = self.recv() {
                buf.extend_from_slice(&msg);
                if let Some(f0) = buf.iter().position(|&b| b == 0xF0) {
                    if let Some(rel) = buf[f0..].iter().position(|&b| b == 0xF7) {
                        let frame = &buf[f0..=f0 + rel];
                        if response_matcher(frame) {
                            return Ok(frame.to_vec());
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(8));
        }
        Err("Timeout waiting for response".into())
    }

    fn is_response(frame: &[u8], expected_command: u8) -> bool {
        frame.len() >= 6 && frame[0] == 0xF0 && frame[1] == 0x52 && frame[4] == expected_command
    }
}
