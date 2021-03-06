use conrod::{self, widget, Colorable, Positionable, Sizeable, Widget, color};
use app::Ids;
use custom_widget::chatview_ownedmsg as chatview;
use dyapplication::{Application, Static_Style};
use std::collections::VecDeque;
use std::sync::mpsc;
use greed_websocket::backend::websocket;
use greed_websocket::app as g_w_app;
use run_conrod_borrow as run_conrod;
use run_conrod_borrow::Conrod_Message;
pub fn set_ui(ref mut ui: conrod::UiCell,
              ids: &Ids,
              demo_text: &mut String,
              history: &mut VecDeque<chatview::Message>,
              styles: Static_Style,
              image_id:conrod::image::Id,
              name:&mut String,
              action_tx:mpsc::Sender<chatview::Message>) {
    widget::Canvas::new().color(color::LIGHT_BLUE).set(ids.master, ui);
    // Instantiate the `Image` at its full size in the middle of the window.
    chatview::ChatView::new(history, demo_text, styles,image_id,name,action_tx).middle_of(ids.master).set(ids.chatview, ui);
}
pub fn update_state(rust_logo: conrod::image::Id,
                    conrod_msg: Conrod_Message,
                    demo_text: &mut String,
                    history: &mut VecDeque<chatview::Message>) {
    if let Conrod_Message::Websocket(j) = conrod_msg.clone() {
        if let websocket::OwnedMessage::Text(z) = websocket::OwnedMessage::from(j) {
            if let Ok(g_w_app::ReceivedMsg { type_name,
                                             tables,
                                             players,
                                             request,
                                             reason,
                                             optional,
                                             location,
                                             sender,
                                             message }) = g_w_app::deserialize_receive(&z) {
                if let Some(_players) = players {}
                if let Some(_request) = request {}
                if type_name == "lobby" {}
                if type_name == "chat" {
                    if let (Some(Some(_location)), Some(Some(_sender)), Some(Some(_message))) =
                        (location, sender, message) {
                            println!("sender {:?}",_sender);
                        if _location == "lobby" {
                            history.push_back(chatview::Message {
                                                  image_id: rust_logo,
                                                  name: _sender,
                                                  text: _message,
                                              });
                        }
                        if _location == "table" {}

                    }

                }
            }
        }

    }
}
