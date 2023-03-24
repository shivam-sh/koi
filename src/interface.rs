use inquire::{
    self,
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
};

pub fn init() {
    inquire::set_global_render_config(inquire_config());
}

fn inquire_config() -> RenderConfig {
    let mut config = RenderConfig::default();

    config.prompt_prefix = Styled::new(" ❯").with_fg(Color::LightMagenta);
    config.answered_prompt_prefix = Styled::new(" ❯").with_fg(Color::LightCyan);
    config.highlighted_option_prefix = Styled::new("▶").with_fg(Color::LightYellow);
    config.scroll_up_prefix = Styled::new("▲");
    config.scroll_down_prefix = Styled::new("▼");

    config.text_input = StyleSheet::new();

    config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightCyan);

    config
}
