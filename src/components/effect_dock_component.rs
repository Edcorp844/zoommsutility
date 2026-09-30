use gettextrs::gettext;
use gtk::gio;
use gtk::prelude::*;
use gtk::Image;
use relm4::prelude::*;
use relm4::ComponentParts;
use std::cell::Cell;
use std::rc::Rc;

use crate::components::led_indicator::LedIndicatorModel;
use crate::components::led_indicator::LedState;
use crate::main_content::utils_ext::SlotChainState;

#[derive(Debug, Clone)]
pub(crate) enum EffectDockComponentAction {
    Insert,
    Replace,
    MoveLeft,
    MoveRight,
    Delete,
}

#[derive(Debug)]
pub struct EffectDockComponentModel {
    pub title: String,
    pub image: Image,
    pub slot_chain_state: Rc<Cell<SlotChainState>>,
    pub led_state: LedState,
    pub led: Controller<LedIndicatorModel>,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub enum EffectDockComponentInput {
    Clicked,
    Activate,
    SetState(SlotChainState),
}

#[derive(Debug, Clone)]
pub enum EffectDockComponentOutput {
    EffectAction { action: EffectDockComponentAction },
    Clicked,
}

#[relm4::component(pub)]
impl SimpleComponent for EffectDockComponentModel {
    type Init = (String, Image, SlotChainState, LedState);
    type Input = EffectDockComponentInput;
    type Output = EffectDockComponentOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Center,
            #[watch]
            set_css_classes: if model.active { &["card"] } else { &[] },
            set_margin_horizontal: 10,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_horizontal: 10,
                set_halign: gtk::Align::Fill,

                model.led.widget() {},

                gtk::Box {
                    set_hexpand: true,
                },

