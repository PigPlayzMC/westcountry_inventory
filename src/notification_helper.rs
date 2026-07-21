use fltk::{
    button,
	prelude::*,
	window,
	text,
};

use crate::{button_consts};

pub struct NotificationPopup {
    pub notification_window: window::Window,
}

const WIDTH: i32 = 400;
const HEIGHT: i32 = 200;

const BUTTON_WIDTH: i32 = 70;
const BUTTON_HEIGHT: i32 = 30;

const SPACING: i32 = 10; // Controls how far elements are bounded inside the window

impl NotificationPopup {
    pub fn new(title: String, body: String, button_set: u8) -> Self {
	let mut notification_window: window::Window = window::Window::default().with_size(WIDTH, HEIGHT).with_label(&title);
	notification_window.set_border(true);
	notification_window.make_modal(true);

	let mut body_text = text::TextDisplay::default()
	    .with_size(WIDTH - SPACING*2, HEIGHT - 70)
	    .with_pos(SPACING, SPACING);

	let mut body_buffer = text::TextBuffer::default();
	body_buffer.set_text(&body);
	
	body_text.set_buffer(body_buffer);
	body_text.wrap_mode(text::WrapMode::AtBounds, 0);

	match button_set {
		button_consts::OK_BUTTON => {
			let mut ok_button: button::Button = button::Button::default()
				.with_size(BUTTON_WIDTH, BUTTON_HEIGHT)
				.with_label("Ok")
				.with_pos(WIDTH - BUTTON_WIDTH - SPACING, HEIGHT - BUTTON_HEIGHT - SPACING);

			ok_button.set_callback({
				let mut notification_window = notification_window.clone();
				
				move |_| {
					notification_window.hide();
				}
			});
		},
		_ => {
			eprintln!("ERROR: Unknown button set {button_set}, please report this.");

			// TODO Kill window and spawn error dialog
		},
	};
	
	notification_window.end();

	Self { notification_window }
    }
}