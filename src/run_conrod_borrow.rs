use std;
use conrod::{self, widget, Colorable, Positionable, Sizeable, Widget, color};
use custom_widget::chatview;
use conrod::backend::glium::glium;
use app::Ids;
use greed_websocket::backend::futures;
use greed_websocket::backend::websocket;
use greed_websocket::backend::futures::{Future, Sink};
use find_folder;
const WIN_W: u32 = 800;
const WIN_H: u32 = 600;
pub fn run(rust_logo: conrod::image::Id,
           event_rx: std::sync::mpsc::Receiver<Conrod_Message>,
           render_tx: std::sync::mpsc::Sender<conrod::render::OwnedPrimitives>,
           window_proxy: glium::glutin::WindowProxy) {
    let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).build();

    // The `WidgetId` for our background and `Image` widgets.
    let ids = Ids::new(ui.widget_id_generator());
    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
    let mut demo_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
            Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
            finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
            fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
            Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
            Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
            magna est, efficitur suscipit dolor eu, consectetur consectetur urna.".to_owned();

    let mut needs_update = true;
    let mut last_update = std::time::Instant::now();
    'conrod: loop {
        let sixteen_ms = std::time::Duration::from_millis(16);
        let now = std::time::Instant::now();
        let duration_since_last_update = now.duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }
        // Collect any pending events.
        let mut events = Vec::new();
        while let Ok(event) = event_rx.try_recv() {
                    if let Conrod_Message::Websocket(j) = event.clone() {
                        if let websocket::OwnedMessage::Text(z) = websocket::OwnedMessage::from(j){
                            demo_text = z;
                        }
                    }
            events.push(event);
        }

        // If there are no events pending, wait for them.
        if events.is_empty() || !needs_update {
            match event_rx.recv() {
                Ok(event) => {
                    if let Conrod_Message::Websocket(j) = event.clone() {
                        if let websocket::OwnedMessage::Text(z) = websocket::OwnedMessage::from(j){
                            demo_text = z;
                        }
                    }
                    events.push(event);
                }
                Err(_) => break 'conrod,
            };
        }

        needs_update = false;

        // Input each event into the `Ui`.
        for event in events {
            if let Conrod_Message::Event(e) = event {
                ui.handle_event(e);
            }
            needs_update = true;
        }


        // Instantiate the widgets.
        set_ui(ui.set_widgets(), &ids, rust_logo, &mut demo_text);
        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            if render_tx.send(primitives.owned()).is_err() {
                break 'conrod;
            }
            // Wakeup `winit` for rendering.
            window_proxy.wakeup_event_loop();
        }

        println!("instantiate c {:?}", demo_text);
    }
    // Load the Rust logo from our assets folder to use as an example image.

}
fn set_ui(ref mut ui: conrod::UiCell,
          ids: &Ids,
          rust_logo: conrod::image::Id,
          demo_text: &mut String) {
    widget::Canvas::new().color(color::LIGHT_BLUE).set(ids.master, ui);
    // Instantiate the `Image` at its full size in the middle of the window.
    let j = vec![chatview::Message {
                     image_id: rust_logo,
                     name: "alan",
                     text: "jj",
                     height: 40.0,
                 },
                 chatview::Message {
                     image_id: rust_logo,
                     name: "alan",
                     text: "jj",
                     height: 80.0,
                 }];
    chatview::ChatView::new(j, demo_text).middle_of(ids.master).set(ids.chatview, ui);
}
#[derive(Clone,Debug)]
pub enum Conrod_Message<'a> {
    Event(conrod::event::Input),
    Websocket(websocket::Message<'a>),
}
