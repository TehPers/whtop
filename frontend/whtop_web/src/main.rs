mod components;
mod contexts;
mod hooks;
mod routes;
mod layers;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::start_app::<components::App>();
}
