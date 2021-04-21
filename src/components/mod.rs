pub mod app;
pub mod generator_pane;
pub mod header;

pub const ESCAPE_KEY: u32 = 27;

use crate::password::{Password, PasswordGenerator, LOWER_SET, NUMERIC_SET, UPPER_SET};
use yew::prelude::*;
use yew_styles::{
    styles::Size,
    text::{Text, TextType},
};
impl PasswordGenerator {
    pub fn to_html(&self, password: &Password) -> Html {
        let display: Html = password
            .iter()
            .map(|c| {
                let class = if !self.addition.is_empty() && self.addition.contains(c) {
                    "addition"
                } else if self.use_lower && LOWER_SET.contains(c) {
                    "lower"
                } else if self.use_upper && UPPER_SET.contains(c) {
                    "upper"
                } else if self.use_numeric && NUMERIC_SET.contains(c) {
                    "numeric"
                } else if !self.mark.is_empty() && self.mark.contains(c) {
                    "mark"
                } else {
                    "addition" // any other character display as addition symbol
                };
                html! {
                    <span class=class>{ c }</span>
                }
            })
            .collect();
        html! {
            <Text
                text_type=TextType::Plain
                text_size={
                    if self.len < 64 {
                        Size::Big
                    } else {
                        Size::Medium
                    }
                }
                html_text= html!{ display }
            />
        }
    }
}
