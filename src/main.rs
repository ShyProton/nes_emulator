use yew::prelude::*;

use cpu::Cpu;
use memory::Memory;

mod cpu;
mod interface;
mod memory;

use interface::header::Header;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Header/>
        </div>
    }
}

fn main() {
    // TODO: Put the Cpu stuff in its own Yew Component section.
    let mut cpu = Cpu::new();
    cpu.load_program(&[0xA9, 0x0A, 0xAA, 0x00]); // Transfer 0x0A in A to X.
    cpu.run();

    yew::Renderer::<App>::new().render();
}
