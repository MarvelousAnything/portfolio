mod app;
mod routes;
mod components;
mod contexts;
mod pages;

use crate::app::Main;

fn main() {
    yew::Renderer::<Main>::new().render();
}
