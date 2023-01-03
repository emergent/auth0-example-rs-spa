pub mod app;
pub mod component;
pub mod util;
pub mod views;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
