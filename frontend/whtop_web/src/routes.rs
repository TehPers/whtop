use yew_router::prelude::*;

#[derive(Clone, PartialEq, Debug, Routable)]
pub enum AppRoute {
    #[at("/")]
    Dashboard,
}
