use web_sys;
use yew::prelude::*;
use yew_styles::{
    layouts::container::{JustifyContent, Mode},
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Style},
};
pub struct Header {
    link: ComponentLink<Self>,
}
pub enum Msg {
    Repository,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Repository => Self::open_repository(),
        }
        false
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
                        <NavbarItem  onclick_signal=self.link.callback(move |_| Msg::Repository)>
                            <span>{ "Repository" }</span>
                        </NavbarItem>
                        <NavbarItem>
                            <span>{ "About" }</span>
                        </NavbarItem>
                    </NavbarContainer>
                </Navbar>
            </header>
        }
    }
}

impl Header {
    pub fn open_repository() {
        let window = match web_sys::window() {
            Some(window) => window,
            None => {
                web_sys::console::log_1(&"cannot get window".into());
                return;
            }
        };
        match window.open_with_url(env!("CARGO_PKG_REPOSITORY")) {
            Ok(_) => (),
            Err(_) => {
                web_sys::console::log_1(&"cannot open window".into());
                return;
            }
        }
    }
}
