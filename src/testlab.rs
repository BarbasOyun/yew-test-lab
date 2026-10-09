use crate::{YewTestLab};

use yew::{Callback, Properties, TargetCast, html, prelude::*};
use implicit_clone::unsync::IString;

#[component]
pub fn TestLabPanel() -> Html {
    html! { <p>
    <CharName />
    </p> }
}

#[component]
pub fn CharName() -> Html {
    let store = use_context::<UseReducerHandle<YewTestLab>>()
        .expect("NameDisplay must be wrapped in ContextProvider");
    let name_state = &store.test_lab.name_state;

    html! { <p>
    {"Name : "}{&name_state.name}<br />
    {"Family Name : "}{&name_state.family_name}<br />
    {"Title : "}{&name_state.title}<br />
    </p> }
}
