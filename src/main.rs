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

    // Menu
    let mut menu = menu::SysMenuBar::default().with_size(800, 35);
    menu.set_frame(FrameType::FlatBox);
    
    menu.add(
	"&Jobs/&Refresh\t",
	Shortcut::Ctrl | 'r',
	menu::MenuFlag::Normal,
	menu_choice_button,
    );

    window.make_resizable(true);
    //TODO make window take previous size and position

    window.end(); // Close the window parenting scope
    window.show();

    gui_control.run().unwrap(); //TODO Error handling
}

// Menu button press handling
fn menu_choice_button(menu: &mut impl MenuExt) {
    if let Ok(menu_path) = menu.item_pathname(None) {
	////println!("{}", menu_path.as_str());
	
	match menu_path.as_str() {
	    "&Jobs/&Refresh\t" => {
		println!("Refresh button clicked");
	    },
	    _ => unreachable!(),
	};
    };
}
