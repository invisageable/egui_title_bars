use eframe::egui::{self};
use egui_title_bars::CustomTitleBar;

fn main() -> eframe::Result {
  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_decorations(false) // Hide the OS-specific "chrome" around the window
      .with_inner_size([400.0, 100.0])
      .with_min_inner_size([400.0, 100.0])
      .with_transparent(true), // To have rounded corners we need transparency

    ..Default::default()
  };
  eframe::run_native(
    "Custom window frame", // unused title
    options,
    Box::new(|_cc| Ok(Box::<MyApp>::default())),
  )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything
                                       // behind the rounded corners
  }

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    custom_window_frame(ctx, |ui| {
      ui.label("This is just the contents of the window.");
      ui.horizontal(|ui| {
        ui.label("egui theme:");
        egui::widgets::global_dark_light_mode_buttons(ui);
      });
    });
  }
}

fn custom_window_frame(
  ctx: &egui::Context,
  add_contents: impl FnOnce(&mut egui::Ui),
) {
  use egui::*;

  let panel_frame = egui::Frame {
    fill: ctx.style().visuals.window_fill(),
    rounding: 10.0.into(),
    stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
    outer_margin: 0.5.into(), // so the stroke is within the bounds
    ..Default::default()
  };

  CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
    let app_rect = ui.max_rect();

    let title_bar_height = 32.0;
    let title_bar_rect = {
      let mut rect = app_rect;
      rect.max.y = rect.min.y + title_bar_height;
      rect
    };

    ui.title_bar();

    // Add the contents:
    let content_rect = {
      let mut rect = app_rect;
      rect.min.y = title_bar_rect.max.y;
      rect
    }
    .shrink(4.0);
    let mut content_ui = ui.child_ui(content_rect, *ui.layout(), None);
    add_contents(&mut content_ui);
  });
}
