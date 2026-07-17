use fltk::{
    enums::*,
    prelude::*,
    window::Window,
    *
};
use std::ops::{Deref, DerefMut};

pub struct NotificationButton {
    button: button::Button,
}

impl NotificationButton {
    pub fn new(label: String) -> Self {
	let mut button = button::Button::default().with_label(&label);
	button.set_frame(FrameType::EngravedFrame);

	Self { button }
    }
}

impl Deref for NotificationButton {
    type Target = button::Button;

    fn deref(&self) -> &Self::Target {
	&self.button
    }
}

impl DerefMut for NotificationButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
	&mut self.button
    }
}

pub struct NotificationPopup {
    pub notification_window: window::Window,
}

impl NotificationPopup {
    pub fn new(title: String, body: String, button_set: u8) -> Self {
	let mut notification_window: Window = window::Window::default().with_size(360, 200).with_label(&title);
	notification_window.set_border(true);
	notification_window.make_modal(true);
	// TODO REDO using text display
	let mut body_text = text::TextDisplay::default()
	    .with_size(340, 180)
	    .center_of_parent();

	let mut body_buffer = text::TextBuffer::default();
	body_buffer.set_text(&body);
	
	body_text.set_buffer(body_buffer);
	body_text.wrap_mode(text::WrapMode::AtBounds, 0);
	
	notification_window.end();

	Self { notification_window }
    }
}
