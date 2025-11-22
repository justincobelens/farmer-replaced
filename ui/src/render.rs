use std::{
	sync::{
		Arc, OnceLock,
		mpsc::{self, Sender},
	},
	thread,
};

use winit::platform::{wayland, x11};

use eframe::{self};
use engine::World;

use crate::app::App;

const TITLE_APP: &str = "Farmer Replaced";

struct UiHandle {
	sender: Sender<Arc<World>>,
}

pub fn render_pipeline(world: &World) {
	static UI_THREAD: OnceLock<UiHandle> = OnceLock::new();

	let handle = UI_THREAD.get_or_init(|| spawn_ui_thread());

	let _ = handle.sender.send(Arc::new(world.clone()));
}

fn spawn_ui_thread() -> UiHandle {
	let (tx, rx) = mpsc::channel::<Arc<World>>();

	thread::spawn(move || {
		let native_options = create_native_options();

		let app = App::new(rx);
		if let Err(err) = eframe::run_native(
			TITLE_APP,
			native_options,
			Box::new(move |_cc| Ok(Box::new(app))),
		) {
			eprintln!("Failed to start egui window: {err}");
		}
	});

	UiHandle { sender: tx }
}

fn create_native_options() -> eframe::NativeOptions {
	eframe::NativeOptions {
		event_loop_builder: Some(Box::new(|builder| {
			#[cfg(target_os = "linux")]
			{
				wayland::EventLoopBuilderExtWayland::with_any_thread(builder, true);
				x11::EventLoopBuilderExtX11::with_any_thread(builder, true);
			}
		})),
		centered: true,
		..Default::default()
	}
}
