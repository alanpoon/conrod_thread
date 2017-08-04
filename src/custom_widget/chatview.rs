use conrod::{self, widget, Colorable, Labelable, Positionable, Widget, image, Sizeable, Rect, color};
use self::widget::id::Generator;
use itertools::multizip;
/// The type upon which we'll implement the `Widget` trait.
const MARGIN: conrod::Scalar = 30.0;
#[derive(WidgetCommon)]
pub struct ChatView<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    lists: Vec<Message<'a>>,
    text_edit: &'a mut String,
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    enabled: bool,
}
pub struct Message<'a> {
    pub image_id: image::Id,
    pub name: &'a str,
    pub text: &'a str,
    pub height: f64,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod::Color>,
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod::text::font::Id>>,
}

widget_ids! {
    pub struct Ids {
        chat_canvas,
        message_panel,
        display_pics[],
        names[],
        texts[],
        rects[],
        text_edit_body,
        text_edit_panel,
        text_edit_panel_scrollbar,
        text_edit,
        text_rect,
        text_edit_button_panel,
        text_edit_button,
    }
}

/// Represents the unique, cached state for our ChatView widget.
pub struct State {
    pub ids: Ids,
}
impl State {
    pub fn resize(&mut self, num_list: usize, gen: &mut Generator) {
        self.ids.display_pics.resize(num_list, gen)
    }
}
impl<'a> ChatView<'a> {
    /// Create a button context to be built upon.
    pub fn new(lists: Vec<Message<'a>>, te: &'a mut String) -> Self {
        ChatView {
            lists: lists,
            common: widget::CommonBuilder::default(),
            text_edit: te,
            style: Style::default(),
            enabled: true,
        }
    }

    /// Specify the font used for displaying the label.
    pub fn label_font_id(mut self, font_id: conrod::text::font::Id) -> Self {
        self.style.label_font_id = Some(Some(font_id));
        self
    }

    /// If true, will allow user inputs.  If false, will disallow user inputs.  Like
    /// other Conrod configs, this returns self for chainability. Allow dead code
    /// because we never call this in the example.
    #[allow(dead_code)]
    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for ChatView<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = Option<()>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Option<()> {
        let widget::UpdateArgs { id, mut state, rect, mut ui, style, .. } = args;
        let num_list = self.lists.len();
        if state.ids.display_pics.len() < num_list {
            state.update(|state| {
                             state.ids.display_pics.resize(num_list, &mut ui.widget_id_generator())
                         });
        }
        if state.ids.names.len() < num_list {
            state.update(|state| state.ids.names.resize(num_list, &mut ui.widget_id_generator()));
        }
        if state.ids.texts.len() < num_list {
            state.update(|state| state.ids.texts.resize(num_list, &mut ui.widget_id_generator()));
        }
        if state.ids.rects.len() < num_list {
            state.update(|state| state.ids.rects.resize(num_list, &mut ui.widget_id_generator()));
        }
        let color = style.color(&ui.theme);

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        widget::Canvas::new()
            .flow_down(&[(state.ids.message_panel,
                          widget::Canvas::new().color(color::GREEN).pad_bottom(20.0)),
                         (state.ids.text_edit_body,
                          widget::Canvas::new()
                              .length(200.0)
                              .flow_right(&[(state.ids.text_edit_panel,
                                             widget::Canvas::new()
                                                 .scroll_kids_vertically()
                                                 .color(color::DARK_CHARCOAL)
                                                 .length(600.0)),
                                            (state.ids.text_edit_button_panel,
                                             widget::Canvas::new()
                                                 .color(color::DARK_CHARCOAL))]))])
            .middle_of(id)
            .set(state.ids.chat_canvas, ui);

        let mut k = self.text_edit;
        for edit in widget::TextEdit::new(k)
            .color(color::GREY)
            .padded_w_of(state.ids.text_edit_panel, 20.0)
            .mid_top_of(state.ids.text_edit_panel)
            .center_justify()
            .line_spacing(2.5)
            .restrict_to_height(false) // Let the height grow infinitely and scroll.
            .set(state.ids.text_edit, ui) {
            *k = edit;
        }
        if widget::Button::new()
               .color(color::GREY)
               .w_h(120.0, 60.0)
               .label("Enter")
               .middle_of(state.ids.text_edit_button_panel)
               .set(state.ids.text_edit_button, ui)
               .was_clicked() {
            *k = "".to_owned();
        };
        widget::Scrollbar::y_axis(state.ids.text_edit_panel)
            .auto_hide(true)
            .set(state.ids.text_edit_panel_scrollbar, ui);
        for (i, a, &dp_i, &name_i, &text_i, &rect_i) in
            multizip((0..100,
                      self.lists.iter(),
                      state.ids.display_pics.iter(),
                      state.ids.names.iter(),
                      state.ids.texts.iter(),
                      state.ids.rects.iter())) {

            /*   widget::Rectangle::fill([w-30.0, a.height])
                .up(a.height)
                .align_middle_x_of(id)
              //  .graphics_for(id)
                .color(color)
                .set(rect_i, ui);
            widget::Image::new(a.image_id)
                .middle_of(rect_i)
                .w_h(0.8 * h, 0.8 * h)
                .parent(rect_i)
                .set(dp_i, ui);

            // Now we'll instantiate our label using the **Text** widget.

            let label_color = style.label_color(&ui.theme);
            let font_size = style.label_font_size(&ui.theme);
            let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
            widget::Text::new(a.name)
                .and_then(font_id, widget::Text::font_id)
                .mid_left_of(rect_i)
                .font_size(font_size)
                .color(label_color)
                .set(name_i, ui);

            widget::Text::new(a.text)
                .and_then(font_id, widget::Text::font_id)
                .mid_left_with_margin_on(rect_i, 50.0)
                .font_size(font_size)
                .color(label_color)
                .set(text_i, ui);
                */
        }

        Some(())
    }
}
