use crate::cpu::Cpu;
use yew::prelude::*;

mod cpu;
mod game;

pub struct Content {
    cpu: Cpu,
    tab: Tab,
}

pub enum Tab {
    Cpu,
    Memory,
}

pub enum Msg {
    Step,
    TabClicked(Tab),
}

impl Component for Content {
    // TODO: Another msg for loading in a program via a textbox for now.
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x0A, 0xAA]);
        Self { cpu, tab: Tab::Cpu }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="content">
                <div class="sub-content"></div>
                <div class="sub-content">
                    <div class="tabs">
                        <button class="tab" onclick={ctx.link().callback(|_| Msg::TabClicked(Tab::Cpu))}>
                            {"CPU"}
                        </button>
                        <button class="tab" onclick={ctx.link().callback(|_| Msg::TabClicked(Tab::Memory))}>
                            {"Memory"}
                        </button>
                    </div>
                    {match self.tab {
                        Tab::Cpu => html! {
                            <div>
                                <h1>
                                    {format!("Program Counter: {:#06x}", self.cpu.registers.program_counter)}
                                </h1>
                            </div>
                        },
                        Tab::Memory => html! { <h1>{"Memory"}</h1> }
                    }}
                    <button onclick={ctx.link().callback(|_| Msg::Step)}>{"Step"}</button>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Step => return self.cpu.cycle(),
            Msg::TabClicked(tab) => self.tab = tab,
        }

        true
    }
}
