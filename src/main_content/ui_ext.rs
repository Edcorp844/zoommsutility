/* main_content/ui_ext.rs
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

use gtk::{gio, prelude::*};
use relm4::{Component, ComponentController, ComponentSender, Controller};
use zoom_ms::effects::ParamDisp;

use crate::{
    components::{
        effect_dock_component::{EffectDockComponentModel, EffectDockComponentOutput},
        footswitch::{FootswitchInput, FootswitchModel, FootswitchOutput},
        handswitch::{HandSwitchModel, HandSwitchOutput},
        knob::{KnobModel, KnobOutput},
        led_indicator::{LedIndicatorInput, LedIndicatorModel},
        mixslider::{MixSliderModel, MixSliderOutput, SliderOrientation},
    },
    main_content::{
        model::{MainContentModel, MainContentModelInput, MainContentModelWidgets},
        utils_ext::{MainContentUtils, SlotChainState, WarningLevel},
    },
};

#[derive(Debug)]
pub(crate) enum ParamControl {
    Knob(Controller<KnobModel>),
    HandSwitch(Controller<HandSwitchModel>),
    MixSlider(Controller<MixSliderModel>),
    Footswitch(Controller<FootswitchModel>),
}

pub(crate) trait MainContentUI {
    fn title_markup(&self) -> String;
    fn load_pedal_image(&self, effect_name: &str, pixel_size: i32) -> gtk::Image;
    fn make_status_chip(&self, text: &str, level: WarningLevel, tooltip: &str) -> gtk::Label;
    fn sync_enable_ui(&mut self, slot: usize, enabled: bool);
    fn rebuild_dock_container(
        model: &mut MainContentModel,
        widgets: &mut MainContentModelWidgets,
        sender: &ComponentSender<MainContentModel>,
    );
    fn rebuild_param_container(
        model: &mut MainContentModel,
        widgets: &mut MainContentModelWidgets,
        sender: &ComponentSender<MainContentModel>,
    );
    fn rebuild_status_row(
        model: &mut MainContentModel,
        widgets: &mut MainContentModelWidgets,
        sender: &ComponentSender<MainContentModel>,
    );
}

impl MainContentUI for MainContentModel {
    fn title_markup(&self) -> String {
        match self.effects.get(self.selected_slot) {
            Some(fx) if fx.group == "THRU" => "<i>Empty Slot (THRU)</i>".to_string(),
            Some(fx) => format!(
                "<span size='x-large' weight='bold'>{}</span>\n<i>{}</i>",
                fx.name, fx.title
            ),
            None => String::new(),
        }
    }

    fn load_pedal_image(&self, effect_name: &str, pixel_size: i32) -> gtk::Image {
        let clean_name = effect_name.trim().replace(' ', "_");
        let resource_path = format!(
            "/io/github/edcorp844/zoommsutility/images/{}.png",
            clean_name
        );
        if gio::resources_lookup_data(&resource_path, gio::ResourceLookupFlags::NONE).is_ok() {
            let img = gtk::Image::from_resource(&resource_path);
            img.set_pixel_size(pixel_size);
            img
        } else {
            let fallback = "/io/github/edcorp844/zoommsutility/images/THRU.png";
            if gio::resources_lookup_data(fallback, gio::ResourceLookupFlags::NONE).is_ok() {
                let img = gtk::Image::from_resource(fallback);
                img.set_pixel_size(pixel_size);
                img
            } else {
                let img = gtk::Image::from_icon_name("audio-card-symbolic");
                img.set_pixel_size(pixel_size);
                img
            }
        }
    }

    fn make_status_chip(&self, text: &str, level: WarningLevel, tooltip: &str) -> gtk::Label {
        let lbl = gtk::Label::new(Some(text));
        lbl.add_css_class("caption");
        lbl.set_tooltip_text(Some(tooltip));
        match level {
            WarningLevel::Ok => lbl.add_css_class("dim-label"),
            WarningLevel::Caution => lbl.add_css_class("warning"),
            WarningLevel::Critical => lbl.add_css_class("error"),
        }
        lbl
    }

    fn rebuild_status_row(
        model: &mut MainContentModel,
        widgets: &mut MainContentModelWidgets,
        sender: &ComponentSender<MainContentModel>,
    ) {
        while let Some(child) = widgets.status_row.first_child() {
            widgets.status_row.remove(&child);
        }
        model.effect_footswitch = None;

        let (used, states) = model.analyse_chain();
        let level = model.overall_warning(used, &states);

        let Some(fx) = model.effects.get(model.selected_slot) else {
            return;
        };

        // Stomp (on/off for real effects)
        if fx.group != "THRU" {
            let enabled = model.slot_enabled(model.selected_slot);
            let selected_slot = model.selected_slot;
            let footswitch = FootswitchModel::builder().launch((enabled, 52)).forward(
                sender.input_sender(),
                move |msg| match msg {
                    FootswitchOutput::StateChanged(on) => MainContentModelInput::ToggleEffect {
                        slot: selected_slot,
                        enabled: on,
                    },
                    FootswitchOutput::Released => MainContentModelInput::ParamReleased {
                        slot: selected_slot,
                        param_idx: 0,
                    },
                },
            );
            let stomp_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
            stomp_box.set_halign(gtk::Align::Center);
            stomp_box.append(footswitch.widget());
            let stomp_name = gtk::Label::new(Some("ON / OFF"));
            stomp_name.add_css_class("caption");
            stomp_name.set_halign(gtk::Align::Center);
            stomp_box.append(&stomp_name);
            widgets.status_row.append(&stomp_box);
            model.effect_footswitch = Some(footswitch);
        }

        // DSP %
        widgets.status_row.append(&model.make_status_chip(
            &format!("DSP {:.0}%", used * 100.0),
            level,
            match level {
                WarningLevel::Ok => "DSP budget OK (g200kg 1/dsp scale)",
                WarningLevel::Caution => "High DSP – later slots may go THRU",
                WarningLevel::Critical => "DSP full – one or more effects forced THRU",
            },
        ));

        // Names forced silent by DSP
        let forced_names: Vec<String> = states
            .iter()
            .zip(model.effects.iter())
            .filter(|(s, _)| **s == SlotChainState::ForcedThru)
            .map(|(_, e)| e.name.clone())
            .collect();

        if !forced_names.is_empty() {
            widgets.status_row.append(&model.make_status_chip(
                &format!("THRU: {}", forced_names.join(", ")),
                WarningLevel::Critical,
                "These effects exceed remaining DSP and will not run (g200kg-style)",
            ));
        } else if level == WarningLevel::Caution {
            widgets.status_row.append(&model.make_status_chip(
                "DSP caution",
                WarningLevel::Caution,
                "Approaching limit – prefer lighter effects for remaining slots",
            ));
        }

        let active = states
            .iter()
            .filter(|s| **s == SlotChainState::Active)
            .count();
        widgets.status_row.append(&model.make_status_chip(
            &format!("{active}/6 active"),
            if active >= 6 {
                WarningLevel::Caution
            } else {
                WarningLevel::Ok
            },
            "Effects currently running in the chain",
        ));
    }

    fn rebuild_param_container(
        model: &mut MainContentModel,
        widgets: &mut MainContentModelWidgets,
        sender: &ComponentSender<MainContentModel>,
    ) {
        while let Some(child) = widgets.param_container.first_child() {
            widgets.param_container.remove(&child);
        }
        model.param_controllers.clear();

        if let Some(fx) = model.effects.get(model.selected_slot) {
            if fx.group != "THRU" {
                let control_size = 100;
                info!(
                    "[MainContent] Rebuilding slot {} ('{}'): {} params",
                    model.selected_slot,
                    fx.name,
                    fx.params.len()
                );

                for (ui_idx, param) in fx.params.iter().enumerate() {
                    let p_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
                    p_box.set_halign(gtk::Align::Center);
                    p_box.set_valign(gtk::Align::Center);

                    let control_wrapper = gtk::Box::new(gtk::Orientation::Vertical, 2);
                    control_wrapper.set_size_request(control_size, control_size);
                    control_wrapper.set_halign(gtk::Align::Center);
                    control_wrapper.set_valign(gtk::Align::Center);

                    let name_label = gtk::Label::new(Some(&param.name));
                    name_label.add_css_class("title-4");
                    name_label.set_halign(gtk::Align::Center);

                    let value_label = gtk::Label::new(Some(&Self::display_value(param, param.def)));
                    value_label.add_css_class("dim-label");
                    value_label.set_halign(gtk::Align::Center);
                    value_label.set_ellipsize(gtk::pango::EllipsizeMode::End);
                    value_label.set_max_width_chars(16);

                    let selected_slot = model.selected_slot;

                    let control = if param.max == 1 {
                        let is_active = param.def > 0;
                        if model.is_labeled_switch(param) {
                            if let ParamDisp::Labels(labels) = &param.disp {
                                let top =
                                    gtk::Label::new(Some(&Self::decode_html_entities(&labels[1])));
                                top.add_css_class("caption");
                                top.set_halign(gtk::Align::Center);
                                top.set_opacity(if is_active { 1.0 } else { 0.40 });

                                let bottom =
                                    gtk::Label::new(Some(&Self::decode_html_entities(&labels[0])));
                                bottom.add_css_class("caption");
                                bottom.set_halign(gtk::Align::Center);
                                bottom.set_opacity(if is_active { 0.40 } else { 1.0 });

                                value_label.set_visible(false);

                                let param_idx = model.ui_to_hw(ui_idx);
                                let hand_switch = HandSwitchModel::builder()
                                    .launch((is_active, control_size, control_size))
                                    .forward(sender.input_sender(), move |msg| match msg {
                                        HandSwitchOutput::StateChanged(new_state) => {
                                            MainContentModelInput::UpdateParamValue {
                                                slot: selected_slot,
                                                param_idx,
                                                val: if new_state { 1.0 } else { 0.0 },
                                            }
                                        }
                                        HandSwitchOutput::Released => {
                                            MainContentModelInput::ParamReleased {
                                                slot: selected_slot,
                                                param_idx,
                                            }
                                        }
                                    });

                                control_wrapper.append(&top);
                                control_wrapper.append(hand_switch.widget());
                                control_wrapper.append(&bottom);
                                p_box.append(&control_wrapper);
                                p_box.append(&name_label);
                                model.param_controllers.insert(
                                    ui_idx,
                                    (ParamControl::HandSwitch(hand_switch), value_label),
                                );
                                widgets.param_container.append(&p_box);
                                continue;
                            }
                        }

                        let param_idx = model.ui_to_hw(ui_idx);
                        let hand_switch = HandSwitchModel::builder()
                            .launch((is_active, control_size, control_size))
                            .forward(sender.input_sender(), move |msg| match msg {
                                HandSwitchOutput::StateChanged(new_state) => {
                                    MainContentModelInput::UpdateParamValue {
                                        slot: selected_slot,
                                        param_idx,
                                        val: if new_state { 1.0 } else { 0.0 },
                                    }
                                }
                                HandSwitchOutput::Released => {
                                    MainContentModelInput::ParamReleased {
                                        slot: selected_slot,
                                        param_idx,
                                    }
                                }
                            });
                        control_wrapper.append(hand_switch.widget());
                        ParamControl::HandSwitch(hand_switch)
                    } else if matches!(param.disp, ParamDisp::Offset(_)) {
                        let param_idx = model.ui_to_hw(ui_idx);
                        let mix_slider = MixSliderModel::builder()
                            .launch((
                                param.def as f64,
                                0.0,
                                param.max as f64,
                                1.0,
                                32,
                                150,
                                SliderOrientation::Vertical,
                            ))
                            .forward(sender.input_sender(), move |msg| match msg {
                                MixSliderOutput::ValueChanged(val) => {
                                    MainContentModelInput::UpdateParamValue {
                                        slot: selected_slot,
                                        param_idx,
                                        val,
                                    }
                                }
                                MixSliderOutput::Released => MainContentModelInput::ParamReleased {
                                    slot: selected_slot,
                                    param_idx,
                                },
                            });
                        control_wrapper.append(mix_slider.widget());
                        ParamControl::MixSlider(mix_slider)
                    } else {
                        let param_idx = model.ui_to_hw(ui_idx);
                        let knob = KnobModel::builder()
                            .launch((param.def as f64, 0.0, param.max as f64, 1.0, control_size))
                            .forward(sender.input_sender(), move |msg| match msg {
                                KnobOutput::ValueChanged(new_val) => {
                                    MainContentModelInput::UpdateParamValue {
                                        slot: selected_slot,
                                        param_idx,
                                        val: new_val,
                                    }
                                }
                                KnobOutput::Released => MainContentModelInput::ParamReleased {
                                    slot: selected_slot,
                                    param_idx,
                                },
                            });
                        control_wrapper.append(knob.widget());
                        ParamControl::Knob(knob)
                    };

                    p_box.append(&control_wrapper);
                    p_box.append(&value_label);
                    p_box.append(&name_label);
                    widgets.param_container.append(&p_box);
                    model
                        .param_controllers
                        .insert(ui_idx, (control, value_label));
                }
            }
        }

        Self::rebuild_status_row(model, widgets, sender);
    }

    fn rebuild_dock_container(
        model: &mut MainContentModel,
        widgets: &mut MainContentModelWidgets,
        sender: &ComponentSender<MainContentModel>,
    ) {
        while let Some(child) = widgets.dock_container.first_child() {
            widgets.dock_container.remove(&child);
        }
        model.dock_leds.clear();
        model.dock_components.clear();

        let (_used, states) = model.analyse_chain();

        for (slot_idx, fx) in model.effects.iter().enumerate() {
            let state = states
                .get(slot_idx)
                .copied()
                .unwrap_or(SlotChainState::Thru);

            let image = model.load_pedal_image(&fx.name, 80);
            match state {
                SlotChainState::Active => image.set_opacity(1.0),
                SlotChainState::Bypassed => image.set_opacity(0.45),
                SlotChainState::Thru | SlotChainState::ForcedThru => image.set_opacity(0.28),
            }

            let led_state = model.led_state_for_slot(state);

            let slot_idx_clone = slot_idx;
            let sender_clone = sender.clone();
            let dock_component = EffectDockComponentModel::builder()
                .launch((fx.name.clone(), image, state, led_state))
                .forward(sender.input_sender(), move |msg| match msg {
                    EffectDockComponentOutput::Clicked => {
                        MainContentModelInput::SelectSlot(slot_idx_clone)
                    }
                    EffectDockComponentOutput::EffectAction { action } => {
                        MainContentModelInput::DockEffectAction {
                            slot: slot_idx_clone,
                            action,
                        }
                    }
                });

            widgets.dock_container.append(dock_component.widget());
            model.dock_components.push(dock_component);
        }
    }

    fn sync_enable_ui(&mut self, slot: usize, enabled: bool) {
        let mut flags = self.enabled_flags();
        if slot < flags.len() {
            flags[slot] = enabled;
        }
        let (_used, states) = self.analyse_chain();

        for (i, state) in states.iter().enumerate() {
            if let Some(led) = self.dock_leds.get(i) {
                led.emit(LedIndicatorInput::SetState(self.led_state_for_slot(*state)));
            }
        }

        if slot == self.selected_slot {
            if let Some(fs) = &self.effect_footswitch {
                fs.emit(FootswitchInput::SetActive(enabled));
            }
        }
    }
}

