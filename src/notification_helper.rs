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
	let mut notification_window: Window = window::Window::default().with_size(320, 200).with_label(&title);
	notification_window.set_border(true);
	notification_window.make_modal(true);
	notification_window.end();

	Self { notification_window }
    }
}
