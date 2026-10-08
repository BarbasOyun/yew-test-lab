mod testlab;

use crate::testlab::{TestLab};

use yew::{Callback, Properties, html, prelude::*};
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/testlab")]
    YewTestLab,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::YewTestLab => html! {
            <TestLab />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <NavItems />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"Home"}</button>
        }
    };

    let go_to_name_gen = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::YewTestLab));
        html! {
            <button {onclick}>{"Name Generator"}</button>
        }
    };

    html! {
        <>
            {go_home_button}
            {go_to_name_gen}
        </>
    }
}

#[component]
pub fn Home() -> Html {
    html! {<>
        <h1>{ "Home" }</h1>
        <h2>{ "Yew Test Lab" }</h2>
        <p>{ "Testing stuff" }</p>
    </>
    }
}