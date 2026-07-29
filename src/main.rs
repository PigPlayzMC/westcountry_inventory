// Extern. imports
use fltk::{
    // Basic feature set
    app,
    app::App,

    // Window management
    window::{
	DoubleWindow,
	Window,
    },

    // Menus
    menu,

    // Additional QoL & menus
    prelude::*,
    enums::*,

    // Images
    image::PngImage,
};

// Local imports
mod notification_helper;
mod button_consts;

const WINDOW_WIDTH_MINIMUM: i32 = 600;
const WINDOW_HEIGHT_MINIMUM: i32 = 480;

struct State {
    changes: bool,
}

struct Job {
    device_name: String,
    description: String,
    notes: String,
    last_modified: u32, //TODO Change to timestamp
    added: u32, //TODO Change to timestamp
    status: bool, //TODO Enum
}

fn main() {
    println!("DEBUG: GUI starting");

    // Start async network interface
    //TODO

    // Start FLTK app
    let gui_control: App = App::default().with_scheme(app::Scheme::Gtk);
    app::get_system_colors();
    app::set_selection_color(15, 52, 131);

    let mut window: DoubleWindow = Window::default()
	.with_size(WINDOW_WIDTH_MINIMUM, WINDOW_HEIGHT_MINIMUM)
	.with_label("IT Job Management Portal");
    window.set_xclass("Itjmp");

    let icon: PngImage = PngImage::load("images/icon.png").unwrap();
    //TODO Error handling
    
    window.set_icon(Some(icon));

    // Menu
    let mut menu: menu::SysMenuBar = menu::SysMenuBar::default().with_size(800, 35);

    // Job specific actions
    menu.add(
	"&Jobs/&Refresh\t",
	Shortcut::Ctrl | 'r',
	menu::MenuFlag::MenuDivider,
	menu_choice_button,
    );
    menu.add(
	"&Jobs/&Commit\t",
	Shortcut::Ctrl | 's',
	menu::MenuFlag::Normal,
	menu_choice_button,
    );
    menu.add(
	"&Jobs/&Discard\t",
	Shortcut::Ctrl | 'd',
	menu::MenuFlag::Normal,
	menu_choice_button,
    );
    
    // About this software section
    menu.add(
	"&About/&About\t",
	Shortcut::None,
	menu::MenuFlag::Normal,
	menu_choice_button,
    );
    menu.add(
	"&About/&License\t",
	Shortcut::None,
	menu::MenuFlag::Normal,
	menu_choice_button,
    );

    if let Some(mut entry) = menu.find_item("&Jobs/&Discard\t") {
	entry.set_label_color(Color::Red);
	// Sets the colour of discard button to differentiate from other
	// less destructive buttons
    };

    menu.end();

    window.make_resizable(true);
    //TODO make window take previous size and position

    window.end(); // Close the window parenting scope
    window.show();

    gui_control.run().unwrap();
}

// Menu buttonpress handling
fn menu_choice_button(menu: &mut impl MenuExt) {
    if let Ok(menu_path) = menu.item_pathname(None) {
	////println!("{}", menu_path.as_str());
	
	match menu_path.as_str() {
	    "&Jobs/&Refresh\t" => {
		println!("Refresh button clicked");
	    },
	    "&Jobs/&Commit\t" => {
		println!("Commit");
	    },
	    "&Jobs/&Discard\t" => {
		println!("Discard");
	    },
	    
	    "&About/&About\t" => {
        show_notification(
            "About this program", 
            "This program is a Free job management platform for IT technicians.", 
            button_consts::OK_BUTTON,
        );
	    },
	    "&About/&License\t" => {
		show_notification(
		    "License",
		    "This program is Free Software, licensed under the terms of the GNU General Public License Version 3, a free, copyleft license. A copy of this license should be distributed with the source code or found at https://www.gnu.org/licenses/gpl-3.0.html",
		    button_consts::OK_BUTTON,
		);
	    },
	    _ => unreachable!(),
	};
    };
}

fn show_notification(title: &str, body: &str, button_set: u8) {
    let mut window = notification_helper::NotificationPopup::new(title.to_string(), body.to_string(), button_set);
    window.notification_window.show();
}

