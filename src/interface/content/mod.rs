use crate::cpu::{registers::Registers, Cpu};
use yew::prelude::*;

use cpu::CpuStats;

mod cpu;
mod game;

pub struct Content {
    cpu: Cpu,
    tab: Tab,
}

pub enum Msg {
    Step,
    Reset,
    TabClicked(Tab),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    Cpu,
    Memory,
}

#[derive(Properties, PartialEq)]
struct TabProps {
    class: String,
    onclick: Callback<MouseEvent>,
}

// Utility functions.
impl Content {
    fn tab_props(&self, ctx: &Context<Self>, tab: Tab) -> TabProps {
        TabProps {
            class: format!("tab {}", if tab == self.tab { "active" } else { "" }),
            onclick: ctx.link().callback(move |_| Msg::TabClicked(tab)),
        }
    }
}

impl Component for Content {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut cpu = Cpu::new();
        cpu.load_program(&[0xA9, 0x0A, 0xAA]);
        Self { cpu, tab: Tab::Cpu }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cpu_props = self.tab_props(ctx, Tab::Cpu);
        let memory_props = self.tab_props(ctx, Tab::Memory);

        html! {
            <div class="content">
                <div class="sub-content"></div>
                <div class="sub-content">
                    <div class="tabs">
                        <button onclick={memory_props.onclick} class={memory_props.class}>
                            {"Memory"}
                        </button>
                        <button onclick={cpu_props.onclick} class={cpu_props.class}>
                            {"CPU"}
                        </button>
                    </div>
                    {match self.tab {
                        Tab::Cpu => {
                            html! {
                                <CpuStats registers={self.cpu.registers}/>
                            }
                        },
                        Tab::Memory => html! {
                            <div class="stats">
                                <h1>
                                    {"Memory"}
                                </h1>
                            </div>
                        }
                    }}
                    <div class="control">
                        <button onclick={ctx.link().callback(|_| Msg::Step)}>{"Step"}</button>
                        <button onclick={ctx.link().callback(|_| Msg::Reset)}>{"Reset"}</button>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Step => return self.cpu.cycle(),
            Msg::Reset => self.cpu.load_program(&[0xA9, 0x0A, 0xAA]),
            Msg::TabClicked(tab) => self.tab = tab,
        }

        true
    }
}
