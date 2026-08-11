//! GUI bottom footer

use iced::widget::Space;
use iced::widget::tooltip::Position;
use iced::widget::{Column, Container, Row, Text, Tooltip, rich_text, span};
use iced::{Alignment, Length};

use crate::gui::components::button::row_open_link_tooltip;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, TOOLTIP_DELAY};
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::utils::formatted_strings::APP_VERSION;
use crate::utils::types::web_page::WebPage;
use crate::SNIFFNET_TITLECASE;

pub fn footer<'a>(
    thumbnail: bool,
    color_gradient: GradientType,
    dots_pulse: &(String, u8),
) -> Container<'a, Message, StyleType> {
    if thumbnail {
        return thumbnail_footer();
    }

    let heart_size = match dots_pulse.1 {
        1 => 17.0,
        2 => 20.0,
        _ => 14.0,
    };

    let footer_row = Row::new()
        .spacing(10)
        .padding([0, 20])
        .align_y(Alignment::Center)
        .push(release_details_row())
        .push(
            Column::new()
                .width(Length::Fill)
                .align_x(Alignment::End)
                .push(
                    Row::new()
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                        .push(Text::new("Made with").size(FONT_SIZE_FOOTER))
                        .push(
                            Text::new("❤")
                                .size(heart_size)
                                .width(25)
                                .align_x(Alignment::Center)
                                .align_y(Alignment::Center),
                        )
                        .push(Text::new("by ").size(FONT_SIZE_FOOTER))
                        .push(
                            Tooltip::new(
                                rich_text![span("nicolasjacques.com").underline(true).link(())]
                                    .on_link_click(|()| Message::OpenWebPage(WebPage::MySite))
                                    .size(FONT_SIZE_FOOTER),
                                row_open_link_tooltip(""),
                                Position::FollowCursor,
                            )
                            .class(ContainerType::Tooltip)
                            .delay(TOOLTIP_DELAY),
                        ),
                ),
        );

    Container::new(footer_row)
        .height(45)
        .align_y(Alignment::Center)
        .class(ContainerType::Gradient(color_gradient))
}

fn release_details_row<'a>() -> Row<'a, Message, StyleType> {
    Row::new()
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .width(Length::Fill)
        .push(Text::new(format!("{SNIFFNET_TITLECASE} {APP_VERSION}")).size(FONT_SIZE_FOOTER))
}

fn thumbnail_footer<'a>() -> Container<'a, Message, StyleType> {
    Container::new(Space::new().width(Length::Fill)).height(0)
}
