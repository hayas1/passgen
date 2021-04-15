use super::ESCAPE_KEY;
use yew::prelude::*;
use yew_styles::{
    button::Button,
    layouts::{
        container::{Container, Direction, JustifyContent, Mode, Wrap},
        item::{AlignSelf, Item, ItemLayout},
    },
    modal::Modal,
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Size, Style},
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
            <header>
                <Navbar
                    navbar_palette=Palette::Info
                    navbar_style=Style::Light
                    fixed=Fixed::None
                    branch=html!{ "logo" }>
                    <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
                        <NavbarItem active=true>
                            <span>{ "Home" }</span>
                        </NavbarItem>
                        <NavbarItem onclick_signal=self.link.callback(|_| Msg::Repository)>
                            <span>{ "Repository" }</span>
                        </NavbarItem>
                        <NavbarItem onclick_signal=self.link.callback(|_| Msg::OpenModal)>
                            <span>{ "About" }</span>
                        </NavbarItem>
                    </NavbarContainer>
                </Navbar>
                { self.view_modal() }
            </header>
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
                        <b>{"About this page"}</b>
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
        let layouts =
            vec![ItemLayout::ItXl(9), ItemLayout::ItL(9), ItemLayout::ItM(6), ItemLayout::ItXs(12)];
        html! {
            <div>
                <h1>{ "passgen" }</h1>
                <p>{ "simple password generator implemented by rust and compiled into wasm." }</p>
                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
                    <Item layouts=layouts.clone() align_self=AlignSelf::FlexStart>
                        <h2>{ "Available" }</h2>
                        <Container direction=Direction::Column wrap=Wrap::Wrap class_name="align-item">
                            <Item layouts=layouts.clone() align_self=AlignSelf::FlexStart>
                                <input id="checkbox-length" type="checkbox" disabled=true checked=true />
                                <label for="checkbox-length">{ "password length from 8 to 128" }</label>
                            </Item>
                            <Item layouts=layouts.clone() align_self=AlignSelf::FlexStart>
                                <input id="checkbox-lower" type="checkbox" disabled=true checked=true />
                                <label for="checkbox-lower">{ "use lower case" }</label>
                            </Item>
                            <Item layouts=layouts.clone() align_self=AlignSelf::FlexStart>
                                <input id="checkbox-upper" type="checkbox" disabled=true checked=true />
                                <label for="checkbox-upper">{ "use upper case" }</label>
                            </Item>
                            <Item layouts=layouts.clone() align_self=AlignSelf::FlexStart>
                                <input id="checkbox-numeric" type="checkbox" disabled=true checked=true />
                                <label for="checkbox-numeric">{ "use numeric" }</label>
                            </Item>
                            <Item layouts=layouts.clone() align_self=AlignSelf::FlexStart>
                                <input id="checkbox-mark-symbol" type="checkbox" disabled=true checked=true />
                                <label for="checkbox-mark-symbol">{ "use some mark symbols" }</label>
                            </Item>
                            <Item layouts=layouts.clone() align_self=AlignSelf::FlexStart>
                                <input id="checkbox-custom-characters" type="checkbox" disabled=true />
                                <label for="checkbox-custom-characters">{ "use custom characters" }</label>
                            </Item>
                        </Container>
                    </Item>
                    <Item layouts=layouts.clone() align_self=AlignSelf::FlexEnd>
                        <h2>{ "Feature" }</h2>
                        <p>{ "Since this is implemented by Rust, it can be compiled into WebAssembly and served as a web application." }</p>
                        <p>{ "Yew is used for the framework. And Yew Styles is used for style." }</p>
                    </Item>
                </Container>
                <hr/>
                <Button
                    button_palette=Palette::Info
                    onclick_signal=self.link.callback(|_| Msg::CloseModal)
                >{ "Close" }</Button>
            </div>
        }
    }
}
