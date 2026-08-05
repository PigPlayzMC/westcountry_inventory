use std::fmt::{
	self,
};

use fltk::{
	frame, group, prelude::{
		DisplayExt, GroupExt, WidgetExt, WindowExt,
	}, text, window,
};

// ## Job components

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

		let mut visit_frame: frame::Frame = frame::Frame::default().with_label(&self.device_name);
		visit_frame.set_frame(fltk::enums::FrameType::BorderBox);

		widget.fixed(&visit_frame, 25);

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

// ## Job view components

pub struct JobView {
	modified: bool,
}

pub trait CreateView {
	fn create_view(job: &Job) -> ();
}

impl CreateView for JobView {
	fn create_view(job: &Job) -> () {
		let mut job_view_window: window::Window = window::Window::default()
			.with_size(400, 400)
			.with_label(&job.device_name);
		job_view_window.make_resizable(true);

		let mut description_buffer: text::TextBuffer = text::TextBuffer::default();

		description_buffer.set_text(&job.description);

		let mut description_editor = text::TextEditor::default()
			.with_size(400, 400)
			.with_pos(0, 0);

		description_editor.set_buffer(description_buffer);

		job_view_window.end();

		job_view_window.show();
		job_view_window.make_current();
	}
}