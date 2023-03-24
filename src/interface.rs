use inquire::{
    self,
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
};

pub fn init() {
    inquire::set_global_render_config(inquire_config());
}

fn inquire_config() -> RenderConfig {
    RenderConfig {
        prompt_prefix: Styled::new(" ❯").with_fg(Color::LightMagenta),
        answered_prompt_prefix: Styled::new(" ❯").with_fg(Color::LightCyan),
        highlighted_option_prefix: Styled::new("▶").with_fg(Color::LightYellow),
        scroll_up_prefix: Styled::new("▲"),
        scroll_down_prefix: Styled::new("▼"),

        answer: StyleSheet::new()
            .with_attr(Attributes::ITALIC)
            .with_fg(Color::LightCyan),

        ..Default::default()
    }
}
