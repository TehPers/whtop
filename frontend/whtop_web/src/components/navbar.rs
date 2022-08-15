use crate::routes::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class={"navbar"}>
            // TODO:
            // <a class={"navbar-item"} href={"/"}>
            //     <img src={"/static/images/logo.png"} alt={"logo"} />
            // </a>
            <NavbarButton route={AppRoute::Dashboard}>{"Dashboard"}</NavbarButton>
        </div>
    }
}

#[derive(Clone, PartialEq, Debug, Properties)]
struct NavbarButtonProps {
    route: AppRoute,
    #[prop_or_default]
    children: Children,
}

#[function_component(NavbarButton)]
fn navbar_button(props: &NavbarButtonProps) -> Html {
    html! {
        <Link<AppRoute> to={props.route.clone()} classes={"navbar-item"} >
            { for props.children.iter() }
        </Link<AppRoute>>
    }
}
