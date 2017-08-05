extern crate conrod_thread;
extern crate conrod;
extern crate find_folder;
extern crate image;
use conrod_thread::run_conrod;
use run_conrod::Conrod_Message;
use self::conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
const WIN_W: u32 = 600;
const WIN_H: u32 = 800;
use conrod_thread::backend::greed_websocket::futures;
use conrod_thread::backend::greed_websocket::futures::{Future, Sink};
use conrod_thread::backend::greed_websocket::websocket;
use conrod_thread::backend::greed_websocket::client1;
fn main() {
    // Build the window.
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIN_W, WIN_H)
        .with_title("Control Panel")
        .with_multisampling(4)
        .build_glium()
        .unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();


    // Create our `conrod::image::Map` which describes each of our widget->image mappings.
    // In our case we only have one image, however the macro may be used to list multiple.
    let rust_logo = load_rust_logo(&display);
    let (w, h) = (rust_logo.get_width(), rust_logo.get_height().unwrap());
    let mut image_map = conrod::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);

    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let (render_tx, render_rx) = std::sync::mpsc::channel();
    // This window proxy will allow conrod to wake up the `winit::Window` for rendering.
    let window_proxy = display.get_window().unwrap().create_window_proxy();
    std::thread::spawn(move || run_conrod::run(rust_logo, event_rx, render_tx, window_proxy));
    let mut c = 0;
    let mut last_update = std::time::Instant::now();
    let event_tx_clone = event_tx.clone();
    let (futures_tx,futures_rx) = futures::sync::mpsc::channel(1);
    std::thread::spawn(move || {
        let mut o = "".to_owned();
        'update: loop {
             futures_tx.clone().send(websocket::OwnedMessage::Text("{chat:'hello',location:'lobby'}".to_owned())).wait().unwrap();
            println!("o {:?}", o);
            let five_sec = std::time::Duration::from_secs(5);
            std::thread::sleep(five_sec);
            event_tx_clone.send(Conrod_Message::Websocket(websocket::OwnedMessage::Text(o.clone())))
                .unwrap();
            let z = "world ";
            o.push_str(z);
        }
    });
    let (proxy_tx,proxy_rx) =  std::sync::mpsc::channel();
     let event_tx_clone_2 = event_tx.clone();

    std::thread::spawn(move||{
        'proxy: loop{
             while let Ok(s) = proxy_rx.try_recv() {
                event_tx_clone_2.send(Conrod_Message::Websocket(s)).unwrap();
             }
        }
    });
    std::thread::spawn(move||{
        client1::run(proxy_tx,futures_rx);
    });
    'main: loop { 

        // We don't want to loop any faster than 60 FPS, so wait until it has been at least
        // 16ms since the last yield.
        let sixteen_ms = std::time::Duration::from_millis(16);
        let now = std::time::Instant::now();
        let duration_since_last_update = now.duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        let mut events: Vec<_> = display.poll_events().collect();

        // If there are no events, wait for the next event.
        if events.is_empty() {
            events.extend(display.wait_events().next());
        }

        // Send any relevant events to the conrod thread.
        for event in events {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                event_tx.send(Conrod_Message::Event(event)).unwrap();
            }

            match event {
                // Break from the loop upon `Escape`.
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::Closed =>
                        break 'main,
                _ => {}
            }
        }

        // Draw the most recently received `conrod::render::Primitives` sent from the `Ui`.
        if let Ok(mut primitives) = render_rx.try_recv() {
            while let Ok(newest) = render_rx.try_recv() {
                primitives = newest;
            }

            renderer.fill(&display, primitives.walk(), &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }

        last_update = std::time::Instant::now();
    }



}

fn load_rust_logo(display: &glium::Display) -> glium::texture::Texture2d {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let path = assets.join("images/rust.png");
    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    let image_dimensions = rgba_image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(rgba_image.into_raw(),
                                                                       image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}
