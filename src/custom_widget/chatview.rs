use conrod::{self, widget, Colorable, Labelable, Positionable, Widget, image, Sizeable, Rect};
use itertools::Zip;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ChatView {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    lists: Vec<Indiv>,
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    enabled: bool,
}
pub struct Indiv {
    pub image_id: image::Id,
    pub name: String,
    pub text: String,
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
    struct Ids {
        display_pics[],
        names[],
        texts[],
        rects[],
    }
}

/// Represents the unique, cached state for our ChatView widget.
pub struct State {
    ids: Ids,
}

impl ChatView {
    /// Create a button context to be built upon.
    pub fn new(lists: Vec<Indiv>) -> Self {
        ChatView {
            lists: lists,
            common: widget::CommonBuilder::default(),
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
impl Widget for ChatView {
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
    fn update(self, args: widget::UpdateArgs<Self>) ->Option<()>{
        let widget::UpdateArgs { id, state, rect, mut ui, style, .. } = args;
        let num_list = self.lists.len();
        state.ids.display_pics.resize(num_list, &mut ui.widget_id_generator());
        state.ids.names.resize(num_list, &mut ui.widget_id_generator());
        state.ids.texts.resize(num_list, &mut ui.widget_id_generator());
        state.ids.rects.resize(num_list, &mut ui.widget_id_generator());
        let color = style.color(&ui.theme);

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //

        let (x, y, w, h) = rect.x_y_w_h();
        for ((image_id, name, text, height), dp_i, name_i, text_i, rect_i) in
            Zip::new((&self.lists,
                      &state.ids.display_pics,
                      &state.ids.names,
                      &state.ids.texts,
                      &state.ids.rects)) {
            widget::Rectangle::fill([w, h])
                .up(height)
                .middle_of(id)
                .graphics_for(id)
                .color(color)
                .set(rect_i, ui);
            widget::Image::new(image_id)
                .middle_of(rect_i)
                .w_h(0.8 * h, 0.8 * h)
                .parent(rect_i)
                .set(dp_i, ui);

            // Now we'll instantiate our label using the **Text** widget.

            let label_color = style.label_color(&ui.theme);
            let font_size = style.label_font_size(&ui.theme);
            let font_id = style.label_font_id(&ui.theme).or(ui.fonts.ids().next());
            widget::Text::new(&name)
                .and_then(font_id, widget::Text::font_id)
                .mid_left_of(rect_i)
                .font_size(font_size)
                .color(label_color)
                .set(name_i, ui);

            widget::Text::new(&text)
                .and_then(font_id, widget::Text::font_id)
                .mid_left_with_margin_on(rect_i, 50.0)
                .font_size(font_size)
                .color(label_color)
                .set(text_i, ui);
        }

        Some(())
    }
}
