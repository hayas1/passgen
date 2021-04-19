use crate::password::{
    generator::PasswordGenerator, password::Password, PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH,
};
use yew::prelude::*;
use yew_styles::{
    button::Button,
    card::Card,
    layouts::{
        container::{Container, Direction, Wrap},
        item::{AlignSelf, Item, ItemLayout},
    },
    styles::{Palette, Size, Style},
};

pub struct GeneratorPane {
    link: ComponentLink<Self>,
    generator: PasswordGenerator,
    password: Password,
}

pub enum Msg {
    Generate,
    EditLength(usize),
    ToggleLower,
    ToggleUpper,
    ToggleNumeric,
    ToggleMark(char),
    CopyPassword,
}

impl Component for GeneratorPane {
    type Message = Msg;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let generator = PasswordGenerator::default();
        let password =
            generator.generate_password().expect("default generator should generate password.");
        Self { link, generator, password }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Generate => (),
            Msg::EditLength(len) => self.generator.len = len,
            Msg::ToggleLower => self.generator.use_lower = !self.generator.use_lower,
            Msg::ToggleUpper => self.generator.use_upper = !self.generator.use_upper,
            Msg::ToggleNumeric => self.generator.use_numeric = !self.generator.use_numeric,
            Msg::ToggleMark(mark) => {
                self.generator.mark.toggle(mark);
            }
            Msg::CopyPassword => {
                self.copy_password_to_clipboard();
                return false; // do not refresh password
            }
        }
        self.refresh_password()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            { self.view_main_card_body() }
        }
    }
}

impl GeneratorPane {
    pub fn refresh_password(&mut self) -> ShouldRender {
        match self.generator.generate_password() {
            Ok(password) => self.password = password,
            Err(error) => web_sys::console::log_2(
                &error.to_string().into(),
                &format!("invalid setting: {:?}", self.generator).into(),
            ),
        }
        true
    }

    pub fn copy_password_to_clipboard(&self) {
        // TODO: error handling
        let password = self.password.to_string(); // this may have subtle security warning
        let task = async move {
            let promise = yew::utils::window().navigator().clipboard().write_text(&password);
            let _result = wasm_bindgen_futures::JsFuture::from(promise).await;
        };
        wasm_bindgen_futures::spawn_local(task);
    }

    pub fn view_main_card_body(&self) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Item class_name="generated-password" layouts=vec![ItemLayout::ItXs(12)] align_self=AlignSelf::Center>
                    { self.view_generated_password() }
                </Item>
                <Item layouts=vec![ItemLayout::ItL(12)] align_self=AlignSelf::FlexStart>
                    <Container direction=Direction::Row wrap=Wrap::Nowrap>
                        <Item layouts=vec![ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12)] align_self=AlignSelf::FlexStart>
                            { self.view_length_bar() }
                        </Item>
                        <Item layouts=vec![ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12)] align_self=AlignSelf::FlexEnd>
                            { self.view_generate_button() }
                        </Item>
                    </Container>
                    <Container direction=Direction::Row wrap=Wrap::Nowrap>
                        <Item layouts=vec![ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12)] align_self=AlignSelf::FlexStart>
                            { self.view_lower_checkbox() }
                        </Item>
                        <Item layouts=vec![ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12)] align_self=AlignSelf::Center>
                            { self.view_upper_checkbox() }
                        </Item>
                        <Item layouts=vec![ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12)] align_self=AlignSelf::FlexEnd>
                            { self.view_numeric_checkbox() }
                        </Item>
                    </Container>
                </Item>
                <Item layouts=vec![ItemLayout::ItL(12)] align_self=AlignSelf::FlexStart>
                    { self.view_mark_checkboxes() }
                </Item>
            </Container>
        }
    }

    pub fn view_generated_password(&self) -> Html {
        html! {
            <div title="Click to copy password">
                <Card
                    card_size=Size::Medium
                    card_palette=Palette::Link
                    card_style=Style::Light
                    onclick_signal=self.link.callback(|_| Msg::CopyPassword)
                    header=Some(html!{
                        <b>{ "Generated Password" }</b>
                    })
                    body=Some(html!{
                        <p id="password-display">{self.generator.to_html(&self.password)}</p>
                    })
                />
            </div>
        }
    }

    pub fn view_generate_button(&self) -> Html {
        let palette = match self.generator.can_generate() {
            Ok(()) => Palette::Success,
            Err(_) => Palette::Warning,
        };
        html! {
            <Button
                id="generate-button"
                onclick_signal=self.link.callback(|_| Msg::Generate)
                button_palette=palette
                button_style=Style::Light
                button_size=Size::Small
            >{
                match self.generator.can_generate() {
                    Ok(()) => "Generate!",
                    Err(_) => "Unavailable!",
                }
            }</Button>
        }
    }

    pub fn view_length_bar(&self) -> Html {
        let oninput = self.link.callback(|d: InputData| {
            Msg::EditLength(d.value.parse().expect("range type input should have only integer."))
        });
        html! {
            <div>
                <input type="range" min=PASSWORD_MIN_LENGTH max=PASSWORD_MAX_LENGTH oninput=oninput/>
                { self.generator.len }
            </div>
        }
    }

    pub fn view_lower_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleLower);
        html! {
            <div>
                <label for="lower-checkbox">{ "Lower Case" }</label>
                <input id="lower-checkbox" type="checkbox" checked={ self.generator.use_lower } onclick=onclick/>
            </div>
        }
    }

    pub fn view_upper_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleUpper);
        html! {
            <div>
                <label for="upper-checkbox">{ "Upper Case" }</label>
                <input id="upper-checkbox" type="checkbox" checked={ self.generator.use_upper } onclick=onclick/>
            </div>
        }
    }

    pub fn view_numeric_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleNumeric);
        html! {
            <div>
                <label for="numeric-checkbox">{ "Numeric" }</label>
                <input id="numeric-checkbox" type="checkbox" checked={ self.generator.use_numeric } onclick=onclick/>
            </div>
        }
    }

    pub fn view_mark_checkboxes(&self) -> Html {
        let checkboxes = self.generator.mark.get_marks().map(|(c, checked)| {
            let onchange = self.link.callback(move |_| Msg::ToggleMark(c));
            let id = format!("mark{}-checkbox", c as u32);
            html! {
                <li>
                    <input id=id type="checkbox" checked=checked onchange=onchange/>
                    <label for=id>{ c }</label>
                </li>
            }
        });
        html! {
            <div>
                <p>{ "Mark" }</p>
                <ul>
                    { checkboxes.collect::<Html>() }
                </ul>
            </div>
        }
    }
}
