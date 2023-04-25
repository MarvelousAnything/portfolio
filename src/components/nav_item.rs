use yew::prelude::*;
use yew_router::components::Link;

use crate::routes::MainRoute;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub route: MainRoute,
    #[prop_or_default]
    pub active: bool,
}

#[function_component(NavItem)]
pub fn nav_item(props: &Props) -> Html {
    html! {
        <li class={format!("nav-item {}", if props.active {"active"} else {""})}>
            <Link<MainRoute> to={props.route.clone()} classes="nav-link">{&props.title}</Link<MainRoute>>
        </li>
    }
}
