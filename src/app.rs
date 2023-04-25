use crate::components::navbar::NavBar;
use crate::components::webgl_canvas::WebGLCanvas;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

#[function_component(Main)]
pub fn app() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        <>
            <ContextProvider<Theme> context={(*ctx).clone()}>
                <header>
                    <NavBar />
                </header>
                <WebGLCanvas />
            </ContextProvider<Theme>>
        </>
    }
}

#[function_component]
pub fn ThemedButtonHOC() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <ThemedButtonStructComponent {theme}/>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub theme: Theme,
}

struct ThemedButtonStructComponent;

impl Component for ThemedButtonStructComponent {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme = &ctx.props().theme;
        html! {
            <button style={
                format!(
                    "background: {}; color: {}",
                    theme.background,
                    theme.foreground
                )

            }>{ "Click me!" }</button>
        }
    }
}
