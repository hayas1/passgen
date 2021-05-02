use crate::password::{Password, PasswordGenerator, PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_styles::{
    button::Button,
    card::Card,
    forms::{
        form_group::{FormGroup, Orientation},
        form_input::{FormInput, InputType},
        form_label::FormLabel,
    },
    layouts::{
        container::{Container, Direction, JustifyContent, Mode, Wrap},
        item::{AlignSelf, Item, ItemLayout},
    },
    styles::{Palette, Size, Style},
    text::{Text, TextType},
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
    DraggedMark(DragEvent),
    DragOverMark(DragEvent),
    DroppedMark(DragEvent, bool),
    ToggleAllMark(bool),
    InputAddition(String),
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
            Msg::DraggedMark(drag_event) => {
                self.dragged_mark_tag(drag_event);
                return false; // do not refresh password
            }
            Msg::DragOverMark(drag_over_event) => {
                self.drag_over_mark_tag(drag_over_event);
                return false; // do no refresh password
            }
            Msg::DroppedMark(drop_event, enable) => self.dropped_mark_tag(drop_event, enable),
            Msg::ToggleAllMark(available) => {
                if available {
                    self.generator.mark.insert_all();
                } else {
                    self.generator.mark.clear()
                }
            }
            Msg::InputAddition(addition) => self.generator.addition = addition.chars().collect(),
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

    pub fn dragged_mark_tag(&mut self, drag_event: DragEvent) {
        let target: web_sys::HtmlElement = drag_event
            .target()
            .expect("cannot get drag target tag")
            .dyn_into()
            .expect("cannot cast");
        drag_event
            .data_transfer()
            .expect("cannot get data_transfer")
            .set_data("mark", &target.text_content().expect("not contain text"))
            .expect("cannot set mark");
    }

    pub fn drag_over_mark_tag(&mut self, drag_over_event: DragEvent) {
        drag_over_event.prevent_default();
    }

    pub fn dropped_mark_tag(&mut self, drop_event: DragEvent, enable: bool) {
        let mark = drop_event
            .data_transfer()
            .expect("cannot get data_transfer")
            .get_data("mark")
            .expect("cannot get mark");
        if enable {
            self.generator.mark.insert(mark.chars().last().expect("invalid more than two chars"));
        } else {
            self.generator.mark.remove(&mark.chars().last().expect("invalid more than two chars"));
        }
    }

    pub fn view_main(&self) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap justify_content=JustifyContent::Center(Mode::NoMode)>
                <Item class_name="fix-width" layouts=vec![ItemLayout::ItXs(12)]>
                    { self.view_generated_password() }
                </Item>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    <Item layouts=vec![ItemLayout::ItXs(7)]>
                        { self.view_setting_pane() }
                    </Item>
                    <Item layouts=vec![ItemLayout::ItXs(5)]>
                        { self.view_character_checkboxes_pane() }
                    </Item>
                </Container>
                <Item layouts=vec![ItemLayout::ItL(12)] align_self=AlignSelf::Stretch>
                    { self.view_mark_container() }
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

    pub fn view_generate_button(&self) -> Html {
        match self.generator.can_generate() {
            Ok(()) => html! {
                <Button
                    id="generate-button"
                    onclick_signal=self.link.callback(|_| Msg::Generate)
                    button_palette=Palette::Success
                    button_style=Style::Light
                    button_size=Size::Small
                >{ "Generate Password!" }</Button>
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

    pub fn view_setting_pane(&self) -> Html {
        html! {
            <Card
                card_size=Size::Small
                card_palette=Palette::Primary
                card_style=Style::Light
                interaction_effect=false
                single_content=Some(html!{
                    <Container direction=Direction::Column wrap=Wrap::Wrap>
                        <Item layouts=vec![ItemLayout::ItXs(12)] align_self=AlignSelf::FlexEnd>
                            { self.view_generate_button() }
                        </Item>
                        <Item layouts=vec![ItemLayout::ItXs(12)] align_self=AlignSelf::Stretch>
                            { self.view_length_bar() }
                        </Item>
                        <Item layouts=vec![ItemLayout::ItXs(12)] align_self=AlignSelf::Stretch>
                            { self.view_addition_form() }
                        </Item>
                    </Container>
                })
            />
        }
    }

    pub fn view_addition_form(&self) -> Html {
        html! {
            <FormGroup orientation=Orientation::Horizontal>
                <FormLabel
                    text="Addition"
                    label_for="addition-form"
                />
                <FormInput
                    id="addition-form"
                    input_type=InputType::Text
                    input_size=Size::Medium
                    oninput_signal=self.link.callback(|d: InputData| Msg::InputAddition(d.value))
                />
            </FormGroup>
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

    pub fn view_character_checkboxes_pane(&self) -> Html {
        html! {
            <Card
                card_size=Size::Small
                card_palette=Palette::Secondary
                card_style=Style::Light
                interaction_effect=false
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
            <FormGroup orientation=Orientation::Horizontal>
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

    pub fn view_upper_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleUpper);
        html! {
            <FormGroup orientation=Orientation::Horizontal>
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

    pub fn view_mark_container(&self) -> Html {
        html! {
            <Container wrap=Wrap::Wrap direction=Direction::Row>
                <Item layouts=vec![ItemLayout::ItM(6), ItemLayout::ItXs(12)]>
                    <Card
                        id="available-mark"
                        card_size=Size::Small
                        card_palette=Palette::Success
                        card_style=Style::Light
                        interaction_effect=false
                        ondrop_signal=self.link.callback(|e| Msg::DroppedMark(e, true))
                        ondragover_signal=self.link.callback(Msg::DragOverMark)
                        header=Some(html!{
                            <Button
                                id="all-available-button"
                                onclick_signal=self.link.callback(|_| Msg::ToggleAllMark(true))
                                button_palette=Palette::Success
                                button_style=Style::Light
                                button_size=Size::Medium
                            >{ "Available" }</Button>
                        })
                        body=Some(self.view_mark_tags(true))
                    />
                </Item>
                <Item layouts=vec![ItemLayout::ItM(6), ItemLayout::ItXs(12)]>
                    <Card
                        id="unavailable-mark"
                        card_size=Size::Small
                        card_palette=Palette::Standard
                        card_style=Style::Light
                        interaction_effect=false
                        ondrop_signal=self.link.callback(|e| Msg::DroppedMark(e, false))
                        ondragover_signal=self.link.callback(Msg::DragOverMark)
                        header=Some(html!{
                            <Button
                                id="all-unavailable-button"
                                onclick_signal=self.link.callback(|_| Msg::ToggleAllMark(false))
                                button_palette=Palette::Standard
                                button_style=Style::Light
                                button_size=Size::Medium
                            >{ "Unavailable" }</Button>
                        })
                        body=Some(self.view_mark_tags(false))
                    />
                </Item>
            </Container>
        }
    }

    pub fn view_mark_tags(&self, selected: bool) -> Html {
        self.generator
            .mark
            .get_marks()
            .filter(|&(_, available)| available == selected)
            .map(|(mark, available)| {
                html! {
                    <Text
                        id=format!("tag-{}", mark as u32)
                        class_name="draggable-tag"
                        draggable=true
                        interaction_effect=true
                        ondragstart_signal=self.link.callback(Msg::DraggedMark)
                        onclick_signal=self.link.callback(move |_| Msg::ToggleMark(mark))
                        text_type=TextType::Tag
                        text_size=Size::Medium
                        plain_text=mark.to_string()
                        html_text=None
                        text_style=Style::Regular
                        text_palette=if available { Palette::Success } else { Palette::Standard }
                    />
                }
            })
            .collect()
    }
}
