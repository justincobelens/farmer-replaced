use std::sync::{Arc, mpsc::Receiver};

use eframe::{
	self,
	egui::{self},
};
use engine::World;

use crate::window::Window;

pub struct App {
	receiver: Receiver<Arc<World>>,
	world: Option<Arc<World>>,
	latest_points: Vec<(f32, f32)>,
	panel: Window,
}

impl App {
	pub fn new(receiver: Receiver<Arc<World>>) -> Self {
		let panel = Window::new(200.0, 200.0);
		Self {
			receiver,
			world: None,
			latest_points: Vec::new(),
			panel,
		}
	}

	fn pull_latest(&mut self) {
		while let Ok(world) = self.receiver.try_recv() {
			self.world = Some(world);
		}
	}

	fn extract_latest_transforms(&mut self) {
		if let Some(world) = &self.world {
			self.latest_points = world
				.get_all_actors()
				.into_iter()
				.map(|actor| {
					let location = actor.get_transform().location;
					(location.x, location.y)
				})
				.collect();
		}
	}

	/// Normalize world space points into 0..1 panel space to make them fit inside the rect
	fn normalize_points(&self, rect: egui::Rect) -> Vec<egui::Pos2> {
		if self.latest_points.is_empty() {
			return Vec::new();
		}

		let span_x = self.panel.width.max(1.0);
		let span_y = self.panel.height.max(1.0);

		self.latest_points
			.iter()
			.map(|(x, y)| {
				let nx = (x / span_x).clamp(0.0, 1.0);
				let ny = (y / span_y).clamp(0.0, 1.0);
				egui::pos2(
					rect.left() + nx * rect.width(),
					rect.bottom() - ny * rect.height(),
				)
			})
			.collect()
	}

	fn update_locations(&mut self, painter: &egui::Painter, rect: egui::Rect) {
		if self.latest_points.is_empty() {
			painter.text(
				rect.center(),
				egui::Align2::CENTER_CENTER,
				"No actor positions yet",
				egui::FontId::proportional(14.0),
				egui::Color32::GRAY,
			);
			return;
		}

		for pos in self.normalize_points(rect) {
			painter.circle_filled(pos, 10.0, egui::Color32::LIGHT_BLUE);
		}
	}

	fn show(&mut self, ui: &mut egui::Ui) {
		ui.heading("Actor positions");
		ui.label(format!("Active actors: {}", self.latest_points.len()));

		ui.set_width(self.panel.width);
		ui.set_height(self.panel.height);

		let desired = ui.available_size();

		let (response, painter) = ui.allocate_painter(desired, egui::Sense::hover());
		let rect = response.rect;

		painter.rect_stroke(
			rect,
			0.0,
			egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
			egui::StrokeKind::Inside,
		);

		self.update_locations(&painter, rect);
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.pull_latest();
		self.extract_latest_transforms();
		ctx.request_repaint();

		// let pointer = ctx.pointer_latest_pos();
		// println!("Pointer position: {:?}", pointer);

		egui::CentralPanel::default().show(ctx, |ui| self.show(ui));
	}
}
