use std;
use conrod;
use conrod::backend::glium::glium;
use greed_websocket::backend::websocket::OwnedMessage;

pub enum Message {
    Event(glium::glutin::Event),
    Websocket(OwnedMessage),
}
pub struct EventLoop {
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

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self,
                display: &glium::Display,
                event_rx: std::sync::mpsc::Receiver<OwnedMessage>)
                -> Vec<Message> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events.extend(display.poll_events().map(|z| Message::Event(z)));
        while let Ok(msg) = event_rx.try_recv() {
            events.push(Message::Websocket(msg));
        }
        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next().map(|z| Message::Event(z)));
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}

#[derive(Debug)]
pub enum SupportIdType {
    ImageId(conrod::image::Id),
    FontId(conrod::text::font::Id),
}
