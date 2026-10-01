/* components/mixslider.rs
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct MixSliderModel {
    val: f64,
    min: f64,
    max: f64,
    step: f64,
    width: i32,
    height: i32,
    orientation: SliderOrientation,
    is_dragging: bool,
}

#[derive(Debug, Clone)]
pub enum MixSliderInput {
    SyncValue(f64),
    SetValue(f64),
    StartDrag(f64, f64),
    DragTo(f64, f64),
    EndDrag,
}

#[derive(Debug, Clone)]
pub enum MixSliderOutput {
    ValueChanged(f64),
    Released,
}

impl MixSliderModel {
    fn pos_to_value(&self, _x: f64, y: f64) -> f64 {
        let margin_tb = 18.0;
        let track_height = (self.height as f64 - margin_tb * 2.0).max(1.0);
        let clamped_y = y.clamp(margin_tb, self.height as f64 - margin_tb);
        let norm = 1.0 - ((clamped_y - margin_tb) / track_height);
        self.min + norm * (self.max - self.min)
    }

    fn draw_vertical_fader(
        ctx: &cairo::Context,
        width: f64,
        height: f64,
        val: f64,
        min: f64,
        max: f64,
        is_dragging: bool,
    ) {
        let margin_tb = 18.0;
        let track_height = height - (margin_tb * 2.0);
        let track_x = width * 0.50;
        let track_w = 4.5;

        let norm = if max > min {
            ((val - min) / (max - min)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let fill_top_y = height - margin_tb - (norm * track_height);
        let cap_center_y = fill_top_y;

        // ─────────────────────────────────────────────
        // 1. Scale ticks – closer to the track, reduced glow
        // ─────────────────────────────────────────────
        let num_ticks = 11;
        let tick_len_major = width * 0.09;
        let tick_len_minor = width * 0.045;

        // Reduced spacing (was 14.0)
        let left_x = track_x - 9.0;
        let right_x = track_x + 9.0;

        for i in 0..num_ticks {
            let t = i as f64 / (num_ticks - 1) as f64;
            let t_y = margin_tb + ((1.0 - t) * track_height);
            let is_major = i == 0 || i == (num_ticks - 1) || i == (num_ticks / 2);
            let is_active = t <= (norm + 0.008);

            let len = if is_major {
                tick_len_major
            } else {
                tick_len_minor
            };

            if is_active {
                // Very subtle glow (was 0.22)
                ctx.set_source_rgba(1.0, 0.55, 0.05, 0.10);
                ctx.set_line_width(if is_major { 3.0 } else { 2.4 });
                ctx.move_to(left_x - len - 0.5, t_y);
                ctx.line_to(left_x + 1.5, t_y);
                let _ = ctx.stroke();
                ctx.move_to(right_x - 1.5, t_y);
                ctx.line_to(right_x + len + 0.5, t_y);
                let _ = ctx.stroke();

                // Core line
                ctx.set_source_rgba(1.0, 0.62, 0.12, 0.90);
                ctx.set_line_width(if is_major { 1.8 } else { 1.3 });
            } else {
                if is_major {
                    ctx.set_source_rgba(0.50, 0.50, 0.53, 0.70);
                    ctx.set_line_width(1.6);
                } else {
                    ctx.set_source_rgba(0.30, 0.30, 0.33, 0.50);
                    ctx.set_line_width(1.15);
                }
            }

            // Left tick
            ctx.move_to(left_x - len, t_y);
            ctx.line_to(left_x, t_y);
            let _ = ctx.stroke();

            // Right tick
            ctx.move_to(right_x, t_y);
            ctx.line_to(right_x + len, t_y);
            let _ = ctx.stroke();
        }

        // ─────────────────────────────────────────────
        // 2. Track groove
        // ─────────────────────────────────────────────
        ctx.set_source_rgb(0.07, 0.07, 0.08);
        let _ = ctx.rectangle(track_x - track_w / 2.0, margin_tb, track_w, track_height);
        let _ = ctx.fill();

        ctx.set_source_rgba(0.18, 0.18, 0.20, 0.55);
        ctx.set_line_width(1.0);
        ctx.move_to(track_x - track_w / 2.0 + 0.5, margin_tb);
        ctx.line_to(track_x - track_w / 2.0 + 0.5, height - margin_tb);
        let _ = ctx.stroke();

        // ─────────────────────────────────────────────
        // 3. Active fill – reduced glow
        // ─────────────────────────────────────────────
        if norm > 0.001 {
            // Much weaker outer glow (was 0.18)
            ctx.set_source_rgba(1.0, 0.50, 0.05, 0.08);
            let _ = ctx.rectangle(
                track_x - track_w / 2.0 - 2.0,
                fill_top_y,
                track_w + 4.0,
                height - margin_tb - fill_top_y,
            );
            let _ = ctx.fill();

            // Main filled track
            let fill_grad =
                cairo::LinearGradient::new(track_x, fill_top_y, track_x, height - margin_tb);
            fill_grad.add_color_stop_rgb(0.0, 1.0, 0.62, 0.12);
            fill_grad.add_color_stop_rgb(1.0, 0.85, 0.40, 0.05);
            let _ = ctx.set_source(&fill_grad);
            let _ = ctx.rectangle(
                track_x - track_w / 2.0,
                fill_top_y,
                track_w,
                height - margin_tb - fill_top_y,
            );
            let _ = ctx.fill();
        }

        // ─────────────────────────────────────────────
        // 4. Smaller fader cap
        // ─────────────────────────────────────────────
        let cap_w = 22.0; // was 28.0
        let cap_h = 34.0; // was 44.0
        let cap_x = track_x - cap_w / 2.0;
        let cap_y = cap_center_y - cap_h / 2.0;
        let corner_r = 3.0;

        // Drop shadow
        ctx.set_source_rgba(0.0, 0.0, 0.0, 0.40);
        let _ = ctx.rectangle(cap_x + 1.2, cap_y + 2.0, cap_w, cap_h);
        let _ = ctx.fill();

        // Metallic body
        let grad = cairo::LinearGradient::new(cap_x, 0.0, cap_x + cap_w, 0.0);
        if is_dragging {
            grad.add_color_stop_rgb(0.0, 0.32, 0.32, 0.35);
            grad.add_color_stop_rgb(0.25, 0.48, 0.48, 0.52);
            grad.add_color_stop_rgb(0.5, 0.34, 0.34, 0.37);
            grad.add_color_stop_rgb(0.75, 0.48, 0.48, 0.52);
            grad.add_color_stop_rgb(1.0, 0.28, 0.28, 0.30);
        } else {
            grad.add_color_stop_rgb(0.0, 0.24, 0.24, 0.26);
            grad.add_color_stop_rgb(0.25, 0.38, 0.38, 0.41);
            grad.add_color_stop_rgb(0.5, 0.27, 0.27, 0.29);
            grad.add_color_stop_rgb(0.75, 0.38, 0.38, 0.41);
            grad.add_color_stop_rgb(1.0, 0.20, 0.20, 0.22);
        }
        let _ = ctx.set_source(&grad);

        // Rounded rect
        ctx.new_sub_path();
        ctx.arc(
            cap_x + cap_w - corner_r,
            cap_y + corner_r,
            corner_r,
            -PI / 2.0,
            0.0,
        );
        ctx.arc(
            cap_x + cap_w - corner_r,
            cap_y + cap_h - corner_r,
            corner_r,
            0.0,
            PI / 2.0,
        );
        ctx.arc(
            cap_x + corner_r,
            cap_y + cap_h - corner_r,
            corner_r,
            PI / 2.0,
            PI,
        );
        ctx.arc(
            cap_x + corner_r,
            cap_y + corner_r,
            corner_r,
            PI,
            3.0 * PI / 2.0,
        );
        ctx.close_path();
        let _ = ctx.fill_preserve();

        // Border
        ctx.set_source_rgba(0.04, 0.04, 0.05, 0.9);
        ctx.set_line_width(1.0);
        let _ = ctx.stroke();

        // Grip ribs (scaled for smaller cap)
        let num_ribs = 5;
        let rib_spacing = (cap_h - 12.0) / (num_ribs as f64 - 1.0);
        let mid_rib = num_ribs / 2;

        for r in 0..num_ribs {
            let rib_y = cap_y + 6.0 + (r as f64 * rib_spacing);

            if r == mid_rib {
                if is_dragging {
                    ctx.set_source_rgb(1.0, 0.62, 0.12);
                } else {
                    ctx.set_source_rgb(0.55, 0.55, 0.60);
                }
                ctx.set_line_width(2.0);
                ctx.move_to(cap_x + 3.5, rib_y);
                ctx.line_to(cap_x + cap_w - 3.5, rib_y);
                let _ = ctx.stroke();
            } else {
                ctx.set_source_rgba(0.10, 0.10, 0.12, 0.85);
                ctx.set_line_width(1.4);
                ctx.move_to(cap_x + 3.5, rib_y);
                ctx.line_to(cap_x + cap_w - 3.5, rib_y);
                let _ = ctx.stroke();

                ctx.set_source_rgba(0.42, 0.42, 0.46, 0.30);
                ctx.set_line_width(0.9);
                ctx.move_to(cap_x + 3.5, rib_y + 1.0);
                ctx.line_to(cap_x + cap_w - 3.5, rib_y + 1.0);
                let _ = ctx.stroke();
            }
        }
    }
}

#[relm4::component(pub)]
impl Component for MixSliderModel {
    type Init = (f64, f64, f64, f64, i32, i32, SliderOrientation);
    type Input = MixSliderInput;
    type Output = MixSliderOutput;
    type CommandOutput = ();

    view! {
        #[name = "drawing_area"]
        gtk::DrawingArea {
            set_content_width: model.width,
            set_content_height: model.height,
            set_focusable: true,
            set_can_target: true,

            add_controller = gtk::GestureDrag {
                set_button: 0,
                connect_drag_begin[sender] => move |gesture, x, y| {
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                    sender.input(MixSliderInput::StartDrag(x, y));
                },
                connect_drag_update[sender] => move |gesture, offset_x, offset_y| {
                    if let Some((start_x, start_y)) = gesture.start_point() {
                        let abs_x = start_x + offset_x;
                        let abs_y = start_y + offset_y;
                        sender.input(MixSliderInput::DragTo(abs_x, abs_y));
                    }
                },
                connect_drag_end[sender] => move |_, _, _| {
                    sender.input(MixSliderInput::EndDrag);
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            val: init.0.clamp(init.1, init.2),
            min: init.1,
            max: init.2,
            step: init.3,
            width: init.4,
            height: init.5,
            orientation: init.6,
            is_dragging: false,
        };

        let widgets = view_output!();

        {
            let val = model.val;
            let min = model.min;
            let max = model.max;
            let is_dragging = model.is_dragging;

            widgets.drawing_area.set_draw_func(move |_, ctx, w, h| {
                MixSliderModel::draw_vertical_fader(
                    ctx,
                    w as f64,
                    h as f64,
                    val,
                    min,
                    max,
                    is_dragging,
                );
            });
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        let mut needs_redraw = false;

        match message {
            MixSliderInput::SyncValue(val) => {
                let new_val = val.clamp(self.min, self.max);
                if (self.val - new_val).abs() > f64::EPSILON {
                    self.val = new_val;
                    needs_redraw = true;
                }
            }
            MixSliderInput::SetValue(val) => {
                let stepped = if self.step > 0.0 {
                    (val / self.step).round() * self.step
                } else {
                    val
                };
                let clamped = stepped.clamp(self.min, self.max);
                if (self.val - clamped).abs() > f64::EPSILON {
                    self.val = clamped;
                    needs_redraw = true;
                    let _ = sender.output(MixSliderOutput::ValueChanged(self.val));
                }
            }
            MixSliderInput::StartDrag(x, y) => {
                self.is_dragging = true;
                needs_redraw = true;
                let val = self.pos_to_value(x, y);
                sender.input(MixSliderInput::SetValue(val));
            }
            MixSliderInput::DragTo(x, y) => {
                if self.is_dragging {
                    let val = self.pos_to_value(x, y);
                    sender.input(MixSliderInput::SetValue(val));
                }
            }
            MixSliderInput::EndDrag => {
                if self.is_dragging {
                    self.is_dragging = false;
                    needs_redraw = true;
                    let _ = sender.output(MixSliderOutput::Released);
                }
            }
        }

        if needs_redraw {
            let val = self.val;
            let min = self.min;
            let max = self.max;
            let is_dragging = self.is_dragging;

            root.set_draw_func(move |_, ctx, w, h| {
                MixSliderModel::draw_vertical_fader(
                    ctx,
                    w as f64,
                    h as f64,
                    val,
                    min,
                    max,
                    is_dragging,
                );
            });
            root.queue_draw();
        }
    }
}
