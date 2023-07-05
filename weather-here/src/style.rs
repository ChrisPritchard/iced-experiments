use iced::{widget::text_input, Color, Theme};

#[derive(Clone)]
pub struct TextBoxValid {
    pub valid: bool,
    pub green: Color,
    pub red: Color,
}

impl text_input::StyleSheet for TextBoxValid {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: if self.valid { self.green } else { self.red },
            icon_color: palette.background.weak.text,
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: if self.valid { self.green } else { self.red },
            icon_color: palette.background.weak.text,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: if self.valid { self.green } else { self.red },
            icon_color: palette.background.weak.text,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        let palette = style.extended_palette();

        palette.background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        let palette = style.extended_palette();

        palette.background.base.text
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        let palette = style.extended_palette();

        palette.primary.weak.color
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.weak.color.into(),
            border_radius: 2.0.into(),
            border_width: 1.0,
            border_color: palette.background.strong.color,
            icon_color: palette.background.strong.color,
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }
}