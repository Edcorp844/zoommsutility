use gtk::{gdk, gio};
use log::{error, info};
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;
use relm4::ComponentParts;
use zoom_ms::model::PedalModel;
use zoom_ms::patch_buffer::PatchBuffer;

use crate::components::main_menu_button;
use crate::components::main_menu_button::MainMenuButton;
use crate::main_content::model::MainContentModel;
use crate::main_content::model::MainContentModelInput;
use crate::main_content::model::MainContentModelOutput;
use crate::workers::zoom_client_worker::{
    ZoomClientWorker, ZoomClientWorkerInput, ZoomClientWorkerOutput,
};

pub struct ApplicationModel {
    main_menu_button_controller: Controller<MainMenuButton>,
    zoom_client_worker_controller: Controller<ZoomClientWorker>,
    main_content_controller: Controller<MainContentModel>,
    available_ports: Vec<String>,
    selected_port: Option<String>,
    detected_pedal: Option<PedalModel>,
    scanned_patches: Vec<String>,
    search_query: String,
    is_scanning_patches: bool,
    scan_progress: f64,
    current_patch: Option<PatchBuffer>,
    current_patch_number: Option<u8>,
    auto_save: bool,
}

#[derive(Debug)]
pub enum ApplicationModelInput {
    AutoDetectDevice,
    SelectPort(String),
    FetchMidiPorts,
    UpdateMidiPorts(Vec<String>),
    UpdateDeviceStatus {
        port_name: Option<String>,
        pedal_model: Option<PedalModel>,
    },
    ScanPatches,
    UpdateScanProgress {
        progress: f64,
    },
    UpdatePatchesList(Vec<String>),
    FilterPatches(String),
    SelectPatch(u8),
    NotifyUiError(String),

    // Context Menu Actions
    RenamePatch(u8, String),
    CopyPatch(u8, String),
    PastePatch(u8, String),
    ImportPatchFile(u8, String),
    ExportPatchFile(u8, String),
    ShowPatchAsText(u8, String),
    ImportPatchAsText(u8, String),
    SavePatchToDevice(u8, String),
    DeletePatch(u8, String),
    ConfirmRenamePatch {
        slot: u8,
        new_name: String,
    },

    RenamedPatch(u8, String),
    SetCurrentPatch(PatchBuffer, u8),
    SelectSlot(usize),
    SelectedSlotForEdit(bool),

    /// Outbound: UI control moved by user -> Worker
    UiParamEdit {
        slot: usize,
        param_idx: usize,
        val: f64,
    },

    /// Inbound: Physical knob turned on hardware -> MainContent UI sync
    HardwareParamUpdated {
        slot: usize,
        param_idx: usize,
        val: f64,
    },

    AutoSave {
        slot: usize,
        param_idx: usize,
    },
}

#[derive(Debug)]
pub enum ApplicationModelOutput {}

#[relm4::component(pub)]
impl Component for ApplicationModel {
    type Init = ();
    type Input = ApplicationModelInput;
    type Output = ApplicationModelOutput;
    type CommandOutput = ();

