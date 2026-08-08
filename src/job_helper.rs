use std::{
	fmt,
	sync::mpsc::Sender
};

use fltk::{
	button,
	enums,
	frame,
	group,
	text,
	prelude::{
		DisplayExt,
		GroupExt,
		WidgetExt,
		WindowExt,
	},
	window::{
		self,
		Window
	},
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

impl Clone for Job {
	fn clone(&self) -> Job {
		return Job {
			device_name: self.device_name.clone(),
			description: self.description.clone(),
			notes: self.notes.clone(),
			added: self.added,
			last_modified: self.last_modified,
			status: self.status,
		}
	}
}

pub trait Defaults {
	fn default() -> Job;
}

impl Defaults for Job {
	fn default() -> Job {
		return Job {
			device_name: "DEFAULT".to_string(),
			description: "THIS JOB SHOULD NOT APPEAR".to_string(),
			notes: "".to_string(),
			added: 0,
			last_modified: 0,
			status: false,
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
	// No components to this struct, purely exists to provide a method
}

pub trait CreateView {
	fn create_view(job: &Job, sender: Sender<Job>) -> Window;
}

impl CreateView for JobView { // TODO MAKE RETURN VALUE
	fn create_view(job: &Job, sender: Sender<Job>) -> Window {
		let mut job_view_window: window::Window = window::Window::default()
			.with_size(400, 400)
			.with_label(&job.device_name);
		job_view_window.make_resizable(true);

		let mut description_buffer: text::TextBuffer = text::TextBuffer::default();

		description_buffer.set_text(&job.description);

		let mut description_editor = text::TextEditor::default()
			.with_size(400, 350)
			.with_pos(0, 0);

		description_editor.set_buffer(description_buffer);
		description_editor.set_cursor_style(text::Cursor::Block);
		description_editor.wrap_mode(text::WrapMode::AtBounds, 0);

		let mut discard_button: button::Button = button::Button::default()
			.with_label("Discard")
			.with_pos(0, 350)
			.with_size(100, 50);

		discard_button.set_label_color(enums::Color::Red);

		discard_button.set_callback( {
			let mut job_view_window: window::Window = job_view_window.clone();

			move |_| {
				////println!("Discard button");

				////sender.try_send(job);

				job_view_window.hide();
			}
		});

		let mut save_button: button::Button = button::Button::default()
			.with_label("Save and exit")
			.with_pos(300, 350)
			.with_size(100, 50);

		save_button.set_callback({
			////println!("{}", modified_job);
			////println!("{}", &description_editor.buffer().expect("Editor should have an associated buffer").text());

			let mut job_view_window: window::Window = job_view_window.clone();
			let job: Job = job.clone();

			move |_| {
				////println!("Save and exit button");
				// TODO Saving code
				let modified_job: Job = Job::new(
					&job.device_name, 
					&description_editor.buffer().expect("Editor should have an associated buffer").text(),
					&job.notes, 
					job.last_modified, 
					job.added, 
					job.status
				);

				////println!("{}", modified_job);
				////println!("{}", &description_editor.buffer().expect("Editor should have an associated buffer").text());

				match sender.send(modified_job) {
					Ok(_) => {
						// This is fine, the data will be recieved on the main thread. NOTE: This does not mean the data has already been recieved.
					},
					Err(e) => {
						eprintln!("Error: Job view reciever unavaliable ({})", e);
					},
				};

				job_view_window.hide();
			}
		});

		job_view_window.end();

		job_view_window.make_modal(true);
		job_view_window.show();

		return job_view_window
	}
}