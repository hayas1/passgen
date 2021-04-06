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
        html! {
            <div>
                <input
                    type="range"
                    min="10"
                    max="100"
                    // value="{ PasswordGenerator::PASSWORD_DEFAULT_LENGTH }"
                    oninput=self.link.callback(|d: InputData| {
                        Msg::EditLength(d.value.parse().expect("range type input should have only integer."))
                    })
                />
                { self.generator.len }
            </div>
        }
    }
}
