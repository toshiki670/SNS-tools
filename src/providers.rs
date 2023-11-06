use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct MainLayoutProps {
    pub children: Children,
}

#[function_component]
pub fn AppProvider(props: &MainLayoutProps) -> Html {
    let MainLayoutProps { children } = props.clone();

    html! {
        <BrowserRouter>{children}</BrowserRouter>
    }
}