    view! {
        adw::ApplicationWindow {
            set_default_size: (1080, 800),

            #[wrap(Some)]
            set_content = &adw::NavigationSplitView {
                #[wrap(Some)]
                set_sidebar = &adw::NavigationPage {
                    set_title: "Patches",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            set_show_title: false,

                            pack_end = model.main_menu_button_controller.widget(),
                        },

                        #[wrap(Some)]
                        set_content = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 6,
                            set_margin_all: 8,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 6,

                                #[name = "search_entry"]
                                gtk::SearchEntry {
                                    set_placeholder_text: Some("Filter patches..."),
                                    connect_search_changed[sender] => move |entry| {
                                        sender.input(ApplicationModelInput::FilterPatches(entry.text().to_string()));
                                    }
                                },

                                #[name = "rescan_patches_button"]
                                gtk::Button {
                                    set_icon_name: "view-refresh-symbolic",
                                    set_tooltip_text: Some("Rescan Device Patches"),
                                    set_sensitive: false,
                                    connect_clicked => ApplicationModelInput::ScanPatches,
                                },
                            },

                            #[name = "patch_progress_bar"]
                            gtk::ProgressBar {
                                set_visible: false,
                                set_show_text: true,
                            },

                            gtk::ScrolledWindow {
                                set_vexpand: true,
                                set_hscrollbar_policy: gtk::PolicyType::Never,

                                #[name = "patch_list_box"]
                                gtk::ListBox {
                                    set_selection_mode: gtk::SelectionMode::Single,
                                    add_css_class: "navigation-sidebar",
                                }
                            }
                        }
                    }
                },

                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    set_tag: Some("main-content"),
                    set_title: "Zoom Ms Utility",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            #[wrap(Some)]
                            #[name = "title_box"]
                            set_title_widget = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 6,
                                set_halign: gtk::Align::Center,

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_halign: gtk::Align::Center,
                                    add_css_class: "linked",

                                    #[name = "port_list_button"]
                                    gtk::MenuButton {
                                        set_label: "Select MIDI Port",
                                        add_css_class: "raised",

                                        #[wrap(Some)]
                                        #[name = "port_popover"]
                                        set_popover = &gtk::Popover {
                                            set_autohide: true,
                                            connect_show => ApplicationModelInput::FetchMidiPorts,

                                            #[name = "port_list_box"]
                                            gtk::Box {
                                                set_orientation: gtk::Orientation::Vertical,
                                                set_spacing: 4,
                                                set_margin_all: 6,
                                            }
                                        }
                                    },

                                    gtk::Button {
                                        set_icon_name: "view-refresh-symbolic",
                                        set_tooltip_text: Some("Rescan MIDI Ports"),
                                        connect_clicked => ApplicationModelInput::FetchMidiPorts,
                                    },
                                },

                                #[name = "device_type_badge"]
                                gtk::Button {
                                    set_visible: false,
                                    add_css_class: "accent",
                                    add_css_class: "raised",
                                    set_can_target: false,
                                },

                                #[name = "status_badge"]
                                gtk::Button {
                                    set_label: "Disconnected",
                                    add_css_class: "raised",
                                    set_can_target: false,
                                }
                            }
                        },

                        #[wrap(Some)]
                        #[name = "toast_overlay"]
                        set_content = &adw::ToastOverlay {
                            model.main_content_controller.widget(),
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let zoom_client_worker_controller = ZoomClientWorker::builder().launch(()).forward(
            sender.input_sender(),
            |msg| match msg {
                ZoomClientWorkerOutput::DeviceStatusUpdated {
                    port_name,
                    pedal_model,
                } => ApplicationModelInput::UpdateDeviceStatus {
                    port_name,
                    pedal_model,
                },
                ZoomClientWorkerOutput::MidiPortsList(ports) => {
                    ApplicationModelInput::UpdateMidiPorts(ports)
                }
                ZoomClientWorkerOutput::ScanProgress(progress) => {
                    ApplicationModelInput::UpdateScanProgress { progress }
                }
                ZoomClientWorkerOutput::PatchesScanned(patches) => {
                    ApplicationModelInput::UpdatePatchesList(patches)
                }
                ZoomClientWorkerOutput::NotifyUiError(err) => {
                    ApplicationModelInput::NotifyUiError(err)
                }
                ZoomClientWorkerOutput::RenamedPatch(slot, name) => {
                    ApplicationModelInput::RenamedPatch(slot, name)
                }
                ZoomClientWorkerOutput::SelectedPatch(patch, patch_num) => {
                    ApplicationModelInput::SetCurrentPatch(patch, patch_num)
                }
                ZoomClientWorkerOutput::SelectedSlotForEdit(val) => {
                    ApplicationModelInput::SelectedSlotForEdit(val)
                }
                ZoomClientWorkerOutput::HardwareParamUpdated {
                    slot,
                    param_idx,
                    val,
                } => {
                    info!("[AppModel Output] Worker -> ApplicationModelInput::HardwareParamUpdated: slot={slot}, param_idx={param_idx}, val={val}");
                    ApplicationModelInput::HardwareParamUpdated {
                        slot,
                        param_idx,
                        val,
                    }
                }
                ZoomClientWorkerOutput::HardwarePatchChanged { slot, patch_buffer } => {
                    ApplicationModelInput::SetCurrentPatch(patch_buffer, slot)
                }
                ZoomClientWorkerOutput::HardwareSlotToggled { slot, enabled } => {
                    info!("slot {slot} state : {enabled}");
                    ApplicationModelInput::SelectedSlotForEdit(enabled)
                }
            },
        );

        let main_content_controller = MainContentModel::builder().launch(()).forward(
            sender.input_sender(),
            |msg| match msg {
                MainContentModelOutput::SelectSlot(slot) => ApplicationModelInput::SelectSlot(slot),
                MainContentModelOutput::UpdateParamValue {
                    slot,
                    param_idx,
                    val,
                } => ApplicationModelInput::UiParamEdit {
                    slot,
                    param_idx,
                    val,
                },
                MainContentModelOutput::AutoSave { slot, param_idx } => {
                    ApplicationModelInput::AutoSave { slot, param_idx }
                }
            },
        );

        let main_menu_button_controller = MainMenuButton::builder().launch(()).detach();

        let model = Self {
            main_menu_button_controller,
            zoom_client_worker_controller,
            main_content_controller,
            available_ports: Vec::new(),
            current_patch: None,
            current_patch_number: None,
            selected_port: None,
            detected_pedal: None,
            scanned_patches: Vec::new(),
            search_query: String::new(),
            is_scanning_patches: false,
            scan_progress: 0.0,
            auto_save: true,
        };

        let widgets = view_output!();

        model
            .zoom_client_worker_controller
            .emit(ZoomClientWorkerInput::AutoDetectDevice);

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            ApplicationModelInput::AutoDetectDevice => {
                widgets.port_list_button.set_label("Scanning...");
                self.zoom_client_worker_controller
                    .emit(ZoomClientWorkerInput::AutoDetectDevice);
            }

            ApplicationModelInput::SelectPort(port_name) => {
                self.selected_port = Some(port_name.clone());
                widgets.port_list_button.set_label(&port_name);
                widgets.port_popover.popdown();

                self.zoom_client_worker_controller
                    .emit(ZoomClientWorkerInput::SelectPort(port_name));
            }

            ApplicationModelInput::FetchMidiPorts => {
                self.zoom_client_worker_controller
                    .emit(ZoomClientWorkerInput::FetchMidiPorts);
            }

            ApplicationModelInput::UpdateMidiPorts(ports) => {
                self.available_ports = ports;

                while let Some(child) = widgets.port_list_box.first_child() {
                    widgets.port_list_box.remove(&child);
                }

                if self.available_ports.is_empty() {
                    let content = adw::ButtonContent::builder()
                        .css_classes(vec!["flat".to_string()])
                        .label("No MIDI ports found")
                        .halign(gtk::Align::Start)
                        .build();

                    let empty_btn = gtk::Button::builder()
                        .css_classes(vec!["flat".to_string()])
                        .child(&content)
                        .sensitive(false)
                        .build();

                    widgets.port_list_box.append(&empty_btn);
                } else {
                    for port in &self.available_ports {
                        let is_selected = self.selected_port.as_deref() == Some(port.as_str());

                        let content = adw::ButtonContent::builder()
                            .css_classes(vec!["flat".to_string()])
                            .label(port)
                            .halign(gtk::Align::Start)
                            .build();

                        let btn = gtk::Button::builder()
                            .css_classes(vec!["flat".to_string()])
                            .child(&content)
                            .build();

                        if is_selected {
                            btn.add_css_class("accent");
                        }

                        let port_name = port.clone();
                        let sender_clone = sender.clone();

                        btn.connect_clicked(move |_| {
                            sender_clone
                                .input(ApplicationModelInput::SelectPort(port_name.clone()));
                        });

                        widgets.port_list_box.append(&btn);
                    }
                }
            }

            ApplicationModelInput::UpdateDeviceStatus {
                port_name,
                pedal_model,
            } => {
                self.selected_port = port_name.clone();
                self.detected_pedal = pedal_model;

                if let Some(ref port) = port_name {
                    widgets
                        .port_list_button
                        .set_label(&Self::clean_port_name(port));
                } else {
                    widgets.port_list_button.set_label("Select MIDI Port");
                }

                if let Some(model) = pedal_model {
                    widgets.device_type_badge.set_label(&format!("{}", model));
                    widgets.device_type_badge.set_visible(true);
                    widgets.rescan_patches_button.set_sensitive(true);

                    sender.input(ApplicationModelInput::ScanPatches);
                } else {
                    widgets.device_type_badge.set_visible(false);
                    widgets.rescan_patches_button.set_sensitive(false);
                    sender.input(ApplicationModelInput::UpdatePatchesList(Vec::new()));
                }

                widgets.status_badge.remove_css_class("success");
                widgets.status_badge.remove_css_class("warning");
                widgets.status_badge.remove_css_class("error");
                widgets.status_badge.remove_css_class("dim-label");

                match (port_name, pedal_model) {
                    (Some(_), Some(model)) if model != PedalModel::Unknown => {
                        widgets.status_badge.set_label("Connected");
                        widgets.status_badge.add_css_class("success");
                    }
                    (Some(_), _) => {
                        widgets.status_badge.set_label("Unrecognized Device");
                        widgets.status_badge.add_css_class("warning");
                    }
                    (None, _) => {
                        widgets.status_badge.set_label("Disconnected");
                        widgets.status_badge.add_css_class("dim-label");
                    }
                }
            }

            ApplicationModelInput::ScanPatches => {
                if self.detected_pedal.is_none() {
                    return;
                }

                self.is_scanning_patches = true;
                self.scan_progress = 0.0;
                widgets.patch_progress_bar.set_visible(true);
                widgets.patch_progress_bar.set_fraction(0.0);
                widgets
                    .patch_progress_bar
                    .set_text(Some("Scanning patches..."));
                widgets.rescan_patches_button.set_sensitive(false);

                self.zoom_client_worker_controller
                    .emit(ZoomClientWorkerInput::ScanLibrary);
            }

            ApplicationModelInput::UpdateScanProgress { progress } => {
                self.scan_progress = progress;
                widgets.patch_progress_bar.set_fraction(progress);
                widgets
                    .patch_progress_bar
                    .set_text(Some(&format!("{:.0}%", progress * 100.0)));
            }

            ApplicationModelInput::UpdatePatchesList(patches) => {
                self.scanned_patches = patches;
                self.is_scanning_patches = false;
                widgets.patch_progress_bar.set_visible(false);
                widgets
                    .rescan_patches_button
                    .set_sensitive(self.detected_pedal.is_some());

                render_patch_list(widgets, &self.scanned_patches, &self.search_query, &sender);
            }

            ApplicationModelInput::FilterPatches(query) => {
                self.search_query = query;
                render_patch_list(widgets, &self.scanned_patches, &self.search_query, &sender);
            }

            ApplicationModelInput::SelectPatch(slot) => {
                self.current_patch_number = Some(slot);
                self.zoom_client_worker_controller
                    .emit(ZoomClientWorkerInput::SelectPatch(slot));
            }

            ApplicationModelInput::NotifyUiError(err) => {
                error!("UI Error: {}", err);
                let toast = adw::Toast::new(&err);
                widgets.toast_overlay.add_toast(toast);
            }

            ApplicationModelInput::RenamePatch(slot, patch) => {
                let entry = gtk::Entry::builder()
                    .text(&patch)
                    .activates_default(true)
                    .margin_top(12)
                    .build();

                let dialog = adw::MessageDialog::builder()
                    .transient_for(
                        widgets
                            .toast_overlay
                            .native()
                            .and_downcast_ref::<gtk::Window>()
                            .unwrap(),
                    )
                    .heading("Rename Patch")
                    .body(&format!("Enter a new name for \"{}\":", patch))
                    .extra_child(&entry)
                    .build();

                dialog.add_response("cancel", "Cancel");
                dialog.add_response("rename", "Rename");
                dialog.set_response_appearance("rename", adw::ResponseAppearance::Suggested);
                dialog.set_default_response(Some("rename"));
                dialog.set_close_response("cancel");

                let sender_clone = sender.clone();
                let old_name = patch.clone();
                dialog.connect_response(None, move |_, response| {
                    if response == "rename" {
                        let new_name = entry.text().to_string().trim().to_string();
                        if !new_name.is_empty() && new_name != old_name {
                            sender_clone.input(ApplicationModelInput::ConfirmRenamePatch {
                                slot,
                                new_name,
                            });
                        }
                    }
                });

                dialog.present();
            }

            ApplicationModelInput::ConfirmRenamePatch { slot, new_name } => {
                self.zoom_client_worker_controller
                    .emit(ZoomClientWorkerInput::RenamePatch(slot, new_name));
            }
            ApplicationModelInput::CopyPatch(slot, patch) => {
                println!("Copy patch {}: {}", slot, patch);
            }
            ApplicationModelInput::PastePatch(slot, patch) => {
                println!("Paste patch {}: {}", slot, patch);
            }
            ApplicationModelInput::ImportPatchFile(slot, patch) => {
                println!("Import patch to slot {} from file: {}", slot, patch);
            }
            ApplicationModelInput::ExportPatchFile(slot, patch) => {
                println!("Export patch from slot {} to file: {}", slot, patch);
            }
            ApplicationModelInput::ShowPatchAsText(slot, patch) => {
                println!("Show patch {} as text: {}", slot, patch);
            }
            ApplicationModelInput::ImportPatchAsText(slot, patch) => {
                println!("Import patch to slot {} as text: {}", slot, patch);
            }
            ApplicationModelInput::SavePatchToDevice(slot, patch) => {
                println!("Save patch from slot {} to device: {}", slot, patch);
            }
            ApplicationModelInput::DeletePatch(slot, patch) => {
                println!("Delete patch at slot {}: {}", slot, patch);
            }
            ApplicationModelInput::SetCurrentPatch(patch, patch_number) => {
                self.current_patch = Some(patch.clone());
                self.current_patch_number = Some(patch_number);

                info!(
                    "set patch {} at number {}",
                    self.current_patch.clone().unwrap().name,
                    self.current_patch_number.unwrap()
                );

                self.main_content_controller
                    .emit(MainContentModelInput::SetPatch(
                        Some(patch),
                        patch_number as usize,
                    ));
            }
            ApplicationModelInput::RenamedPatch(slot, name) => {
                if (slot as usize) < self.scanned_patches.len() {
                    self.scanned_patches[slot as usize] = name;
                    render_patch_list(widgets, &self.scanned_patches, &self.search_query, &sender);
                }
            }
            ApplicationModelInput::SelectSlot(slot) => {
                self.zoom_client_worker_controller
                    .emit(ZoomClientWorkerInput::SelectSlot(slot));
            }
            ApplicationModelInput::SelectedSlotForEdit(_val) => {}

            ApplicationModelInput::UiParamEdit {
                slot,
                param_idx,
                val,
            } => {
                info!("Sending parame edit from ui: slot={slot}, param_idx={param_idx}, val={val}");
                self.zoom_client_worker_controller
                    .emit(ZoomClientWorkerInput::UiParamEdit {
                        slot,
                        param_idx,
                        val,
                    });
            }

            ApplicationModelInput::HardwareParamUpdated {
                slot,
                param_idx,
                val,
            } => {
                info!(
                    "[Hardware sync message recieved: slot={slot}, param_idx={param_idx}, val={val}"
                );
                self.main_content_controller
                    .emit(MainContentModelInput::HardwareSyncParamValue {
                        slot,
                        param_idx,
                        val,
                    });
            }
            ApplicationModelInput::AutoSave { slot, param_idx } => {
                if self.auto_save {
                    if let Some(patch_number) = self.current_patch_number {
                        info!("Saving slot {slot}, number {patch_number}");
                        self.zoom_client_worker_controller
                            .emit(ZoomClientWorkerInput::StorePatch(patch_number));
                    }
                }
            }
        }
    }
}

