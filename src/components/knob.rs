use gtk::cairo;
use gtk::prelude::*;
use log::info;
use relm4::prelude::*;
use relm4::ComponentParts;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct KnobModel {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub size: i32,

    is_dragging: bool,
    last_angle: f64,
}

#[derive(Debug, Clone)]
pub enum KnobInput {
    SyncValue(f64),
    StartDrag { x: f64, y: f64 },
    UpdateDrag { x: f64, y: f64 },
    EndDrag,
}

#[derive(Debug, Clone)]
pub enum KnobOutput {
    /// Value changed while dragging (or via SetValue)
    ValueChanged(f64),
    /// Finger / mouse released – parent can commit full patch write
    Released,
}

#[relm4::component(pub)]
impl SimpleComponent for KnobModel {
    type Init = (f64, f64, f64, f64, i32);
    type Input = KnobInput;
    type Output = KnobOutput;

    view! {
        gtk::DrawingArea {
            set_content_width: model.size,
            set_content_height: model.size,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            set_focusable: true,
            set_can_target: true,

            add_controller = gtk::GestureDrag {
                set_button: 0,
                connect_drag_begin[sender] => move |gesture, x, y| {
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                    sender.input(KnobInput::StartDrag { x, y });
                },
                connect_drag_update[sender] => move |gesture, offset_x, offset_y| {
                    if let Some((sx, sy)) = gesture.start_point() {
                        sender.input(KnobInput::UpdateDrag {
                            x: sx + offset_x,
                            y: sy + offset_y,
                        });
                    }
                },
                connect_drag_end[sender] => move |_, _, _| {
                    sender.input(KnobInput::EndDrag);
                },
            },

            #[watch]
            set_draw_func: {
                let value = model.value;
                let min = model.min;
                let max = model.max;

                move |_, ctx, width, height| {
                    let w = width as f64;
                    let h = height as f64;
                    let cx = w / 2.0;
                    let cy = h / 2.0;
                    let radius = (w.min(h) / 2.0) - 2.0;

                    let start_angle = 0.75 * PI;
                    let end_angle = 2.25 * PI;
                    let norm_val = ((value - min) / (max - min)).clamp(0.0, 1.0);
                    let current_angle = start_angle + norm_val * (end_angle - start_angle);

                    let num_ticks = 15;
                    let tick_radius = radius * 0.84;
                    let dot_size = (radius * 0.038).max(1.5);
                    let font_size = (radius * 0.11).max(8.0);
                    ctx.set_font_size(font_size);

                    let mid_index = num_ticks / 2;

                    for i in 0..num_ticks {
                        let t = i as f64 / (num_ticks - 1) as f64;
                        let angle = start_angle + t * (end_angle - start_angle);
                        let tx = cx + tick_radius * angle.cos();
                        let ty = cy + tick_radius * angle.sin();

                        let is_active = t <= (norm_val + 0.01);
                        let is_labeled_tick = i == 0 || i == mid_index || i == num_ticks - 1;

                        if is_active {
                            ctx.set_source_rgba(1.0, 0.62, 0.12, 0.95);
                            ctx.arc(tx, ty, dot_size, 0.0, 2.0 * PI);
                            let _ = ctx.fill();

                            ctx.set_source_rgba(1.0, 0.60, 0.0, 0.25);
                            ctx.arc(tx, ty, dot_size * 1.8, 0.0, 2.0 * PI);
                            let _ = ctx.fill();
                        } else if is_labeled_tick {
                            let val = min + t * (max - min);
                            let text = if val.fract() == 0.0 {
                                format!("{:.0}", val)
                            } else {
                                format!("{:.1}", val)
                            };

                            if let Ok(extents) = ctx.text_extents(&text) {
                                let lx = tx - (extents.width() / 2.0 + extents.x_bearing());
                                let ly = ty - (extents.height() / 2.0 + extents.y_bearing());
                                ctx.set_source_rgb(0.35, 0.38, 0.42);
                                ctx.move_to(lx, ly);
                                let _ = ctx.show_text(&text);
                            }
                        } else {
                            ctx.set_source_rgb(0.18, 0.20, 0.22);
                            ctx.arc(tx, ty, dot_size * 0.8, 0.0, 2.0 * PI);
                            let _ = ctx.fill();
                        }
                    }

                    let base_r = radius * 0.68;
                    let base_grad = cairo::RadialGradient::new(
                        cx, cy - (base_r * 0.2), base_r * 0.2,
                        cx, cy, base_r,
                    );
                    base_grad.add_color_stop_rgb(0.0, 0.22, 0.24, 0.27);
                    base_grad.add_color_stop_rgb(1.0, 0.10, 0.11, 0.13);
                    let _ = ctx.set_source(&base_grad);
                    ctx.arc(cx, cy, base_r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    let cap_r = base_r * 0.72;

                    let shadow_grad = cairo::RadialGradient::new(
                        cx, cy + 3.0, cap_r * 0.8,
                        cx, cy + 3.0, cap_r + 4.0,
                    );
                    shadow_grad.add_color_stop_rgba(0.0, 0.0, 0.0, 0.0, 0.6);
                    shadow_grad.add_color_stop_rgba(1.0, 0.0, 0.0, 0.0, 0.0);
                    let _ = ctx.set_source(&shadow_grad);
                    ctx.arc(cx, cy + 3.0, cap_r + 4.0, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    let cap_grad = cairo::RadialGradient::new(
                        cx - (cap_r * 0.2), cy - (cap_r * 0.3), cap_r * 0.1,
                        cx, cy, cap_r,
                    );
                    cap_grad.add_color_stop_rgb(0.0, 0.24, 0.26, 0.29);
                    cap_grad.add_color_stop_rgb(0.7, 0.16, 0.17, 0.19);
                    cap_grad.add_color_stop_rgb(1.0, 0.11, 0.12, 0.14);
                    let _ = ctx.set_source(&cap_grad);
                    ctx.arc(cx, cy, cap_r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    ctx.set_source_rgba(1.0, 1.0, 1.0, 0.08);
                    ctx.set_line_width(1.0);
                    ctx.arc(cx, cy, cap_r - 0.5, 0.0, 2.0 * PI);
                    let _ = ctx.stroke();

                    let ptr_dist = cap_r * 0.58;
                    let ptr_cx = cx + ptr_dist * current_angle.cos();
                    let ptr_cy = cy + ptr_dist * current_angle.sin();
                    let capsule_len = cap_r * 0.30;
                    let capsule_w = (cap_r * 0.12).max(2.5);

                    ctx.save().unwrap();
                    ctx.translate(ptr_cx, ptr_cy);
                    ctx.rotate(current_angle + (PI / 2.0));

                    let h_len = capsule_len / 2.0;
                    let r_cap = capsule_w / 2.0;

                    ctx.new_path();
                    ctx.arc(0.0, -h_len + r_cap, r_cap, PI, 0.0);
                    ctx.arc(0.0, h_len - r_cap, r_cap, 0.0, PI);
                    ctx.close_path();

                    ctx.set_source_rgb(1.0, 0.65, 0.15);
                    let _ = ctx.fill_preserve();

                    ctx.set_source_rgba(1.0, 0.92, 0.60, 0.8);
                    ctx.set_line_width(0.8);
                    let _ = ctx.stroke();

                    ctx.restore().unwrap();
                }
            }
        }
    }

    fn init(
        (value, min, max, step, size): Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            value: value.clamp(min, max),
            min,
            max,
            step,
            size,
            is_dragging: false,
            last_angle: 0.0,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            // KnobInput::SetValue(val) => {
            //     let clamped = val.clamp(self.min, self.max);
            //     if (self.value - clamped).abs() > f64::EPSILON {
            //         self.value = clamped;
            //         let _ = sender.output(KnobOutput::ValueChanged(self.value));
            //     }
            // }
            KnobInput::SyncValue(val) => {
                info!("Received Hardware val={val}");
                self.value = val.clamp(self.min, self.max);
            }
            KnobInput::StartDrag { x, y } => {
                self.is_dragging = true;
                let cx = self.size as f64 / 2.0;
                let cy = self.size as f64 / 2.0;
                self.last_angle = (y - cy).atan2(x - cx);
            }
            KnobInput::UpdateDrag { x, y } => {
                if !self.is_dragging {
                    return;
                }

                let cx = self.size as f64 / 2.0;
                let cy = self.size as f64 / 2.0;
                let angle = (y - cy).atan2(x - cx);

                // Signed shortest delta (−π … +π)
                let mut delta = angle - self.last_angle;
                if delta > PI {
                    delta -= 2.0 * PI;
                } else if delta < -PI {
                    delta += 2.0 * PI;
                }
                self.last_angle = angle;

                // Positive delta = clockwise = increase value (matches the drawn arc)
                // Full turn (2π) ≈ full parameter range feels natural
                let sensitivity = (self.max - self.min) / (2.0 * PI);
                let raw_val = self.value + (delta * sensitivity);

                let steps = ((raw_val - self.min) / self.step).round();
                let stepped_val = (self.min + steps * self.step).clamp(self.min, self.max);

                if (stepped_val - self.value).abs() > f64::EPSILON {
                    self.value = stepped_val;
                    let _ = sender.output(KnobOutput::ValueChanged(self.value));
                }
            }
            KnobInput::EndDrag => {
                if self.is_dragging {
                    self.is_dragging = false;
                    let _ = sender.output(KnobOutput::Released);
                }
            }
        }
    }
}
