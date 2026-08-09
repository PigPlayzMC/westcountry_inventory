#![deny(clippy::unwrap_used)]

// Extern. imports
use fltk::{
	app::{
		self,
		App,
	},
	window::{
		DoubleWindow,
		Window,
	},
	enums::*,
	group::Grid,
	image::PngImage,
	menu,
	prelude::*,
};

use std::{
	sync::{
		mpsc,
		Mutex,
		Arc,
	}, // Cross thread communication (multiple producer single consumer)
};

//# Local imports
// Helpers
mod notification_helper;
mod job_helper;

// Constants
mod button_consts;

//# Local uses
use job_helper::{
	Job,
	Defaults,
	Widget,
	JobView,
	CreateView,
};

const WINDOW_WIDTH_MINIMUM: i32 = 600;
const WINDOW_HEIGHT_MINIMUM: i32 = 480;

const MENU_HEIGHT: i32 = 35;

const JOB_ROWS_MAX: i32 = 5;
const JOB_COLUMNS_MAX: i32 = 7;

struct State {
	changes: bool,
}

fn main() {
	println!("DEBUG: GUI starting");

	// Start async network interface
	//TODO

	// Initialise state
	#[allow(unused)]
	let mut state: State = State { changes: false};

	// Start FLTK app
	let gui_control: App = App::default().with_scheme(app::Scheme::Gtk);
	app::get_system_colors();
	app::set_selection_color(15, 52, 131);
	app::set_font(Font::Courier);

	let mut window: DoubleWindow = Window::default()
	.with_size(WINDOW_WIDTH_MINIMUM, WINDOW_HEIGHT_MINIMUM)
	.with_label("IT Job Management Portal");
	window.set_xclass("Itjmp");

	let icon: PngImage = PngImage::load("images/icon.png").expect("Image should exist and be loaded correctly");
	//TODO Error handling
	
	window.set_icon(Some(icon));

	// Menu
	let mut menu: menu::SysMenuBar = menu::SysMenuBar::default().with_size(800, MENU_HEIGHT);

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

	// ### Example job ###
	let example_job: Job = Job::new(
		"EXAMPLE", 
		"description", 
		"none", 
		0, 
		0,
		false
	);

	// Creates an array to store displayed jobs in, and fills the array with default jobs (default jobs must not be displayed)
	let job_array: Arc<Mutex<[Job; (JOB_COLUMNS_MAX*JOB_ROWS_MAX) as usize]>> = Arc::new(core::array::from_fn(|_| Job::default()).into());
	// This creates a thread safe array which can be cloned and updated to avoid issues with borrowing / moving into/out of closures
	
	let mut job_grid: Grid = Grid::new(0, MENU_HEIGHT, WINDOW_HEIGHT_MINIMUM, WINDOW_HEIGHT_MINIMUM, "");
	let layout: [i32; 2] = get_grid_dimensions(window.width(), window.height());
	job_grid.set_layout(layout[1], layout[0]);
	job_grid.set_margin(0, 0, 0, 0);

	let job: &mut fltk::group::Flex = &mut Job::make_widget(&example_job);

	// Example system TODO Implement fully
	let row: usize = 0;
	let index: usize = 0;
	job_array.lock().expect("This thread should not already hold a lock")[row + index] = example_job;
	
	// Click handling
	let job_array_clone_to_handle: Arc<Mutex<[Job; (JOB_COLUMNS_MAX*JOB_ROWS_MAX) as usize]>> = Arc::clone(&job_array);
	job.handle(move |_widget: &mut fltk::group::Flex, ev: Event| {
		match ev {
			Event::Push => {
				let (sender, reciever) = mpsc::channel::<Job>(); // Create communication channel
				let index: i32 = 0;

				let job_to_view: Job = job_array_clone_to_handle.lock().expect("This thread should not already hold a lock")[index as usize].clone();
				let mut job_view_window: DoubleWindow = JobView::create_view(&job_to_view, sender);

				let job_array_clone: Arc<Mutex<[Job; 35]>> = Arc::clone(&job_array);
				job_view_window.handle(move |_window: &mut Window, ev: Event| {
					match ev {
						Event::Hide => {
							////println!("Hide callback, main thread!");

							match reciever.try_recv() {
								Ok(modified_job) => {
									println!("{}", modified_job);

									//job_array[index as usize] = <Job as Clone>::clone(&*&modified_job);
									job_array_clone.lock().expect("This thread should not already hold a lock")[index as usize] = modified_job;

									state.changes = true;
								},
								Err(mpsc::TryRecvError::Empty) => {
									// No message recieved, job must have been discarded.
									println!("Job discarded.");
								},
								Err(mpsc::TryRecvError::Disconnected) => {
									eprintln!("Error: Cross thread job view reciever disconnected");
								},
							};

							true
						},
						_ => false,
					}
				});

				////println!("{}", job_to_insert);

				// Handled results must return true
				true
			},
			_ => false, // Unhandled results must return false
		}
	});

	if state.changes {
		println!("changes");
	};

	////println!("{:?}", job.trigger());
	
	let _ = job_grid.set_widget(job, 0, 0);

	/* Debug settings for alignment */
	////job_grid.debug(1);
	////job_grid.show_grid(true);

	window.make_resizable(true);
	//TODO make window take previous size and position

	window.end(); // Close the window parenting scope

	window.resize_callback(move |_window: &mut DoubleWindow, _x: i32, _y: i32, width: i32, height: i32| {
		// Ensure size formatting remains constant despite window resizing
		menu.resize(0, 0, width, MENU_HEIGHT);

		job_grid.set_pos(0, MENU_HEIGHT);
		job_grid.set_size(width, height);

		let layout: [i32; 2] = get_grid_dimensions(width, height);
		job_grid.set_layout(layout[0], layout[1]);
	});

	window.show();

	gui_control.run().expect("App should run correctly");
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
			println!("@Commit");
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
				"This program is Free Software, licensed under the terms of the GNU General Public License Version 3, a free, copyleft license. A copy of this license should be distributed with the source code, or found at https://www.gnu.org/licenses/gpl-3.0.html",
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

fn get_grid_dimensions(window_width: i32, window_height: i32) -> [i32; 2] {
	// Works out how many job widgets can be added to the job grid, at size 100x100 pixels
	let mut rows: i32 = ((window_height - MENU_HEIGHT) as f32 / 100.0).floor() as i32;
	let mut columns: i32 = (window_width as f32 / 100.0).floor() as i32;
	
	if rows > JOB_ROWS_MAX {
		rows = JOB_ROWS_MAX;
	};
	if columns > JOB_COLUMNS_MAX {
		columns = JOB_COLUMNS_MAX;
	};

	[ rows, columns ]
}
