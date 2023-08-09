use interface::{content::Content, header::Header};
use yew::prelude::*;

mod cpu;
mod interface;
mod memory;

#[function_component]
fn App() -> Html {
    html! {
        <div id="root">
            <Header/>
            <Content/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