                gtk::MenuButton {
                    set_icon_name: "view-more-symbolic",
                    set_css_classes: &vec!["circular", "flat"],
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,

                    #[wrap(Some)]
                    #[name = "menu_button"]
                    set_popover = &gtk::PopoverMenu::from_model(Some(&{
                        let menu = gtk::gio::Menu::new();

                        let insert_section = gtk::gio::Menu::new();
                        let replace_item = gtk::gio::MenuItem::new(Some("Replace"), Some("actions.replace"));
                        let insert_item = gtk::gio::MenuItem::new(Some("Insert"), Some("actions.insert"));
                        insert_section.append_item(&replace_item);
                        insert_section.append_item(&insert_item);
                        menu.append_section(None, &insert_section);

                        let move_section = gtk::gio::Menu::new();
                        let move_left_item = gtk::gio::MenuItem::new(Some("Move Left"), Some("actions.move-left"));
                        let move_right_item = gtk::gio::MenuItem::new(Some("Move Right"), Some("actions.move-right"));
                        move_section.append_item(&move_left_item);
                        move_section.append_item(&move_right_item);
                        menu.append_section(None, &move_section);

                        let delete_section = gtk::gio::Menu::new();
                        let delete_item = gtk::gio::MenuItem::new(Some("Delete"), Some("actions.delete"));
                        delete_section.append_item(&delete_item);
                        menu.append_section(None, &delete_section);

                        menu
                    })) {
                        set_position: gtk::PositionType::Top,
                    },
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                // Image with overlay: the pin-to-pin wire + THRU/BYPASS label
                // is drawn on top of the pedal image when the slot is not active.
                gtk::Overlay {
                    model.image.clone() {},

                    add_overlay = &gtk::DrawingArea {
                        set_can_target: false,
                        set_halign: gtk::Align::Fill,
                        set_valign: gtk::Align::Fill,
                        set_hexpand: true,
                        set_vexpand: true,
                        set_draw_func: {
                            let state = model.slot_chain_state.clone();
                            move |_area, ctx, w, h| {
                                let (draw, label, wire) = match state.get() {
                                    SlotChainState::Active => (false, "", (0.0, 0.0, 0.0, 0.0)),
                                    SlotChainState::Thru => (true, "THRU", (0.90, 0.75, 0.20, 1.0)),
                                    SlotChainState::ForcedThru => (true, "THRU", (1.00, 0.35, 0.25, 1.0)),
                                    SlotChainState::Bypassed => (true, "BYPASS", (0.60, 0.60, 0.65, 1.0)),
                                };
                                if !draw {
                                    return;
                                }

                                let w = w as f64;
                                let h = h as f64;
                                let y = h / 2.0;
                                let x_left = 6.0;
                                let x_right = w - 6.0;

                                // Soft dark shadow under the wire for contrast.
                                ctx.set_source_rgba(0.0, 0.0, 0.0, 0.55);
                                ctx.set_line_width(7.0);
                                ctx.move_to(x_left, y);
                                ctx.line_to(x_right, y);
                                let _ = ctx.stroke();

                                // Wire body.
                                ctx.set_source_rgba(wire.0, wire.1, wire.2, wire.3);
                                ctx.set_line_width(3.0);
                                ctx.move_to(x_left, y);
                                ctx.line_to(x_right, y);
                                let _ = ctx.stroke();

                                // Endpoint "pins".
                                let pin_r = 4.0;
                                ctx.set_source_rgba(wire.0, wire.1, wire.2, 1.0);
                                ctx.arc(x_left, y, pin_r, 0.0, std::f64::consts::TAU);
                                let _ = ctx.fill();
                                ctx.arc(x_right, y, pin_r, 0.0, std::f64::consts::TAU);
                                let _ = ctx.fill();

                                ctx.set_source_rgba(0.0, 0.0, 0.0, 0.85);
                                ctx.set_line_width(1.5);
                                ctx.arc(x_left, y, pin_r, 0.0, std::f64::consts::TAU);
                                let _ = ctx.stroke();
                                ctx.arc(x_right, y, pin_r, 0.0, std::f64::consts::TAU);
                                let _ = ctx.stroke();

                                // Dark band behind the label for readability.
                                let text_w = 64.0;
                                let text_h = 20.0;
                                let band_x = (w - text_w) / 2.0;
                                let band_y = y - text_h / 2.0;
                                ctx.set_source_rgba(0.0, 0.0, 0.0, 0.78);
                                ctx.rectangle(band_x, band_y, text_w, text_h);
                                let _ = ctx.fill();

                                // Label text.
                                ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                                ctx.select_font_face(
                                    "Sans",
                                    gtk::cairo::FontSlant::Normal,
                                    gtk::cairo::FontWeight::Bold,
                                );
                                ctx.set_font_size(12.0);
                                if let Ok(extents) = ctx.text_extents(label) {
                                    let tx = (w - extents.width()) / 2.0 - extents.x_bearing();
                                    let ty = y - (extents.height() / 2.0) - extents.y_bearing();
                                    ctx.move_to(tx, ty);
                                    let _ = ctx.show_text(label);
                                }
                            }
                        },
                    },
                },

                gtk::Label {
                    set_margin_horizontal: 10,
                    set_label: &model.title,
                    #[watch]
                    add_css_class: if model.dim_label() { "dimmed" } else { "normal" },
                },

                add_controller: {
                    let gesture = gtk::GestureClick::new();
                    let sender = sender.clone();
                    gesture.connect_pressed(move |_, _, _, _| {
                        sender.input(EffectDockComponentInput::Clicked);
                    });
                    gesture
                },
            },
        }
    }

    fn init(
        (title, image, slot_chain_state, led_state): Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let action_group = gio::SimpleActionGroup::new();

        // Replace
        let replace = gio::SimpleAction::new("replace", None);
        let sender_clone = sender.clone();
        replace.connect_activate(move |_, _| {
            let _ = sender_clone.output(EffectDockComponentOutput::EffectAction {
                action: EffectDockComponentAction::Replace,
            });
        });

        // Insert
        let insert = gio::SimpleAction::new("insert", None);
        let sender_clone = sender.clone();
        insert.connect_activate(move |_, _| {
            let _ = sender_clone.output(EffectDockComponentOutput::EffectAction {
                action: EffectDockComponentAction::Insert,
            });
        });

        // Move left
        let move_left = gio::SimpleAction::new("move-left", None);
        let sender_clone = sender.clone();
        move_left.connect_activate(move |_, _| {
            let _ = sender_clone.output(EffectDockComponentOutput::EffectAction {
                action: EffectDockComponentAction::MoveLeft,
            });
        });

        // Move right
        let move_right = gio::SimpleAction::new("move-right", None);
        let sender_clone = sender.clone();
        move_right.connect_activate(move |_, _| {
            let _ = sender_clone.output(EffectDockComponentOutput::EffectAction {
                action: EffectDockComponentAction::MoveRight,
            });
        });

        // Delete
        let delete = gio::SimpleAction::new("delete", None);
        let sender_clone = sender.clone();
        delete.connect_activate(move |_, _| {
            let _ = sender_clone.output(EffectDockComponentOutput::EffectAction {
                action: EffectDockComponentAction::Delete,
            });
        });

        action_group.add_action(&insert);
        action_group.add_action(&replace);
        action_group.add_action(&move_left);
        action_group.add_action(&move_right);
        action_group.add_action(&delete);

        let led = LedIndicatorModel::builder()
            .launch((led_state, 18))
            .detach();

        let slot_chain_state = Rc::new(Cell::new(slot_chain_state));

        let model = Self {
            title,
            image,
            slot_chain_state,
            led_state,
            led,
            active: false,
        };
        let widgets = view_output!();

        // Insert the action group on the root widget so the popover can
        // resolve "actions.replace", "actions.insert", etc. When the group
        // is inserted on the MenuButton itself, the popover often can't
        // find it because it lives in a separate widget tree.
        root.insert_action_group("actions", Some(&action_group));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            EffectDockComponentInput::Clicked => {
                let _ = sender.output(EffectDockComponentOutput::Clicked);
            }
            EffectDockComponentInput::Activate => {
                self.active = true;
            }
            EffectDockComponentInput::SetState(new_state) => {
                self.slot_chain_state.set(new_state);
            }
        }
    }
}

impl EffectDockComponentModel {
    fn dim_label(&self) -> bool {
        match self.slot_chain_state.get() {
            SlotChainState::Active => false,
            SlotChainState::Bypassed => true,
            SlotChainState::Thru | SlotChainState::ForcedThru => true,
        }
    }
}