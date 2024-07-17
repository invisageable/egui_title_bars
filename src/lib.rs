use eframe::egui::Widget;

/// The Custom Title Bar Interface.
pub trait CustomTitleBar {
  fn title_bar(&mut self) -> eframe::egui::Response;
}

#[derive(Default)]
pub struct TitleBar {
  text: Option<eframe::egui::WidgetText>,
  decorations: bool,
  align: eframe::egui::Align,
}

impl TitleBar {
  #[inline]
  pub fn new(text: impl Into<eframe::egui::WidgetText>) -> Self {
    Self::with_text(Some(text.into()))
  }

  #[inline]
  pub fn with_text(text: Option<eframe::egui::WidgetText>) -> Self {
    Self {
      text,
      decorations: true,
      align: eframe::egui::Align::LEFT,
    }
  }

  #[inline]
  pub fn show_decorations(mut self, decorations: bool) -> Self {
    self.decorations = decorations;
    self
  }

  #[inline]
  pub fn align_decorations(mut self, align: eframe::egui::Align) -> Self {
    self.align = align;
    self
  }
}

impl CustomTitleBar for eframe::egui::Ui {
  fn title_bar(&mut self) -> eframe::egui::Response {
    TitleBar::new("title").ui(self)
  }
}

impl eframe::egui::Widget for TitleBar {
  fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
    let app_rect = ui.max_rect();

    let title_bar_height = 32.0;
    let title_bar_rect = {
      let mut rect = app_rect;
      rect.max.y = rect.min.y + title_bar_height;
      rect
    };

    let painter = ui.painter();

    let title_bar_response = ui.interact(
      title_bar_rect,
      eframe::egui::Id::new("title_bar"),
      eframe::egui::Sense::click_and_drag(),
    );

    // Paint the title:
    painter.text(
      title_bar_rect.center(),
      eframe::egui::Align2::CENTER_CENTER,
      self.text.unwrap().text(),
      eframe::egui::FontId::proportional(20.0),
      ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
      [
        title_bar_rect.left_bottom() + eframe::egui::Vec2::new(1.0, 0.0),
        title_bar_rect.right_bottom() + eframe::egui::Vec2::new(-1.0, 0.0),
      ],
      ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
      let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

      ui.ctx()
        .send_viewport_cmd(eframe::egui::ViewportCommand::Maximized(
          !is_maximized,
        ));
    }

    if title_bar_response.drag_started_by(eframe::egui::PointerButton::Primary)
    {
      ui.ctx()
        .send_viewport_cmd(eframe::egui::ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
      ui.with_layout(
        match self.align {
          eframe::egui::Align::LEFT => {
            eframe::egui::Layout::left_to_right(eframe::egui::Align::Center)
          }
          eframe::egui::Align::RIGHT => {
            eframe::egui::Layout::right_to_left(eframe::egui::Align::Center)
          }
          _ => todo!("error: not available alignment — left/right only."),
        },
        |ui| {
          ui.spacing_mut().item_spacing.x = 0.0;
          ui.visuals_mut().button_frame = false;
          ui.add_space(8.0);
          ui.spacing_mut().item_spacing.x = 8.0;
          close_maximize_minimize(ui);
        },
      );
    })
    .response
  }
}

fn close_maximize_minimize(ui: &mut eframe::egui::Ui) {
  let close_response = ui.add(Decoration::close()).on_hover_ui(|ui| {
    ui.label("text");
  });

  if close_response.clicked() {
    ui.ctx()
      .send_viewport_cmd(eframe::egui::ViewportCommand::Close);
  }

  let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

  let minimized_response = ui.add(Decoration::minimize()).on_hover_ui(|ui| {
    ui.label("text");
  });

  if minimized_response.clicked() {
    ui.ctx()
      .send_viewport_cmd(eframe::egui::ViewportCommand::Minimized(true));
  }

  if is_maximized {
    let maximized_response = ui.add(Decoration::maximize()).on_hover_ui(|ui| {
      ui.label("text");
    });

    if maximized_response.clicked() {
      ui.ctx()
        .send_viewport_cmd(eframe::egui::ViewportCommand::Maximized(false));
    }
  } else {
    let maximized_response = ui.add(Decoration::maximize()).on_hover_ui(|ui| {
      ui.label("text");
    });

    if maximized_response.clicked() {
      ui.ctx()
        .send_viewport_cmd(eframe::egui::ViewportCommand::Maximized(true));
    }
  }
}

#[derive(Default)]
pub struct Decoration {
  color: eframe::egui::Color32,
}

impl Decoration {
  const WIDTH: f32 = 12.0;

  #[inline]
  pub fn new(color: eframe::egui::Color32) -> Self {
    Self { color }
  }

  #[inline]
  pub fn close() -> Self {
    Self::new(eframe::egui::Color32::from_rgb(255, 97, 87))
  }

  #[inline]
  pub fn maximize() -> Self {
    Self::new(eframe::egui::Color32::from_rgb(42, 203, 66))
  }

  #[inline]
  pub fn minimize() -> Self {
    Self::new(eframe::egui::Color32::from_rgb(255, 193, 47))
  }
}

impl eframe::egui::Widget for Decoration {
  fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
    let response = ui
      .allocate_response([Self::WIDTH; 2].into(), eframe::egui::Sense::click());

    ui.painter().rect_filled(response.rect, 50.0, self.color);

    response
  }
}

// encapsulated decoration in a control struct.
