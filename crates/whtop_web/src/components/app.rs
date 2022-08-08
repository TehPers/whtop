use crate::{
    components::{dashboard::Dashboard, Navbar},
    routes::AppRoute,
};
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Dashboard => html! {
            <Dashboard />
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Navbar />
            <BrowserRouter>
                <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}
