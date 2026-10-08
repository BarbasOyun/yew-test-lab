mod testlab;

use crate::testlab::{CharName, TestLab, TestLabPanel, TestLabAction};

use std::rc::Rc;
use yew::{Callback, Properties, html, prelude::*};
use yew_router::prelude::*;

pub enum StoreAction {
    Counter(CounterAction),
    TestLab(TestLabAction),
}

// Store
#[derive(Clone, Default, PartialEq)]
struct YewTestLab {
    count: i32,
    test_lab: TestLab,
}

impl Reducible for YewTestLab {
    type Action = StoreAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_count = match action {
            StoreAction::Increment => self.count + 1,
            StoreAction::Decrement => self.count - 1,
            _ =>
        };

        Rc::new(Self {
            count: new_count,
            ..(*self).clone()
        })
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[component]
fn App() -> Html {
    let store = use_reducer(YewTestLab::default);

    html! {
        <BrowserRouter>
            <NavItems />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
    // html! {
    //     <ContextProvider<UseReducerHandle<YewTestLab>> context={store}>
    //         <BrowserRouter>
    //             <NavItems />
    //             <Switch<Route> render={switch} />
    //         </BrowserRouter>
    //     </ContextProvider<UseReducerHandle<YewTestLab>>>
    // }
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
    let store = use_reducer(YewTestLab::default);

    html! {<>
    <h1>{ "Home" }</h1>
    <h2>{ "Yew Test Lab" }</h2>
    <p>{ "Testing stuff" }</p>
    <Counter />
    <CharName name_gen={store.test_lab.name_state.clone()}/>
    </>
    }
}

#[function_component(ChangeNameButton)]
fn change_names() -> Html {
    let on_increment = {
        let store = store.clone();
        Callback::from(move |_| store.dispatch(StoreAction::Increment))
    };

    html! {
        <div>
            <button onclick={on_increment}>{ "+1" }</button>
        </div>
    }
}

// COUNTER

#[derive(Clone, PartialEq, Default)]
pub struct CounterState {
    pub count: i32,
}

pub enum CounterAction {
    Increment,
    Decrement,
}

impl CounterState {
    pub fn reduce(&mut self, action: CounterAction) {
        match action {
            CounterAction::Increment => self.count += 1,
            CounterAction::Decrement => self.count -= 1,
        }
    }
}

#[function_component(Counter)]
pub fn counter() -> Html {
    // Initialize the reducer state
    let store = use_reducer(YewTestLab::default);

    // Create action handlers
    let on_increment = {
        let store = store.clone();
        Callback::from(move |_| store.dispatch(StoreAction::Increment))
    };

    let on_decrement = {
        let store = store.clone();
        Callback::from(move |_| store.dispatch(StoreAction::Decrement))
    };

    html! {
        <div>
            <p>{ format!("Count: {}", store.count) }</p>
            <button onclick={on_increment}>{ "+1" }</button>
            <button onclick={on_decrement}>{ "-1" }</button>
        </div>
    }
}
