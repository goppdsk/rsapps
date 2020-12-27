use yew::prelude::*;

mod todo;
use todo::Todo;

#[derive(Clone, Properties, Default)]
struct AppProps {
    children: Children,
}

struct App {
    props: AppProps,
}

impl Component for App {
    type Message = ();
    type Properties = AppProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <div id="todomvc-wrapper">
                <Todo />
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

pub fn main() {
    yew::start_app::<App>();
}
