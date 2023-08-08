use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <nav>
            <div>
                <h1>
                    <i class="fa fa-gamepad"/>
                    {" NetNES"}
                </h1>
            </div>
            <div>
                <SocialIcon social="github" href="https://github.com/ShyProton/nes_emulator"/>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
struct SocialProps {
    pub social: String,
    pub href: String,
}

#[function_component]
fn SocialIcon(props: &SocialProps) -> Html {
    html! {
        <a href={props.href.clone()}>
            <i class={format!("social fa fa-{}", props.social)}/>
        </a>
    }
}
