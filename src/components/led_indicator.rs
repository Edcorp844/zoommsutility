/* components/led_indicator.rs
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
use relm4::prelude::*;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedState {
    Off,
    Ok,
    Caution,
    Critical,
}

#[derive(Debug)]
pub struct LedIndicatorModel {
    pub state: LedState,
    pub size: i32,
}

#[derive(Debug, Clone)]
pub enum LedIndicatorInput {
    SetState(LedState),
    /// Convenience: true → Ok, false → Off
    SetOn(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for LedIndicatorModel {
    /// Prefer `(LedState, size)`. `(bool, size)`
    type Init = (LedState, i32);
    type Input = LedIndicatorInput;
    type Output = ();

    view! {
        gtk::DrawingArea {
            set_content_width: model.size,
            set_content_height: model.size,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,

            #[watch]
            set_draw_func: {
                let state = model.state;
                move |_, ctx, width, height| {
                    let cx = width as f64 / 2.0;
                    let cy = height as f64 / 2.0;
                    let r = (width.min(height) as f64 / 2.0) - 1.0;

                    // Metal bezel
                    ctx.set_source_rgb(0.55, 0.57, 0.60);
                    ctx.arc(cx, cy, r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    // Dark well
                    ctx.set_source_rgb(0.10, 0.11, 0.12);
                    ctx.arc(cx, cy, r * 0.72, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    let lens_r = r * 0.58;

                    match state {
                        LedState::Off => {
                            let lens = cairo::RadialGradient::new(
                                cx - lens_r * 0.3,
                                cy - lens_r * 0.35,
                                lens_r * 0.1,
                                cx,
                                cy,
                                lens_r,
                            );
                            lens.add_color_stop_rgb(0.0, 0.16, 0.20, 0.16);
                            lens.add_color_stop_rgb(1.0, 0.05, 0.07, 0.06);
                            let _ = ctx.set_source(&lens);
                            ctx.arc(cx, cy, lens_r, 0.0, 2.0 * PI);
                            let _ = ctx.fill();
                        }

                        LedState::Ok => {
                            // Soft green bloom
                            let bloom =
                                cairo::RadialGradient::new(cx, cy, 0.0, cx, cy, lens_r * 2.2);
                            bloom.add_color_stop_rgba(0.0, 0.20, 0.95, 0.35, 0.55);
                            bloom.add_color_stop_rgba(0.5, 0.05, 0.70, 0.20, 0.18);
                            bloom.add_color_stop_rgba(1.0, 0.00, 0.00, 0.00, 0.00);
                            let _ = ctx.set_source(&bloom);
                            ctx.arc(cx, cy, lens_r * 2.2, 0.0, 2.0 * PI);
                            let _ = ctx.fill();

                            let lens = cairo::RadialGradient::new(
                                cx - lens_r * 0.3,
                                cy - lens_r * 0.35,
                                lens_r * 0.1,
                                cx,
                                cy,
                                lens_r,
                            );
                            lens.add_color_stop_rgb(0.0, 0.70, 1.00, 0.55);
                            lens.add_color_stop_rgb(0.55, 0.05, 0.90, 0.28);
                            lens.add_color_stop_rgb(1.0, 0.02, 0.45, 0.12);
                            let _ = ctx.set_source(&lens);
                            ctx.arc(cx, cy, lens_r, 0.0, 2.0 * PI);
                            let _ = ctx.fill();

                            // Specular
                            ctx.set_source_rgba(1.0, 1.0, 1.0, 0.85);
                            ctx.arc(
                                cx - lens_r * 0.3,
                                cy - lens_r * 0.3,
                                lens_r * 0.28,
                                0.0,
                                2.0 * PI,
                            );
                            let _ = ctx.fill();
                        }

                        LedState::Caution => {
                            // Amber bloom
                            let bloom =
                                cairo::RadialGradient::new(cx, cy, 0.0, cx, cy, lens_r * 2.2);
                            bloom.add_color_stop_rgba(0.0, 1.00, 0.75, 0.15, 0.55);
                            bloom.add_color_stop_rgba(0.5, 0.90, 0.45, 0.05, 0.18);
                            bloom.add_color_stop_rgba(1.0, 0.00, 0.00, 0.00, 0.00);
                            let _ = ctx.set_source(&bloom);
                            ctx.arc(cx, cy, lens_r * 2.2, 0.0, 2.0 * PI);
                            let _ = ctx.fill();

                            let lens = cairo::RadialGradient::new(
                                cx - lens_r * 0.3,
                                cy - lens_r * 0.35,
                                lens_r * 0.1,
                                cx,
                                cy,
                                lens_r,
                            );
                            lens.add_color_stop_rgb(0.0, 1.00, 0.92, 0.45);
                            lens.add_color_stop_rgb(0.55, 0.95, 0.55, 0.08);
                            lens.add_color_stop_rgb(1.0, 0.55, 0.28, 0.02);
                            let _ = ctx.set_source(&lens);
                            ctx.arc(cx, cy, lens_r, 0.0, 2.0 * PI);
                            let _ = ctx.fill();

                            ctx.set_source_rgba(1.0, 1.0, 1.0, 0.80);
                            ctx.arc(
                                cx - lens_r * 0.3,
                                cy - lens_r * 0.3,
                                lens_r * 0.28,
                                0.0,
                                2.0 * PI,
                            );
                            let _ = ctx.fill();
                        }

                        LedState::Critical => {
                            // Red bloom
                            let bloom =
                                cairo::RadialGradient::new(cx, cy, 0.0, cx, cy, lens_r * 2.2);
                            bloom.add_color_stop_rgba(0.0, 1.00, 0.25, 0.20, 0.55);
                            bloom.add_color_stop_rgba(0.5, 0.85, 0.08, 0.05, 0.18);
                            bloom.add_color_stop_rgba(1.0, 0.00, 0.00, 0.00, 0.00);
                            let _ = ctx.set_source(&bloom);
                            ctx.arc(cx, cy, lens_r * 2.2, 0.0, 2.0 * PI);
                            let _ = ctx.fill();

                            let lens = cairo::RadialGradient::new(
                                cx - lens_r * 0.3,
                                cy - lens_r * 0.35,
                                lens_r * 0.1,
                                cx,
                                cy,
                                lens_r,
                            );
                            lens.add_color_stop_rgb(0.0, 1.00, 0.55, 0.45);
                            lens.add_color_stop_rgb(0.55, 0.95, 0.12, 0.08);
                            lens.add_color_stop_rgb(1.0, 0.45, 0.04, 0.03);
                            let _ = ctx.set_source(&lens);
                            ctx.arc(cx, cy, lens_r, 0.0, 2.0 * PI);
                            let _ = ctx.fill();

                            ctx.set_source_rgba(1.0, 1.0, 1.0, 0.80);
                            ctx.arc(
                                cx - lens_r * 0.3,
                                cy - lens_r * 0.3,
                                lens_r * 0.28,
                                0.0,
                                2.0 * PI,
                            );
                            let _ = ctx.fill();
                        }
                    }
                }
            }
        }
    }

    fn init(
        (state, size): Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { state, size };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            LedIndicatorInput::SetOn(on) => {
                self.state = if on { LedState::Ok } else { LedState::Off };
            }
            LedIndicatorInput::SetState(state) => {
                self.state = state;
            }
        }
    }
}
