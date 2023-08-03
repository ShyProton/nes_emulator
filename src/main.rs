use yew::prelude::*;

use cpu::Cpu;
use memory::Memory;

mod cpu;
mod memory;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xA9, 0x0A, 0xAA, 0x00]); // Transfer 0x0A in A to X.
    cpu.run();

    yew::Renderer::<App>::new().render();
}
