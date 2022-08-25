use crate::{
    components::{dashboard::Dashboard, Navbar},
    contexts::{HttpClient, Theme},
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
        <WithContext>
            <Navbar />
            <BrowserRouter>
                <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        </WithContext>
    }
}

#[derive(Clone, PartialEq, Debug, Properties)]
struct WithContextProps {
    children: Children,
}

#[function_component(WithContext)]
fn with_context(props: &WithContextProps) -> Html {
    let theme = Theme::default();
    let client = HttpClient::default();
    html! {
        <ContextProvider<Theme> context={theme}>
            <ContextProvider<HttpClient> context={client}>
                { for props.children.iter() }
            </ContextProvider<HttpClient>>
        </ContextProvider<Theme>>
    }
}
