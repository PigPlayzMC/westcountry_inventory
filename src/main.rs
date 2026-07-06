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

    // Text display
    frame::Frame,

    // Images
    image::PngImage,
};

const WINDOW_WIDTH_MINIMUM: i32 = 600;
const WINDOW_HEIGHT_MINIMUM: i32 = 480;

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
    let mut menu = menu::SysMenuBar::default().with_size(800, 35);

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

    let mut message_frame = Frame::default()
	.with_size(200, 75)
	.center_of(&window);
    message_frame.set_frame(FrameType::EngravedFrame);
    message_frame.set_label("Test Message");
    message_frame.hide();

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
		println!("About screen");
	    },
	    "&About/&License\t" => {
		println!("FREE SOFTWARE");
	    },
	    _ => unreachable!(),
	};
    };
}
