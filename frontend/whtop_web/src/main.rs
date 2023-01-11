mod components;
mod contexts;
mod layers;
mod routes;

fn main() {
    yew::Renderer::<components::App>::new().render();
}
