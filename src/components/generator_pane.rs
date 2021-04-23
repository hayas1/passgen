use super::{LOWER_CLASS, NUMERIC_CLASS, UPPER_CLASS};
use crate::password::{Password, PasswordGenerator, PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH};
use yew::prelude::*;
use yew_styles::{
    button::Button,
    card::Card,
    forms::{
        form_file::FormFile,
        form_group::{FormGroup, Orientation},
        form_input::{FormInput, InputType},
        form_label::FormLabel,
        form_select::FormSelect,
        form_textarea::FormTextArea,
    },
    layouts::{
        container::{Container, Direction, JustifyContent, Mode, Wrap},
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
        self.view_main()
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

    pub fn view_main(&self) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap justify_content=JustifyContent::Center(Mode::NoMode)>
                <Item class_name="stretch-card" layouts=vec![ItemLayout::ItXs(12)] align_self=AlignSelf::Center>
                    { self.view_generated_password() }
                </Item>
                <Container direction=Direction::Row wrap=Wrap::Nowrap>
                    <Item layouts=vec![ItemLayout::ItL(8)] align_self=AlignSelf::FlexStart>
                        { self.view_length_bar_button_pane() }
                    </Item>
                    <Item class_name="stretch-card"  layouts=vec![ItemLayout::ItL(4)] align_self=AlignSelf::FlexEnd>
                        { self.view_character_checkboxes_pane() }
                    </Item>
                </Container>
                <Item layouts=vec![ItemLayout::ItL(12)] align_self=AlignSelf::Stretch>
                    { self.view_mark_checkboxes() }
                </Item>
            </Container>
        }
    }

    pub fn view_generated_password(&self) -> Html {
        html! {
            <div title="Click to copy password!">
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

    pub fn view_length_bar_button_pane(&self) -> Html {
        html! {
            <Card
                card_size=Size::Small
                card_palette=Palette::Primary
                card_style=Style::Light
                interaction_effect=false
                header=Some(html!{
                    <p>{ "Setting" }</p>
                })
                body=Some(html!{
                    <Container direction=Direction::Row wrap=Wrap::Wrap>
                        <Item layouts=vec![ItemLayout::ItXs(12)] align_self=AlignSelf::Stretch>
                            { self.view_length_bar() }
                        </Item>
                        <Item layouts=vec![ItemLayout::ItXs(12)] align_self=AlignSelf::Stretch>
                            { self.view_generate_button() }
                        </Item>
                    </Container>
                })
            />
        }
    }

    pub fn view_length_bar(&self) -> Html {
        let oninput = self.link.callback(|d: InputData| {
            Msg::EditLength(d.value.parse().expect("range type input should have only integer."))
        });
        html! {
            <FormGroup orientation=Orientation::Horizontal>
                <FormInput
                    id="password-length"
                    input_type=InputType::Range
                    input_size=Size::Big
                    min=PASSWORD_MIN_LENGTH as u16
                    max=PASSWORD_MAX_LENGTH as u16
                    oninput_signal=oninput
                    // value=PASSWORD_DEFAULT_LENGTH  // Yew Styles do not allow `value` attribute
                />
                <FormLabel
                  text=self.generator.len.to_string()
                  label_for="password-length"
                />
            </FormGroup>
        }
    }

    pub fn view_generate_button(&self) -> Html {
        match self.generator.can_generate() {
            Ok(()) => html! {
                <Button
                    id="generate-button"
                    onclick_signal=self.link.callback(|_| Msg::Generate)
                    button_palette=Palette::Success
                    button_style=Style::Light
                    button_size=Size::Small
                >{ "Generate!" }</Button>
            },
            Err(_) => html! {
                <Button
                    id="generate-button"
                    onclick_signal=self.link.callback(|_| Msg::Generate)
                    button_palette=Palette::Warning
                    button_style=Style::Light
                    button_size=Size::Small
                >{ "Unavailable!" }</Button>
            },
        }
    }

    pub fn view_character_checkboxes_pane(&self) -> Html {
        html! {
            <Card
                card_size=Size::Small
                card_palette=Palette::Secondary
                card_style=Style::Light
                interaction_effect=false
                // header=Some(html!{
                //     <p>{ "Character" }</p>
                // })
                single_content=Some(html!{
                    <Container direction=Direction::Column wrap=Wrap::Wrap>
                        <Item layouts=vec![ItemLayout::ItM(6)] align_self=AlignSelf::Stretch>
                            { self.view_lower_checkbox() }
                        </Item>
                        <Item layouts=vec![ItemLayout::ItM(6)] align_self=AlignSelf::Stretch>
                            { self.view_upper_checkbox() }
                        </Item>
                        <Item layouts=vec![ItemLayout::ItM(6)] align_self=AlignSelf::Stretch>
                            { self.view_numeric_checkbox() }
                        </Item>
                    </Container>
                })
            />
        }
    }

    pub fn view_lower_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleLower);
        html! {
            html! {
                <FormGroup orientation=Orientation::Horizontal>
                    // <div class=LOWER_CLASS>{ "abc" }</div>
                    <FormLabel
                      text="Lower"
                      label_for="lower-checkbox"
                    />
                    <FormInput
                        id="lower-checkbox"
                        input_type=InputType::Checkbox
                        input_size=Size::Medium
                        oninput_signal=onclick
                        checked=self.generator.use_lower
                    />
                </FormGroup>
            }
        }
    }

    pub fn view_upper_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleUpper);
        html! {
            <FormGroup orientation=Orientation::Horizontal>
                // <div class=UPPER_CLASS>{ "ABC" }</div>
                <FormLabel
                  text="Upper"
                  label_for="upper-checkbox"
                />
                <FormInput
                    id="upper-checkbox"
                    input_type=InputType::Checkbox
                    input_size=Size::Medium
                    oninput_signal=onclick
                    checked=self.generator.use_upper
                />
            </FormGroup>
        }
    }

    pub fn view_numeric_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleNumeric);
        html! {
            <FormGroup orientation=Orientation::Horizontal>
                // <div class=NUMERIC_CLASS>{ "123" }</div>
                <FormLabel
                  text="Numeric"
                  label_for="numeric-checkbox"
                />
                <FormInput
                    id="numeric-checkbox"
                    input_type=InputType::Checkbox
                    input_size=Size::Medium
                    oninput_signal=onclick
                    checked=self.generator.use_numeric
                />
            </FormGroup>
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