/// Helper function to construct a sidebar `ListBoxRow` without an icon
fn create_sidebar_row(label_text: &str) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::new();

    let layout_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    layout_box.set_margin_start(8);
    layout_box.set_margin_end(8);
    layout_box.set_margin_top(8);
    layout_box.set_margin_bottom(8);

    let label = gtk::Label::new(Some(label_text));
    layout_box.append(&label);

    row.set_child(Some(&layout_box));
    row.set_widget_name(label_text);
    row
}

/// Helper to create the multi-section context menu model
fn create_patch_context_menu() -> gio::Menu {
    let menu = gio::Menu::new();

    // Section 1: Edit actions
    let section1 = gio::Menu::new();
    section1.append(Some("Rename"), Some("row.rename"));
    section1.append(Some("Copy"), Some("row.copy"));
    section1.append(Some("Paste"), Some("row.paste"));
    menu.append_section(None, &section1);

    // Section 2: File import/export
    let section2 = gio::Menu::new();
    section2.append(Some("Import Patch from File"), Some("row.import-file"));
    section2.append(Some("Export Patch to File"), Some("row.export-file"));
    menu.append_section(None, &section2);

    // Section 3: Text import/export
    let section3 = gio::Menu::new();
    section3.append(Some("Show Patch as Text"), Some("row.show-text"));
    section3.append(Some("Import Patch as Text"), Some("row.import-text"));
    menu.append_section(None, &section3);

    // Section 4: Device sync
    let section4 = gio::Menu::new();
    section4.append(Some("Save Patch to Device"), Some("row.save-device"));
    menu.append_section(None, &section4);

    // Section 5: Deletion
    let section5 = gio::Menu::new();
    section5.append(Some("Delete Patch"), Some("row.delete"));
    menu.append_section(None, &section5);

    menu
}

