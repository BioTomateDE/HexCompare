use iced::{Background, Border, Color, Element, Length};
use iced::border::Radius;
use iced::widget::{button, container, Column, Container, Space};
use crate::Msg;
use crate::scene::{MainScene, FONT_SIZE};

impl MainScene {
    fn render_scrollbar(&self) -> Element<Msg> {
        let thumb_height: f32 = (self.window_height / self.max_scroll_offset * FONT_SIZE).max(20.0);
        let scroll_ratio: f32 = self.scroll_offset / self.max_scroll_offset;
        let thumb_offset: f32 = scroll_ratio * (self.window_height - thumb_height);

        let thumb = Container::new("")
            .width(Length::Fill)
            .height(Length::Fixed(thumb_height))
            .style(scrollbar_thumb_style)
            .padding(0);

        let track_column = Column::new()
            .push(Space::with_height(Length::Fixed(thumb_offset)))
            .push(thumb)
            .spacing(0);

        // Wrap the column in a styled container:
        let scrollbar = button(track_column)
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(self.window_height))
            .style(iced::theme::Button::Custom(Box::new(ScrollbarTrackButton)))
            .on_press(Msg::StartScrollbarDrag); // If you want it to be clickable

        scrollbar.into()
    }
}



pub struct ScrollbarTrackButton;
impl button::StyleSheet for ScrollbarTrackButton {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 0.3))),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(4.0),
            },
            text_color: Color::TRANSPARENT, // No label on the scrollbar
            shadow: Default::default(),
            shadow_offset: Default::default(),
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = self.active(style);
        appearance.background = Some(Background::Color(Color::from_rgba(0.3, 0.3, 0.3, 0.4)));
        appearance
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = self.hovered(style);
        appearance.background = Some(Background::Color(Color::from_rgba(0.4, 0.4, 0.4, 0.6)));
        appearance
    }
    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 0.1))),
            ..self.active(&_style)
        }
    }
}

fn scrollbar_thumb_style(_theme: &iced::Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgba(0.4, 0.4, 0.4, 0.8))), // dark gray semi-transparent
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(4.0),
        },
        text_color: None,
        shadow: Default::default(),
    }
}

