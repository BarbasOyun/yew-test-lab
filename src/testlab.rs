use yew::{Callback, Properties, TargetCast, html, prelude::*};

#[component]
pub fn TestLab() -> Html {
    let name_state = use_state(|| yew::props!(NameProps {}));
    
    html! { <p>
    <CharName name_gen={(*name_state).clone()}/>
    </p> }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NameProps {
    #[prop_or(AttrValue::Static("Default Name"))]
    pub name: AttrValue,
    #[prop_or(AttrValue::Static("Default Family Name"))]
    pub family_name: AttrValue,
    #[prop_or(AttrValue::Static("Default Title"))]
    pub title: AttrValue,
}

#[derive(Properties, PartialEq)]
pub struct NameGenProps {
    name_gen: NameProps,
}

#[component]
pub fn CharName(props: &NameGenProps) -> Html {
    html! { <p>
    {"Name : "}{&props.name_gen.name}<br />
    {"Family Name : "}{&props.name_gen.family_name}<br />
    {"Title : "}{&props.name_gen.title}<br />
    </p> }
}
