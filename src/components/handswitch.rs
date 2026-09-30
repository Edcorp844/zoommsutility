use gtk::cairo;
use gtk::prelude::*;
use log::info;
use relm4::prelude::*;
use relm4::ComponentParts;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct HandSwitchModel {
    pub is_active: bool,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub enum HandSwitchInput {
    /// Silent hardware / parent sync – no output
    SetState(bool),
    /// User-driven set – emits StateChanged + Released
    SetActive(bool),
    /// User click – emits StateChanged + Released
    Toggle,
}

#[derive(Debug, Clone)]
pub enum HandSwitchOutput {
    /// New on/off state after user interaction
    StateChanged(bool),
    /// Gesture finished – parent can commit live buffer (WriteCurrentPatch)
    Released,
}

#[relm4::component(pub)]
impl SimpleComponent for HandSwitchModel {
    type Init = (bool, i32, i32);
    type Input = HandSwitchInput;
    type Output = HandSwitchOutput;

    view! {
        gtk::DrawingArea {
            set_content_width: model.width,
            set_content_height: model.height,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,

            add_controller: {
                let gesture = gtk::GestureClick::new();
                let sender = sender.clone();
                gesture.connect_pressed(move |_, _, _, _| {
                    sender.input(HandSwitchInput::Toggle);
                });
                gesture
            },

            #[watch]
            set_draw_func: {
                let is_active = model.is_active;
                move |_, ctx, width, height| {
                    let w = width as f64;
                    let h = height as f64;
                    let cx = w / 2.0;
                    let cy = h / 2.0;

                    let hex_radius = (w.min(h) / 2.0) * 0.40;
                    let base_w = hex_radius * 0.50;
                    let ball_r = hex_radius * 0.65;
                    let lever_reach = h * 0.32;

                    let draw_hexagon = |r: f64| {
                        ctx.new_path();
                        for i in 0..6 {
                            let angle = (i as f64) * PI / 3.0 - (PI / 6.0);
                            let x = cx + r * angle.cos();
                            let y = cy + r * angle.sin();
                            if i == 0 {
                                ctx.move_to(x, y);
                            } else {
                                ctx.line_to(x, y);
                            }
                        }
                        ctx.close_path();
                    };

                    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.6);
                    draw_hexagon(hex_radius + 1.5);
                    let _ = ctx.fill();

                    ctx.set_source_rgb(0.72, 0.74, 0.78);
                    draw_hexagon(hex_radius);
                    let _ = ctx.fill();

                    ctx.set_source_rgba(1.0, 1.0, 1.0, 0.85);
                    ctx.set_line_width(1.5);
                    draw_hexagon(hex_radius);
                    let _ = ctx.stroke();

                    ctx.set_source_rgb(0.85, 0.88, 0.92);
                    draw_hexagon(hex_radius - 1.8);
                    let _ = ctx.fill();

                    let led_cx = cx + (hex_radius * 1.05);
                    let led_cy = cy - (hex_radius * 0.75);
                    let led_r = (hex_radius * 0.25).max(3.0);
                    let bezel_r = led_r + 2.0;

                    ctx.set_source_rgb(0.70, 0.72, 0.76);
                    ctx.arc(led_cx, led_cy, bezel_r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    ctx.set_source_rgb(0.12, 0.14, 0.16);
                    ctx.arc(led_cx, led_cy, led_r + 0.5, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    if is_active {
                        let glow_r = led_r * 2.8;
                        let bloom = cairo::RadialGradient::new(
                            led_cx, led_cy, 0.0, led_cx, led_cy, glow_r,
                        );
                        bloom.add_color_stop_rgba(0.0, 0.2, 0.95, 0.3, 0.65);
                        bloom.add_color_stop_rgba(0.5, 0.1, 0.80, 0.2, 0.25);
                        bloom.add_color_stop_rgba(1.0, 0.0, 0.00, 0.0, 0.00);
                        let _ = ctx.set_source(&bloom);
                        ctx.arc(led_cx, led_cy, glow_r, 0.0, 2.0 * PI);
                        let _ = ctx.fill();

                        let lens_grad = cairo::RadialGradient::new(
                            led_cx - (led_r * 0.3), led_cy - (led_r * 0.3), led_r * 0.1,
                            led_cx, led_cy, led_r,
                        );
                        lens_grad.add_color_stop_rgb(0.0, 0.75, 1.0, 0.55);
                        lens_grad.add_color_stop_rgb(0.6, 0.15, 0.88, 0.25);
                        lens_grad.add_color_stop_rgb(1.0, 0.05, 0.55, 0.10);
                        let _ = ctx.set_source(&lens_grad);
                        ctx.arc(led_cx, led_cy, led_r, 0.0, 2.0 * PI);
                        let _ = ctx.fill();
                    } else {
                        let lens_grad = cairo::RadialGradient::new(
                            led_cx - (led_r * 0.3), led_cy - (led_r * 0.3), led_r * 0.1,
                            led_cx, led_cy, led_r,
                        );
                        lens_grad.add_color_stop_rgb(0.0, 0.12, 0.25, 0.14);
                        lens_grad.add_color_stop_rgb(1.0, 0.04, 0.08, 0.05);
                        let _ = ctx.set_source(&lens_grad);
                        ctx.arc(led_cx, led_cy, led_r, 0.0, 2.0 * PI);
                        let _ = ctx.fill();
                    }

                    ctx.set_source_rgba(1.0, 1.0, 1.0, if is_active { 0.85 } else { 0.35 });
                    ctx.arc(
                        led_cx - (led_r * 0.35),
                        led_cy - (led_r * 0.35),
                        led_r * 0.35,
                        0.0,
                        2.0 * PI,
                    );
                    let _ = ctx.fill();

                    let socket_r = hex_radius * 0.55;

                    ctx.set_source_rgb(0.50, 0.52, 0.56);
                    ctx.arc(cx, cy, socket_r + 1.2, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    ctx.set_source_rgb(0.04, 0.04, 0.05);
                    ctx.arc(cx, cy, socket_r, 0.0, 2.0 * PI);
                    let _ = ctx.fill();

                    ctx.set_source_rgba(0.0, 0.0, 0.0, 0.8);
                    ctx.set_line_width(1.0);
                    ctx.arc(cx, cy, socket_r - 0.5, 0.0, 2.0 * PI);
                    let _ = ctx.stroke();

                    if is_active {
                        let tip_cy = cy - lever_reach;

                        ctx.set_source_rgba(0.0, 0.0, 0.0, 0.7);
                        ctx.arc(cx, cy + 3.0, socket_r * 0.6, 0.0, 2.0 * PI);
                        let _ = ctx.fill();

                        ctx.set_source_rgb(0.78, 0.81, 0.86);
                        ctx.move_to(cx - (base_w / 2.0), cy + 2.0);
                        ctx.line_to(cx + (base_w / 2.0), cy + 2.0);
                        ctx.line_to(cx + (ball_r * 0.85), tip_cy);
                        ctx.line_to(cx - (ball_r * 0.85), tip_cy);
                        ctx.close_path();
                        let _ = ctx.fill();

                        ctx.set_source_rgb(0.88, 0.91, 0.95);
                        ctx.arc(cx, tip_cy, ball_r, 0.0, 2.0 * PI);
                        let _ = ctx.fill();

                        ctx.set_source_rgba(1.0, 1.0, 1.0, 0.95);
                        ctx.arc(
                            cx - (ball_r * 0.3),
                            tip_cy - (ball_r * 0.3),
                            ball_r * 0.45,
                            0.0,
                            2.0 * PI,
                        );
                        let _ = ctx.fill();

                        ctx.set_source_rgba(1.0, 1.0, 1.0, 0.8);
                        ctx.set_line_width(1.4);
                        ctx.move_to(cx - (base_w / 2.0) + 0.6, cy + 1.0);
                        ctx.line_to(cx - (ball_r * 0.6), tip_cy);
                        let _ = ctx.stroke();
                    } else {
                        let tip_cy = cy + lever_reach;

                        ctx.set_source_rgba(0.0, 0.0, 0.0, 0.7);
                        ctx.arc(cx, cy - 3.0, socket_r * 0.6, 0.0, 2.0 * PI);
                        let _ = ctx.fill();

                        ctx.set_source_rgb(0.52, 0.55, 0.60);
                        ctx.move_to(cx - (base_w / 2.0), cy - 2.0);
                        ctx.line_to(cx + (base_w / 2.0), cy - 2.0);
                        ctx.line_to(cx + (ball_r * 0.85), tip_cy);
                        ctx.line_to(cx - (ball_r * 0.85), tip_cy);
                        ctx.close_path();
                        let _ = ctx.fill();

                        ctx.set_source_rgb(0.65, 0.68, 0.73);
                        ctx.arc(cx, tip_cy, ball_r, 0.0, 2.0 * PI);
                        let _ = ctx.fill();

                        ctx.set_source_rgba(1.0, 1.0, 1.0, 0.55);
                        ctx.arc(
                            cx - (ball_r * 0.3),
                            tip_cy - (ball_r * 0.3),
                            ball_r * 0.45,
                            0.0,
                            2.0 * PI,
                        );
                        let _ = ctx.fill();

                        ctx.set_source_rgba(1.0, 1.0, 1.0, 0.5);
                        ctx.set_line_width(1.2);
                        ctx.move_to(cx - (base_w / 2.0) + 0.7, cy - 1.0);
                        ctx.line_to(cx - (ball_r * 0.6), tip_cy);
                        let _ = ctx.stroke();
                    }
                }
            }
        }
    }

    fn init(
        (is_active, width, height): Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            is_active,
            width,
            height,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            HandSwitchInput::SetState(state) => {
                // Hardware sync – silent
                info!("Received Hardware val={state}");
                self.is_active = state;
            }
            HandSwitchInput::SetActive(active) => {
                self.is_active = active;
                info!("HandSwitch state set to: {}", self.is_active);
                let _ = sender.output(HandSwitchOutput::StateChanged(self.is_active));
                let _ = sender.output(HandSwitchOutput::Released);
            }
            HandSwitchInput::Toggle => {
                self.is_active = !self.is_active;
                info!("HandSwitch toggled to: {}", self.is_active);
                let _ = sender.output(HandSwitchOutput::StateChanged(self.is_active));
                let _ = sender.output(HandSwitchOutput::Released);
            }
        }
    }
}
