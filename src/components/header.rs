use super::ESCAPE_KEY;
use yew::prelude::*;
use yew_styles::{
    button::Button,
    layouts::container::{JustifyContent, Mode},
    modal::Modal,
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Size, Style},
    text::{Text, TextType},
};

pub struct Header {
    link: ComponentLink<Self>,
    show_modal: bool,
}
pub enum Msg {
    Repository,
    OpenModal,
    CloseModal,
    Keyboard(web_sys::KeyboardEvent),
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, show_modal: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Repository => Self::open_repository(),
            Msg::OpenModal => self.open_modal(),
            Msg::CloseModal => self.close_modal(),
            Msg::Keyboard(ke) => {
                if ke.key_code() == ESCAPE_KEY {
                    self.close_modal()
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Navbar
                navbar_palette=Palette::Info
                navbar_style=Style::Light
                fixed=Fixed::None
                branch=html!{
                    <img src="passgen.png"/>
                }
            >
                <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
                    <NavbarItem active=true>
                        <a href="" style="text-decoration:none">{ "Home" }</a>
                    </NavbarItem>
                    <NavbarItem onclick_signal=self.link.callback(|_| Msg::Repository)>
                        <span>{ "Repository" }</span>
                    </NavbarItem>
                    <NavbarItem onclick_signal=self.link.callback(|_| Msg::OpenModal)>
                        <span>{ "About" }</span>
                    </NavbarItem>
                </NavbarContainer>
                { self.view_modal() }
            </Navbar>
        }
    }
}

impl Header {
    pub fn open_repository() -> ShouldRender {
        let window = yew::utils::window();
        match window.open_with_url(env!("CARGO_PKG_REPOSITORY")) {
            Ok(_) => (),
            Err(_) => web_sys::console::log_1(&"cannot open window".into()),
        }
        false
    }

    pub fn open_modal(&mut self) -> ShouldRender {
        let page_body_style = yew::utils::document().body().expect("cannot get body").style();
        page_body_style.set_property("overflow", "hidden").expect("cannot set property");
        self.show_modal = true;
        true
    }

    pub fn close_modal(&mut self) -> ShouldRender {
        let page_body_style = yew::utils::document().body().expect("cannot get body").style();
        page_body_style.set_property("overflow", "auto").expect("cannot set property");
        self.show_modal = false;
        true
    }

    pub fn view_modal(&self) -> Html {
        html! {
            html! {
                <Modal
                    modal_size=Size::Big
                    header=html!{
                        <b>{"About this application"}</b>
                    }
                    header_palette=Palette::Link
                    body=self.view_modal_body()
                    body_style=Style::Outline
                    body_palette=Palette::Link
                    is_open=self.show_modal
                    onclick_signal=self.link.callback(|_| Msg::CloseModal)
                    onkeydown_signal=self.link.callback(Msg::Keyboard)
                />
            }
        }
    }

    pub fn view_modal_body(&self) -> Html {
        html! {
            <div>
                <h3>{ "What" }</h3>
                <Text
                    text_type=TextType::Paragraph
                    text_size=Size::Medium
                    html_text=Some(html!{
                        <div>
                            { "This page can generate a random and maybe secure password. " }
                            { "But if you use this password, you may use clipboard, so be careful about security." }
                        </div>
                    })
                />
                <h3>{ "Warning" }</h3>
                    <Text
                        text_type=TextType::Paragraph
                        text_size=Size::Medium
                        html_text=Some(html!{
                            <div>
                                { "This page was created for practicing Rust. " }
                                { "As such, we cannot guarantee its safety." }
                            </div>
                        })
                    />
            <hr/>
                <Button
                    button_palette=Palette::Info
                    onclick_signal=self.link.callback(|_| Msg::CloseModal)
                >{ "Close" }</Button>
            </div>
        }
    }
}
