use yew::{Callback, Properties, TargetCast, html, prelude::*};
use implicit_clone::unsync::IString;

#[derive(Clone, Default, PartialEq)]
pub struct TestLab {
    pub name_state: NameProps,
}

pub enum TestLabAction {
    UpdateNames(NameProps),
}

impl TestLab {
    pub fn reduce(&mut self, action: TestLabAction) {
        match action {
            TestLabAction::UpdateNames(new_name) => self.name_state = new_name,
        }
    }
}

#[component]
pub fn TestLabPanel() -> Html {
    let name_state = use_state(|| NameProps::default());

    html! { <p>
    <CharName name_gen={(*name_state).clone()}/>
    </p> }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NameProps {
    pub name: AttrValue,
    pub family_name: AttrValue,
    pub title: AttrValue,
}

impl Default for NameProps {
    fn default() -> Self {
        NameProps {
            name: IString::from("Default Name"),
            family_name: IString::from("Default Family Name"),
            title: IString::from("Default Title"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct NameGenProps {
    pub name_gen: NameProps,
}

#[component]
pub fn CharName(props: &NameGenProps) -> Html {
    html! { <p>
    {"Name : "}{&props.name_gen.name}<br />
    {"Family Name : "}{&props.name_gen.family_name}<br />
    {"Title : "}{&props.name_gen.title}<br />
    </p> }
}
