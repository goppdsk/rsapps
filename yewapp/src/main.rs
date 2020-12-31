#![recursion_limit = "512"]
extern crate yew;
#[macro_use]
extern crate strum;

use yew::prelude::*;

mod todo;
use todo::TodoApp;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <TodoApp />
        }
    }
}

pub fn main() {
    yew::start_app::<App>();
}
