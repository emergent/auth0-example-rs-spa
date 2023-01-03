mod app;
mod component;
mod util;
mod views;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
