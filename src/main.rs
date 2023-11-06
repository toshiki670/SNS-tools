mod app;
mod components;
mod pages;
mod providers;
mod routes;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
