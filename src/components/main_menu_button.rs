use adw::prelude::*;
use relm4::{Component, prelude::*};

pub struct MainMenuButton;

#[relm4::component(pub)]
impl Component for MainMenuButton {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::MenuButton {
            set_icon_name: "open-menu-symbolic",
            set_tooltip_text: Some("Main Menu"),
            add_css_class: "flat",

            #[wrap(Some)]
            set_popover = &gtk::PopoverMenu::from_model(Some(&{
                let menu = gtk::gio::Menu::new();

                let section = gtk::gio::Menu::new();

                section.append(Some("About Zoom MS Utility"), Some("app.about"));

                menu.append_section(None, &section);

                let quit_window_section = gtk::gio::Menu::new();

                let quit_window_item = gtk::gio::MenuItem::new(Some("Quit"), Some("app.quit"));
                quit_window_item.set_attribute_value("accel", Some(&"<Primary>Q".to_variant()));

                quit_window_section.append_item(&quit_window_item);

                menu.append_section(None, &quit_window_section);

                menu
            })) {}
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MainMenuButton {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}