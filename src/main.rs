mod store;
mod testlab;

use crate::store::{YewTestLab, StoreEffect, NameProps};
use crate::testlab::{CharName, TestLabPanel};

use yew::{Callback, Properties, html, prelude::*};
use yew_router::prelude::*;
use implicit_clone::unsync::IString;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[component]
fn App() -> Html {
    let store = use_reducer(YewTestLab::default);

    html! {
        <ContextProvider<UseReducerHandle<YewTestLab>> context={store}>
            <BrowserRouter>
                <NavItems />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<UseReducerHandle<YewTestLab>>>
    }
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
            <TestLabPanel />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
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
            <button {onclick}>{"Test Lab"}</button>
        }
    };

    html! {
        <>
            {go_home_button}
            {go_to_name_gen}
        </>
    }
}

// HOME PAGE

#[component]
pub fn Home() -> Html {
    let store = use_context::<UseReducerHandle<YewTestLab>>().expect("No Store Context");

    html! {<>
    <h1>{ "Home" }</h1>
    <h2>{ "Yew Test Lab" }</h2>
    <p>{ "Testing stuff" }</p>
    <br />
    <Counter />
    <br />
    <CharName name_state={store.name_state.clone()} />
    <ChangeNameButton />
    </>
    }
}

#[function_component(ChangeNameButton)]
fn change_names() -> Html {
    let store = use_context::<UseReducerHandle<YewTestLab>>()
        .expect("No Store Context");

    let change_names = {
        let store = store.clone();
        Callback::from(move |_| {
            store.dispatch(StoreEffect::UpdateNames(NameProps {
                name: IString::from("Uriel"),
                family_name: IString::from("Kiel"),
                title: IString::from("Pain Seeker"),
            }))
        })
    };

    html! {
        <div>
            <button onclick={change_names}>{ "Change Names" }</button>
        </div>
    }
}

#[function_component(Counter)]
pub fn counter() -> Html {
    let store = use_context::<UseReducerHandle<YewTestLab>>()
        .expect("No Store Context");

    // Create action handlers
    let on_increment = {
        let store = store.clone();
        Callback::from(move |_| store.dispatch(StoreEffect::Increment))
    };

    let on_decrement = {
        let store = store.clone();
        Callback::from(move |_| store.dispatch(StoreEffect::Decrement))
    };

    html! {
        <div>
            <p>{ format!("Count: {}", store.count) }</p>
            <button onclick={on_increment}>{ "+1" }</button>
            <button onclick={on_decrement}>{ "-1" }</button>
        </div>
    }
}
