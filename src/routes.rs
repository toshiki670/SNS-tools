use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::MainLayout;
use crate::pages::dashboard::Dashboard;
use crate::pages::not_found::NotFound;
use crate::pages::x::X;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/x")]
    X,
    #[at("/")]
    Dashboard,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::X => {
            html! { <X /> }
        }
        Route::Dashboard => {
            html! { <Dashboard /> }
        }
        Route::NotFound => {
            html! { <NotFound /> }
        }
    }
}

#[function_component]
pub fn AppRoutes() -> Html {
    html! {
        <MainLayout>
            <Switch<Route> render={switch} />
        </MainLayout>
    }
}
