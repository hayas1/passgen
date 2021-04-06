use yew::prelude::*;

use crate::password::{generator::PasswordGenerator, password::Password};

pub struct GeneratorPane {
    link: ComponentLink<Self>,
    generator: PasswordGenerator,
    password: Password,
}

pub enum Msg {
    Generate,
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
            Msg::Generate => self.password = self.generator.generate_password().unwrap(),
        }
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
                    <p>{ format!("{:?}", self.password) }</p>
                    <button onclick=self.link.callback(|_| Msg::Generate)>{ "generate!" }</button>
                </div>
            </div>
        }
    }
}
