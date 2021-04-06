use crate::password::{generator::PasswordGenerator, password::Password};
use web_sys::console;
use yew::prelude::*;

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
}

impl Component for GeneratorPane {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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
        }
        self.password = self.generator.generate_password().unwrap();
        console::log_2(
            &format!("generated password: {}", self.password).into(),
            &format!("generator setting: {:?}", self.generator).into(),
        );
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Password Generator" }</h1>
                <div>
                    { self.view_generated_password() }
                    { self.view_generate_button() }
                    { self.view_length_bar() }
                    { self.view_lower_checkbox() }
                    { self.view_upper_checkbox() }
                    { self.view_numeric_checkbox() }
                </div>
            </div>
        }
    }
}

impl GeneratorPane {
    pub fn view_generated_password(&self) -> Html {
        html! {
            <p>{ format!("{:?}", self.password) }</p>
        }
    }

    pub fn view_generate_button(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::Generate)>{ "generate!" }</button>
        }
    }

    pub fn view_length_bar(&self) -> Html {
        let oninput = self.link.callback(|d: InputData| {
            Msg::EditLength(d.value.parse().expect("range type input should have only integer."))
        });
        html! {
            <div>
                <input type="range" min="10" max="100" oninput=oninput/>
                { self.generator.len }
            </div>
        }
    }

    pub fn view_lower_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleLower);
        html! {
            <div>
                <label for="lower_checkbox">{ "Lower Case" }</label>
                <input id="lower_checkbox" type="checkbox" checked={ self.generator.use_lower } onclick=onclick/>
            </div>
        }
    }

    pub fn view_upper_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleUpper);
        html! {
            <div>
                <label for="upper_checkbox">{ "Upper Case" }</label>
                <input id="upper_checkbox" type="checkbox" checked={ self.generator.use_upper } onclick=onclick/>
            </div>
        }
    }

    pub fn view_numeric_checkbox(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::ToggleNumeric);
        html! {
            <div>
                <label for="numeric_checkbox">{ "Numeric" }</label>
                <input id="numeric_checkbox" type="checkbox" checked={ self.generator.use_numeric } onclick=onclick/>
            </div>
        }
    }
}
