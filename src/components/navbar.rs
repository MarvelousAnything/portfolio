use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::MainRoute;
use crate::components::nav_item::NavItem;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <>
            <BrowserRouter>
                <div class="navbar">
                    <nav>
                        {nav_items()}
                    </nav>
                </div>
            </BrowserRouter>
        </>
    }
}

fn nav_items() -> Html {
    html! {
        <ul class="nav-items">
            <NavItem title="Home" route={MainRoute::Home} active=true />
            <NavItem title="About" route={MainRoute::About} />
            <NavItem title="Contact" route={MainRoute::Contact} />
        </ul>
    }
}
