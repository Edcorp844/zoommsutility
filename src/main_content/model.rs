use crate::components::{
    effect_dock_component::{
        EffectDockComponentAction, EffectDockComponentInput, EffectDockComponentModel,
    },
    footswitch::{FootswitchInput, FootswitchModel},
    handswitch::HandSwitchInput,
    knob::KnobInput,
    led_indicator::LedIndicatorModel,
    mixslider::MixSliderInput,
};
use crate::main_content::ui_ext::{MainContentUI, ParamControl};
use crate::main_content::utils_ext::MainContentUtils;
use relm4::{gtk::prelude::*, prelude::*, ComponentParts};
use std::collections::HashMap;
use zoom_ms::{
    apatch::Apatch, effects::EffectDef, mock_trait::MockData, patch_buffer::PatchBuffer,
};

pub struct MainContentModel {
    pub(crate) patch: Option<PatchBuffer>,
    pub(crate) effects: Vec<EffectDef>,
    pub(crate) selected_slot: usize,
    pub(crate) chain_label: String,
    pub(crate) param_controllers: HashMap<usize, (ParamControl, gtk::Label)>,
    pub(crate) effect_footswitch: Option<Controller<FootswitchModel>>,
    pub(crate) dock_leds: Vec<Controller<LedIndicatorModel>>,
    pub(crate) dock_components: Vec<Controller<EffectDockComponentModel>>,
}

#[derive(Debug, Clone)]
pub enum MainContentModelInput {
    SetPatch(Option<PatchBuffer>, usize),
    SelectSlot(usize),
    UpdateParamValue {
        slot: usize,
        param_idx: usize,
        val: f64,
    },
    HardwareSyncParamValue {
        slot: usize,
        param_idx: usize,
        val: f64,
    },
    ParamReleased {
        slot: usize,
        param_idx: usize,
    },
    ToggleEffect {
        slot: usize,
        enabled: bool,
    },
    DockEffectAction {
        slot: usize,
        action: EffectDockComponentAction,
    },
}

#[derive(Debug, Clone)]
pub enum MainContentModelOutput {
    SelectSlot(usize),
    UpdateParamValue {
        slot: usize,
        param_idx: usize,
        val: f64,
    },
    AutoSave {
        slot: usize,
        param_idx: usize,
    },
}

#[relm4::component(pub)]
impl Component for MainContentModel {
    type Init = ();
    type Input = MainContentModelInput;
    type Output = MainContentModelOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_margin_all: 16,

            gtk::Frame {
                #[watch]
                set_label: Some(&model.chain_label),
                set_vexpand: true,
                set_hexpand: true,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_all: 16,

                    gtk::Label {
                        #[watch]
                        set_markup: &model.title_markup(),
                    },

                    #[name = "param_container"]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 24,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                        set_vexpand: true,
                    },