/// Helper function to render matching patches into the sidebar `ListBox`
fn render_patch_list(
    widgets: &mut ApplicationModelWidgets,
    scanned_patches: &[String],
    search_query: &str,
    sender: &ComponentSender<ApplicationModel>,
) {
    while let Some(child) = widgets.patch_list_box.first_child() {
        widgets.patch_list_box.remove(&child);
    }

    let filtered: Vec<(usize, &String)> = scanned_patches
        .iter()
        .enumerate()
        .filter(|(_, p)| p.to_lowercase().contains(&search_query.to_lowercase()))
        .collect();

    if filtered.is_empty() {
        let label_text = if scanned_patches.is_empty() {
            "No patches found"
        } else {
            "No matching patches"
        };

        let row = create_sidebar_row(label_text);
        row.set_sensitive(false);
        widgets.patch_list_box.append(&row);
    } else {
        for (real_slot, patch_name) in filtered {
            let row = create_sidebar_row(&format!("{:02}. {}", real_slot + 1, patch_name));
            let real_slot_u8 = real_slot as u8;

            let gesture = gtk::GestureClick::new();
            gesture.set_button(gdk::BUTTON_PRIMARY);

            let sender_clone = sender.clone();
            gesture.connect_pressed(move |_, _, _, _| {
                sender_clone.input(ApplicationModelInput::SelectPatch(real_slot_u8));
            });

            row.add_controller(gesture);

            let sender_clone = sender.clone();
            row.connect_activate(move |_| {
                info!("selecting {real_slot}");
                sender_clone.input(ApplicationModelInput::SelectPatch(real_slot_u8));
            });

            // Context Popover Menu Setup positioned on the right side of the sidebar row
            let menu_model = create_patch_context_menu();
            let popover = gtk::PopoverMenu::from_model(Some(&menu_model));
            popover.set_parent(&row);
            popover.set_has_arrow(true);
            popover.set_position(gtk::PositionType::Right);

            // Row-scoped Action Group
            let action_group = gio::SimpleActionGroup::new();

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let rename_action = gio::SimpleAction::new("rename", None);
            rename_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::RenamePatch(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&rename_action);

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let copy_action = gio::SimpleAction::new("copy", None);
            copy_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::CopyPatch(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&copy_action);

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let paste_action = gio::SimpleAction::new("paste", None);
            paste_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::PastePatch(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&paste_action);

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let import_file_action = gio::SimpleAction::new("import-file", None);
            import_file_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::ImportPatchFile(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&import_file_action);

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let export_file_action = gio::SimpleAction::new("export-file", None);
            export_file_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::ExportPatchFile(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&export_file_action);

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let show_text_action = gio::SimpleAction::new("show-text", None);
            show_text_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::ShowPatchAsText(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&show_text_action);

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let import_text_action = gio::SimpleAction::new("import-text", None);
            import_text_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::ImportPatchAsText(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&import_text_action);

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let save_device_action = gio::SimpleAction::new("save-device", None);
            save_device_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::SavePatchToDevice(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&save_device_action);

            let sender_clone = sender.clone();
            let patch = patch_name.to_string();
            let delete_action = gio::SimpleAction::new("delete", None);
            delete_action.connect_activate(move |_, _| {
                sender_clone.input(ApplicationModelInput::DeletePatch(
                    real_slot_u8,
                    patch.clone(),
                ));
            });
            action_group.add_action(&delete_action);

            row.insert_action_group("row", Some(&action_group));

            let gesture = gtk::GestureClick::new();
            gesture.set_button(gdk::BUTTON_SECONDARY);
            gesture.connect_pressed(move |_, _, x, y| {
                popover.set_pointing_to(Some(&gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
                popover.popup();
            });
            row.add_controller(gesture);

            widgets.patch_list_box.append(&row);
        }
    }
}

impl ApplicationModel {
    pub fn clean_port_name(raw_name: &str) -> String {
        let parts: Vec<&str> = raw_name.split(':').collect();
        let name = parts.first().copied().unwrap_or(raw_name).trim();

        if name.is_empty() {
            raw_name.trim().to_string()
        } else {
            name.to_string()
        }
    }
}
