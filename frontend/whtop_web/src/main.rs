mod components;
mod contexts;
mod layers;
mod routes;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::start_app::<components::App>();
}