                    #[name = "status_row"]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 16,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                        set_margin_top: 4,
                    }
                }
            },

            gtk::Frame {
                set_label: Some("Signal Chain"),
                gtk::Overlay {
                    gtk::DrawingArea {
                        set_content_height: 120,
                        set_draw_func: move |_, ctx, width, height| {
                            let y_center = height as f64 / 2.0;
                            ctx.set_source_rgb(0.15, 0.15, 0.18);
                            ctx.set_line_width(8.0);
                            ctx.move_to(16.0, y_center);
                            ctx.line_to(width as f64 - 16.0, y_center);
                            let _ = ctx.stroke();
                            ctx.set_source_rgb(0.85, 0.55, 0.15);
                            ctx.set_line_width(2.5);
                            ctx.move_to(16.0, y_center);
                            ctx.line_to(width as f64 - 16.0, y_center);
                            let _ = ctx.stroke();
                        }
                    },
                    #[name = "dock_container"]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_homogeneous: true,
                        set_valign: gtk::Align::Center,
                        set_margin_top: 8,
                        set_margin_bottom: 8,
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            patch: None,
            effects: Vec::new(),
            selected_slot: 0,
            chain_label: String::new(),
            param_controllers: HashMap::new(),
            effect_footswitch: None,
            dock_leds: Vec::new(),
            dock_components: Vec::new(),
        };
        let widgets = view_output!();

       // let patch = PatchBuffer::mock();

       // _sender.input(MainContentModelInput::SetPatch(Some(patch), 0));

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            MainContentModelInput::SetPatch(patch, _patch_number) => {
                if let Some(ref p) = patch {
                    if let Ok(fx_list) = p.get_patch_effects() {
                        self.effects = fx_list;
                    }
                } else {
                    self.effects.clear();
                }
                self.patch = patch;
                sender.input(MainContentModelInput::SelectSlot(0));
            }

            MainContentModelInput::SelectSlot(slot) => {
                self.selected_slot = slot;
                self.chain_label = self.selected_effect_label();
                let model = self;
                Self::rebuild_param_container(model, widgets, &sender);
                Self::rebuild_dock_container(model, widgets, &sender);
                model.dock_components[model.selected_slot].emit(EffectDockComponentInput::Activate);
                let _ = sender.output(MainContentModelOutput::SelectSlot(slot));
            }

            MainContentModelInput::UpdateParamValue {
                slot,
                param_idx,
                val,
            } => {
                if let Some(ui_idx) = self.hw_to_ui(param_idx) {
                    if let Some(fx) = self.effects.get_mut(slot) {
                        if let Some(param) = fx.params.get_mut(ui_idx) {
                            param.def = val as u16;
                            if slot == self.selected_slot {
                                if let Some((_, value_label)) = self.param_controllers.get(&ui_idx)
                                {
                                    let text = Self::display_value(param, param.def);
                                    value_label.set_text(&text);
                                }
                            }
                        }
                    }
                }
                let _ = sender.output(MainContentModelOutput::UpdateParamValue {
                    slot,
                    param_idx,
                    val,
                });
            }

            MainContentModelInput::ToggleEffect { slot, enabled } => {
                if let Some(p) = &mut self.patch {
                    p.set_slot_enabled(slot, enabled);
                }
                self.sync_enable_ui(slot, enabled);
                let model = self;
                Self::rebuild_dock_container(model, widgets, &sender);
                Self::rebuild_status_row(model, widgets, &sender);

                let _ = sender.output(MainContentModelOutput::UpdateParamValue {
                    slot,
                    param_idx: 0,
                    val: if enabled { 1.0 } else { 0.0 },
                });
            }

            MainContentModelInput::HardwareSyncParamValue {
                slot,
                param_idx,
                val,
            } => {
                if param_idx == 0 {
                    let on = val > 0.0;
                    if let Some(p) = &mut self.patch {
                        p.set_slot_enabled(slot, on);
                    }
                    self.sync_enable_ui(slot, on);

                    Self::rebuild_dock_container(self, widgets, &sender);
                    Self::rebuild_status_row(self, widgets, &sender);
                    return;
                }

                if slot != self.selected_slot {
                    if let Some(ui_idx) = self.hw_to_ui(param_idx) {
                        if let Some(fx) = self.effects.get_mut(slot) {
                            if let Some(param) = fx.params.get_mut(ui_idx) {
                                param.def = val as u16;
                            }
                        }
                    }
                    return;
                }

                let Some(ui_idx) = self.hw_to_ui(param_idx) else {
                    return;
                };

                if let Some(fx) = self.effects.get_mut(slot) {
                    if let Some(param) = fx.params.get_mut(ui_idx) {
                        param.def = val as u16;
                        if let Some((control, value_label)) = self.param_controllers.get(&ui_idx) {
                            value_label.set_text(Self::display_value(param, param.def).as_str());
                            match control {
                                ParamControl::Knob(c) => c.emit(KnobInput::SyncValue(val)),
                                ParamControl::HandSwitch(c) => {
                                    c.emit(HandSwitchInput::SetState(val > 0.0))
                                }
                                ParamControl::MixSlider(c) => {
                                    c.emit(MixSliderInput::SyncValue(val))
                                }
                                ParamControl::Footswitch(c) => {
                                    c.emit(FootswitchInput::SetActive(val > 0.0))
                                }
                            }
                        }
                    }
                }
            }

            MainContentModelInput::ParamReleased { slot, param_idx } => {
                let _ = sender.output(MainContentModelOutput::AutoSave { slot, param_idx });
            }
            MainContentModelInput::DockEffectAction { slot, action } => {
                info!("slot: {slot}, action: {:?}", action);
                sender.input(MainContentModelInput::SelectSlot(slot));
            }
        }
    }
}

impl MainContentModel {
    pub(crate) fn selected_effect_label(&self) -> String {
        match self.effects.get(self.selected_slot) {
            Some(fx) => format!("Signal Chain — {}", fx.name),
            None => "Signal Chain".to_string(),
        }
    }
}
