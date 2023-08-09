use crate::cpu::Cpu;
use yew::prelude::*;

mod cpu;
mod game;

pub struct Content {
    cpu: Cpu,
}

impl Component for Content {
    // TODO: Another msg for loading in a program via a textbox for now.
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x0A, 0xAA]);
        Self { cpu }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let step_callback = ctx.link().callback(|_| {});
        html! {
            <div>
                <h1>
                    {format!("Program Counter: {:#06x}", self.cpu.registers.program_counter)}
                </h1>
                <button onclick={step_callback}>{"Step"}</button>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.cpu.cycle()
    }
}
