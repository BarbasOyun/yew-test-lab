use crate::{NameProps, YewTestLab};

use implicit_clone::unsync::IString;
use std::rc::Rc;
use yew::{Callback, Properties, TargetCast, html, prelude::*, virtual_dom::VNode};

// Page State
#[derive(Clone, Default, PartialEq)]
pub struct TestLab {
    pub name_state: NameProps,
}

pub enum TestLabEffect {
    UpdateNames(NameProps),
}

// Reduced Style 2
impl Reducible for TestLab {
    type Action = TestLabEffect;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TestLabEffect::UpdateNames(new_name) => {
                return Rc::new(Self {
                    name_state: new_name,
                    ..(*self).clone()
                });
            }
        }
    }
}

// COMPONENTS

#[component]
pub fn TestLabPanel() -> Html {
    let store = use_context::<UseReducerHandle<YewTestLab>>().expect("No Store Context");
    let page_state = use_reducer(TestLab::default);

    html! { <p>
    <ContextProvider<UseReducerHandle<TestLab>> context={page_state.clone()}>
        {"Global Data"}
        {char_name(store.name_state.clone())}
        <br />
        {"Page Data :"}
        {char_name(page_state.name_state.clone())}
        <br />
        <ChangeNameButton />
    </ContextProvider<UseReducerHandle<TestLab>>>
    </p> }
}

#[component]
pub fn CharName(props: &NameProps) -> Html {
    html! { <p>
    {"Name : "}{&props.name}<br />
    {"Family Name : "}{&props.family_name}<br />
    {"Title : "}{&props.title}<br />
    </p> }
}

pub fn char_name(props: NameProps) -> VNode {
    html! { <p>
    <CharName name={props.name} family_name={props.family_name} title={props.title} />
    </p> }
}

/// Update Page Names
#[function_component(ChangeNameButton)]
fn change_names() -> Html {
    // let store = use_context::<UseReducerHandle<YewTestLab>>().expect("No Store Context");
    let page_state = use_context::<UseReducerHandle<TestLab>>().expect("No Page Context");

    let change_names = {
        let page_state = page_state.clone();
        Callback::from(move |_| {
            page_state.dispatch(TestLabEffect::UpdateNames(NameProps {
                name: IString::from("Uriel2"),
                family_name: IString::from("Kiel2"),
                title: IString::from("Pain Seeker2"),
            }))
        })
    };

    html! {
        <div>
            <button onclick={change_names}>{ "Change Names" }</button>
        </div>
    }
}
