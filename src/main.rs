mod app;
mod footer;

use app::App;
use footer::Footer;

fn main() {
    yew::Renderer::<App>::new().render();
}
