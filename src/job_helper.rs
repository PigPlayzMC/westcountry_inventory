use std::fmt::{
	self,
};

use fltk::{
	button::{self, Button}, group, prelude::{
		DisplayExt,
		GroupExt,
		WidgetExt,
	}, text,
};

pub struct Job {
	device_name: String,
	description: String,
	notes: String,
	last_modified: u32, //TODO Change to timestamp
	added: u32, //TODO Change to timestamp
	status: bool, //TODO Enum
}

impl Job {
	pub fn new(device_name: &str, description: &str, notes: &str, last_modified: u32, added: u32, status: bool) -> Job {
		return Job {
			device_name: device_name.to_string(),
			description: description.to_string(),
			notes: notes.to_string(),
			added: added,
			last_modified: last_modified,
			status: status,
		};
	}
}

pub trait Widget {
	fn make_widget(&self) -> group::Flex;
}

impl Widget for Job {
	fn make_widget(&self) -> group::Flex {
		let mut widget: group::Flex = group::Flex::default().with_size(100, 100).column();
		widget.set_spacing(0);

		let visit_button: Button = button::Button::default().with_label(&self.device_name);

		widget.fixed(&visit_button, 25);

		let mut desciption_display: text::TextDisplay = text::TextDisplay::default();

		let mut description_buffer: text::TextBuffer = text::TextBuffer::default();
		description_buffer.set_text(&self.description);

		desciption_display.set_buffer(description_buffer);
		desciption_display.wrap_mode(text::WrapMode::AtBounds, 0);

		widget.end();

		widget // Implicit return
	}
}

impl fmt::Display for Job { // Used for debug while testing
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f, 
			"{} - {}\nNOTES: {}\nAdded: {}\nLast Modified: {}\nStatus: {}",
			self.device_name,
			self.description,
			self.notes,
			self.last_modified,
			self.added,
			self.status
		)
	}
}