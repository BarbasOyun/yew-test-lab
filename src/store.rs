use std::rc::Rc;
use yew::{Properties, prelude::*};
use implicit_clone::unsync::IString;

pub enum StoreEffect {
    Increment,
    Decrement,
    UpdateNames(NameProps),
    Patch, // TODO
}

// Store -> Survive page change
#[derive(Clone, Default, PartialEq)]
pub struct YewTestLab {
    pub count: i32,
    pub name_state: NameProps,
}

// Reduced Style 1
impl Reducible for YewTestLab {
    type Action = StoreEffect;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = self;
        let state = Rc::make_mut(&mut new_state);

        match action {
            StoreEffect::Increment | StoreEffect::Decrement => {
                state.reduce_counter_effect(action);
            }
            StoreEffect::UpdateNames(_) => {
                state.reduce_name_effect(action);
            }
            _ => (),
        }

        new_state
    }
}

impl YewTestLab {
    fn reduce_counter_effect(&mut self, action: StoreEffect) {
        match action {
            StoreEffect::Increment => self.count += 1,
            StoreEffect::Decrement => self.count -= 1,
            _ => unreachable!(),
        }
    }

    fn reduce_name_effect(&mut self, action: StoreEffect) {
        if let StoreEffect::UpdateNames(new_name) = action {
            self.name_state = new_name;
        }
    }
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