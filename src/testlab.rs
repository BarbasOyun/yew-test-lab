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

        // let mut new_state = self;
        // let state = Rc::make_mut(&mut new_state);

        // match action {
        //     TestLabEffect::UpdateNames(new_name) => {
        //         state.name_state = new_name;
        //     }
        // }

        // new_state
    }
}

#[derive(Properties, PartialEq)]
pub struct CharNameProps {
    pub name_state: NameProps,
}

// COMPONENTS

#[component]
pub fn TestLabPanel() -> Html {
    let store = use_context::<UseReducerHandle<YewTestLab>>().expect("No Store Context");
    let page_state = use_reducer(TestLab::default);

    html! { <p>
    <ContextProvider<UseReducerHandle<TestLab>> context={page_state.clone()}>
        <CharName name_state={store.name_state.clone()} />
        <CharName name_state={page_state.name_state.clone()} />
        <br />
        {test(page_state.name_state.clone())}
        <br />
        <ChangeNameButton />
    </ContextProvider<UseReducerHandle<TestLab>>>
    </p> }
}

#[component]
pub fn CharName(props: &CharNameProps) -> Html {
    html! { <p>
    {"Name : "}{&props.name_state.name}<br />
    {"Family Name : "}{&props.name_state.family_name}<br />
    {"Title : "}{&props.name_state.title}<br />
    </p> }
}

#[component]
pub fn CharName2(props: &NameProps) -> Html {
    html! { <p>
    {"Name : "}{&props.name}<br />
    {"Family Name : "}{&props.family_name}<br />
    {"Title : "}{&props.title}<br />
    </p> }
}

fn test(props: NameProps) -> VNode {
    html! { <p>
    <CharName2 name={props.name} family_name={props.family_name} title={props.title} />
    </p> }
}

#[function_component(ChangeNameButton)]
fn change_names() -> Html {
    let store = use_context::<UseReducerHandle<YewTestLab>>().expect("No Store Context");
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
