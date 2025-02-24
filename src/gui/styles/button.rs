//! Buttons style

use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Color, Vector};

use crate::get_colors;
use crate::gui::styles::style_constants::{BORDER_BUTTON_RADIUS, BORDER_WIDTH, STARRED};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::palette::mix_colors;
use crate::gui::styles::types::style_tuple::StyleTuple;

impl From<StyleTuple> for iced::theme::Button {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Button::Custom(Box::new(tuple))
    }
}

impl button::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            background: Some(Background::Color(match self {
                StyleTuple(_, ElementType::TabActive | ElementType::NotStarred) => colors.primary,
                StyleTuple(_, ElementType::Starred) => STARRED,
                StyleTuple(_, ElementType::Badge) => colors.secondary,
                StyleTuple(_, ElementType::BorderedRound) => colors.round_containers,
                StyleTuple(_, ElementType::BorderedRoundSelected) => {
                    mix_colors(colors.primary, colors.buttons)
                }
                _ => colors.buttons,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
                StyleTuple(_, ElementType::BorderedRound | ElementType::BorderedRoundSelected) => {
                    12.0
                }
                _ => BORDER_BUTTON_RADIUS,
            },
            border_width: match self {
                StyleTuple(
                    _,
                    ElementType::TabActive
                    | ElementType::TabInactive
                    | ElementType::Starred
                    | ElementType::NotStarred
                    | ElementType::Badge,
                ) => 0.0,
                StyleTuple(_, ElementType::BorderedRound) => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: match self {
                StyleTuple(_, ElementType::Starred) => Color::BLACK,
                StyleTuple(_, ElementType::Badge) => colors.text_headers,
                _ => colors.text_body,
            },
            border_color: match self {
                StyleTuple(_, ElementType::Alert) => Color::new(1.0, 0.0, 0.0, 1.0),
                StyleTuple(_, ElementType::BorderedRound) => colors.round_borders,
                _ => colors.secondary,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            shadow_offset: Vector::new(0.0, 2.0),
            background: Some(Background::Color(match self {
                StyleTuple(_, ElementType::Starred) => STARRED,
                StyleTuple(_, ElementType::TabActive) => colors.primary,
                _ => mix_colors(colors.primary, colors.buttons),
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
                StyleTuple(_, ElementType::BorderedRound | ElementType::BorderedRoundSelected) => {
                    12.0
                }
                _ => BORDER_BUTTON_RADIUS,
            },
            border_width: match self {
                StyleTuple(
                    _,
                    ElementType::Starred
                    | ElementType::NotStarred
                    | ElementType::TabActive
                    | ElementType::TabInactive
                    | ElementType::BorderedRound,
                ) => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: match self {
                StyleTuple(_, ElementType::Alert) => Color::new(1.0, 0.0, 0.0, 1.0),
                StyleTuple(_, ElementType::BorderedRound) => colors.round_borders,
                _ => colors.secondary,
            },
            text_color: match self {
                StyleTuple(_, ElementType::Starred) => Color::BLACK,
                _ => colors.text_body,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        button::StyleSheet::active(self, style)
    }
}
