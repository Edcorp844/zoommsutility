/* components/footswitch.rs
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

use gtk::cairo;
use gtk::prelude::*;
use log::info;
use relm4::prelude::*;
use relm4::ComponentParts;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct FootswitchModel {
    pub is_active: bool,
    pub size: i32,
}

#[derive(Debug, Clone)]
pub enum FootswitchInput {
    /// Silent hardware / parent sync – no output
    SetActive(bool),
    /// User press – toggles and emits
    Toggle,
}

#[derive(Debug, Clone)]
pub enum FootswitchOutput {
    StateChanged(bool),
    /// After user toggles – parent can commit live buffer
    Released,
}

#[relm4::component(pub)]
impl SimpleComponent for FootswitchModel {
    type Init = (bool, i32);
    type Input = FootswitchInput;
    type Output = FootswitchOutput;

    view! {
        gtk::DrawingArea {
            set_content_width: model.size,
            set_content_height: model.size,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            set_focusable: true,
            set_can_target: true,

            add_controller = gtk::GestureClick {
                set_button: 0,
                connect_pressed[sender] => move |gesture, _, _, _| {
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                    sender.input(FootswitchInput::Toggle);
                },
            },

            #[watch]
            set_draw_func: {
                let is_active = model.is_active;
                move |_, ctx, width, height| {
                    let w = width as f64;
                    let h = height as f64;
                    let cx = w / 2.0;
                    let cy = h / 2.0;
                    let r = (w.min(h) / 2.0) - 1.5;

                    // Soft drop shadow under the stomp
                    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.45);
                    ctx.arc(cx, cy + 1.5, r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    // Outer metal ring (stomp housing)
                    let ring_grad = cairo::RadialGradient::new(
                        cx - r * 0.25,
                        cy - r * 0.3,
                        r * 0.1,
                        cx,
                        cy,
                        r,
                    );
                    ring_grad.add_color_stop_rgb(0.0, 0.42, 0.44, 0.48);
                    ring_grad.add_color_stop_rgb(0.55, 0.28, 0.30, 0.33);
                    ring_grad.add_color_stop_rgb(1.0, 0.16, 0.17, 0.19);
                    let _ = ctx.set_source(&ring_grad);
                    ctx.arc(cx, cy, r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    // Outer rim highlight
                    ctx.set_source_rgba(1.0, 1.0, 1.0, 0.18);
                    ctx.set_line_width(1.2);
                    ctx.arc(cx, cy, r - 0.6, 0.0, 2.0 * PI);
                    let _ = ctx.stroke();

                    // Recessed well
                    let well_r = r * 0.72;
                    ctx.set_source_rgb(0.08, 0.09, 0.10);
                    ctx.arc(cx, cy, well_r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.7);
                    ctx.set_line_width(1.0);
                    ctx.arc(cx, cy, well_r - 0.4, 0.0, 2.0 * PI);
                    let _ = ctx.stroke();

                    // Plunger (slightly inset when active to feel “pressed”)
                    let press = if is_active { 1.0 } else { 0.0 };
                    let plunger_r = well_r * 0.78;
                    let plunger_cy = cy + press * 0.8;

                    let plunger_grad = cairo::RadialGradient::new(
                        cx - plunger_r * 0.25,
                        plunger_cy - plunger_r * 0.35,
                        plunger_r * 0.08,
                        cx,
                        plunger_cy,
                        plunger_r,
                    );
                    if is_active {
                        plunger_grad.add_color_stop_rgb(0.0, 0.55, 0.58, 0.62);
                        plunger_grad.add_color_stop_rgb(0.7, 0.38, 0.40, 0.44);
                        plunger_grad.add_color_stop_rgb(1.0, 0.22, 0.24, 0.27);
                    } else {
                        plunger_grad.add_color_stop_rgb(0.0, 0.72, 0.75, 0.80);
                        plunger_grad.add_color_stop_rgb(0.7, 0.52, 0.55, 0.60);
                        plunger_grad.add_color_stop_rgb(1.0, 0.32, 0.34, 0.38);
                    }
                    let _ = ctx.set_source(&plunger_grad);
                    ctx.arc(cx, plunger_cy, plunger_r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    // Plunger edge
                    ctx.set_source_rgba(1.0, 1.0, 1.0, if is_active { 0.12 } else { 0.22 });
                    ctx.set_line_width(1.0);
                    ctx.arc(cx, plunger_cy, plunger_r - 0.5, 0.0, 2.0 * PI);
                    let _ = ctx.stroke();

                    // Center green LED
                    let led_r = plunger_r * 0.38;
                    let led_cy = plunger_cy;

                    // Dark LED bezel
                    ctx.set_source_rgb(0.12, 0.13, 0.15);
                    ctx.arc(cx, led_cy, led_r + 1.6, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    if is_active {
                        // Soft green bloom
                        let glow_r = led_r * 2.4;
                        let bloom = cairo::RadialGradient::new(
                            cx, led_cy, 0.0, cx, led_cy, glow_r,
                        );
                        bloom.add_color_stop_rgba(0.0, 0.15, 0.95, 0.35, 0.55);
                        bloom.add_color_stop_rgba(0.45, 0.05, 0.70, 0.20, 0.20);
                        bloom.add_color_stop_rgba(1.0, 0.0, 0.0, 0.0, 0.0);
                        let _ = ctx.set_source(&bloom);
                        ctx.arc(cx, led_cy, glow_r, 0.0, 2.0 * PI);
                        let _ = ctx.fill();

                        // Bright green lens
                        let lens = cairo::RadialGradient::new(
                            cx - led_r * 0.3,
                            led_cy - led_r * 0.35,
                            led_r * 0.1,
                            cx,
                            led_cy,
                            led_r,
                        );
                        lens.add_color_stop_rgb(0.0, 0.70, 1.0, 0.55);
                        lens.add_color_stop_rgb(0.55, 0.05, 0.90, 0.28);
                        lens.add_color_stop_rgb(1.0, 0.02, 0.45, 0.12);
                        let _ = ctx.set_source(&lens);
                        ctx.arc(cx, led_cy, led_r, 0.0, 2.0 * PI);
                        let _ = ctx.fill();

                        // Specular highlight on glass
                        ctx.set_source_rgba(1.0, 1.0, 1.0, 0.85);
                        ctx.arc(
                            cx - led_r * 0.32,
                            led_cy - led_r * 0.32,
                            led_r * 0.28,
                            0.0,
                            2.0 * PI,
                        );
                        let _ = ctx.fill();
                    } else {
                        // Dark “off” glass
                        let lens = cairo::RadialGradient::new(
                            cx - led_r * 0.3,
                            led_cy - led_r * 0.35,
                            led_r * 0.1,
                            cx,
                            led_cy,
                            led_r,
                        );
                        lens.add_color_stop_rgb(0.0, 0.18, 0.22, 0.18);
                        lens.add_color_stop_rgb(1.0, 0.06, 0.08, 0.07);
                        let _ = ctx.set_source(&lens);
                        ctx.arc(cx, led_cy, led_r, 0.0, 2.0 * PI);
                        let _ = ctx.fill();

                        ctx.set_source_rgba(1.0, 1.0, 1.0, 0.25);
                        ctx.arc(
                            cx - led_r * 0.32,
                            led_cy - led_r * 0.32,
                            led_r * 0.28,
                            0.0,
                            2.0 * PI,
                        );
                        let _ = ctx.fill();
                    }
                }
            }
        }
    }

    fn init(
        (is_active, size): Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { is_active, size };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            FootswitchInput::SetActive(active) => {
                // Hardware / parent sync – silent
                info!("Footswitch hardware sync: {active}");
                self.is_active = active;
            }
            FootswitchInput::Toggle => {
                self.is_active = !self.is_active;
                info!("Footswitch toggled: {}", self.is_active);
                let _ = sender.output(FootswitchOutput::StateChanged(self.is_active));
                let _ = sender.output(FootswitchOutput::Released);
            }
        }
    }
}
