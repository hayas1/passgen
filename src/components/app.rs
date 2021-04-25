use super::{generator_pane::GeneratorPane, header::Header};
use yew::prelude::*;

pub struct PassGenApp {}

impl Component for PassGenApp {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <header>
                    <Header />
                </header>
                <main>
                    <GeneratorPane />
                </main>
            </>
        }
    }
}
