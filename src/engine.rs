use find_folder;
use super::conrod;
use app::{Ids};
use support::{self, SupportIdType};
use support::threaded;
use std::rc::Rc;
use std;
use std::time::{Duration, Instant};
use std::thread::sleep;
use custom_widget::chatview;

pub struct Engine {}
impl Engine {
    pub fn new() -> Self {
        Engine {}
    }

    pub fn run_loop(&self) {
        use image;
        use conrod::{self, widget, Colorable, Positionable, Sizeable, Widget, color};
        use conrod::backend::glium::glium;
        use conrod::backend::glium::glium::{DisplayBuild, Surface};
        
        const WIDTH: u32 = 800;
        const HEIGHT: u32 = 600;

        // Build the window.
        let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Control Panel")
        .with_multisampling(4)
        .build_glium()
        .unwrap();

        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // The `WidgetId` for our background and `Image` widgets.
        let ids = Ids::new(ui.widget_id_generator());

        // Create our `conrod::image::Map` which describes each of our widget->image mappings.
        // In our case we only have one image, however the macro may be used to list multiple.
        let rust_logo = load_rust_logo(&display);
        let (w, h) = (rust_logo.get_width(), rust_logo.get_height().unwrap());
        let mut image_map = conrod::image::Map::new();
        let rust_logo = image_map.insert(rust_logo);
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();
        // Poll events from the window.
         let (event_tx, event_rx) = std::sync::mpsc::channel();
        let mut event_loop = support::threaded::EventLoop::new();
            let mut demo_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
            Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
            finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
            fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
            Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
            Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
            magna est, efficitur suscipit dolor eu, consectetur consectetur urna.".to_owned();
        'main: loop {

               // Handle all events.
            for event in event_loop.next(&display,event_rx) {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
         /*       if let threaded::Message::Event(even) = event{
                    if let Some(even) = conrod::backend::winit::convert(even.clone(), &display) {
                        ui.handle_event(even);
                        event_loop.needs_update();
                    }
                }
                */
                match event {
                    // Break from the loop upon `Escape`.
                    threaded::Message::Event(glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape))) |
                   threaded::Message::Event(glium::glutin::Event::Closed) =>
                    break 'main,
                    threaded::Message::Websocket(k)=> ui.needs_redraw(),
                    _ => {}
                }
            }

            // Instantiate the widgets.
            {
                let ui = &mut ui.set_widgets();
                // Draw a light blue background.
                widget::Canvas::new().color(color::LIGHT_BLUE).set(ids.master, ui);
                // Instantiate the `Image` at its full size in the middle of the window.
                let j = vec![chatview::Message{image_id:rust_logo,name:"alan",text:"jj",height:40.0},
                chatview::Message{image_id:rust_logo,name:"alan",text:"jj",height:80.0}];
               chatview::ChatView::new(j,&mut demo_text).middle_of(ids.master).set(ids.chatview,ui);
            }

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
        // Load the Rust logo from our assets folder to use as an example image.
        fn load_rust_logo(display: &glium::Display) -> glium::texture::Texture2d {
            let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
            let path = assets.join("images/rust.png");
            let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
            let image_dimensions = rgba_image.dimensions();
            let raw_image =
                glium::texture::RawImage2d::from_raw_rgba_reversed(rgba_image.into_raw(),
                                                                   image_dimensions);
            let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
            texture
        }
    }
}
