use super::Registers;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub registers: Registers,
}

#[function_component]
pub fn CpuStats(props: &Props) -> Html {
    html! {
        <div class="stats">
            <h1>
                {format!("Program Counter: {:#06x}", props.registers.program_counter)}
            </h1>
            <h1>
                {format!("Accumulator: {:#06x}", props.registers.accumulator)}
            </h1>
            <h1>
                {format!("Index X: {:#06x}", props.registers.index_x)}
            </h1>
            <h1>
                {format!("Index Y: {:#06x}", props.registers.index_y)}
            </h1>
            <h1>
                {format!("Stack Pointer: {:#06x}", props.registers.stack_pointer)}
            </h1>
            <h1>
                {format!("Status: {:#06x}", props.registers.status.get_byte())}
            </h1>
        </div>
    }
}
