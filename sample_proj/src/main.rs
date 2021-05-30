#[macro_use]

extern crate conrod;
extern crate find_folder;

use conrod::{widget, color, Colorable, Borderable, Sizeable, Positionable, Labelable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};

widget_ids!(
    struct Ids {
        canvas,
        title,
        text_box,
        button,
        result,
    });

fn increment(x: u64) -> u64 {
    x + 1
}

fn main() {

    const TITLE: &'static str = "Increment";

    let width = 300;
    let height = 100;

    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(width, height)
        .with_title(TITLE)
        .with_multisampling(4)
        .build_glium()
        .unwrap();

    let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();

    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");

    ui.fonts.insert_from_file(font_path).unwrap();

    let ids = &mut Ids::new(ui.widget_id_generator());

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut text = "0".to_string();
    let mut answer = "1".to_string();

    let mut event_loop = EventLoop::new();
    'main: loop {
        for event in event_loop.next(&display) {
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                // Break from the loop upon `Escape`.
                glium::glutin::Event::KeyboardInput(
                    _,
                    _,
                    Some(glium::glutin::VirtualKeyCode::Escape),
                ) |
                glium::glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        set_widgets(ui.set_widgets(), ids, &mut text, &mut answer);

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, text: &mut String, answer: &mut String) {
    widget::Canvas::new()
        .pad(0.0)
        .color(conrod::color::rgb(0.2, 0.35, 0.45))
        .set(ids.canvas, ui);

    let canvas_wh = ui.wh_of(ids.canvas).unwrap();

    widget::Text::new("Increment calc")
        .top_left_with_margin_on(ids.canvas, 5.0)
        .font_size(20)
        .color(color::WHITE)
        .set(ids.title, ui);

    for event in widget::TextBox::new(text)
        .font_size(15)
        .w_h((canvas_wh[0] - 90.) / 2., 30.0)
        .border(2.0)
        .border_color(color::BLUE)
        .color(color::WHITE)
        .set(ids.text_box, ui)
    {
        match event {
            widget::text_box::Event::Enter => println!("TextBox {:?}", text),
            widget::text_box::Event::Update(string) => *text = string,
        }
    }

    if widget::Button::new()
        .w_h((canvas_wh[0] - 90.) / 2., 30.0)
        .rgb(0.4, 0.75, 0.6)
        .border(2.0)
        .label("run")
        .set(ids.button, ui)
        .was_clicked()
    {
        if let Ok(num) = text.parse::<u64>() {
            *answer = increment(num).to_string();
        } else {
            println!("invalid number");
        }
    }

    widget::Text::new(answer)
        .font_size(20)
        .color(color::WHITE)
        .set(ids.result, ui);
}

struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    pub fn next(&mut self, display: &glium::Display) -> Vec<glium::glutin::Event> {
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        let mut events = Vec::new();
        events.extend(display.poll_events());

        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next());
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}
