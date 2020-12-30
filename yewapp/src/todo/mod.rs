use yew::events::{InputData, KeyboardEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct TodoModel {
    body: String,
    complete: bool,
}

pub enum TodoMessage {
    ChangeInput(String),
    ClearCompleted,
    Add,
    Toggle(usize),
    Delete(usize),
    None,
}

#[derive(Clone)]
pub struct TodoState {
    text: String,
    list: Vec<TodoModel>,
    completed: i32,
}

pub struct Todo {
    state: TodoState,
    link: ComponentLink<Self>,
}

impl Component for Todo {
    type Message = TodoMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: TodoState {
                text: "".to_owned(),
                list: vec![],
                completed: 0,
            },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            TodoMessage::ChangeInput(value) => {
                self.state.text = value;
            }
            TodoMessage::ClearCompleted => {
                self.state.completed += self.state.list.len() as i32;
                self.state.list.clear();
            }
            TodoMessage::Add => {
                if self.state.text.is_empty() {
                    return false;
                }
                self.state.list.push(TodoModel {
                    body: self.state.text.to_owned(),
                    complete: false,
                });
                self.state.text = "".to_string();
            }
            TodoMessage::Toggle(index) => {
                let item = self.state.list.get_mut(index).unwrap();
                item.complete = !item.complete;
            }
            TodoMessage::Delete(index) => {
                self.state.list.remove(index);
            }
            TodoMessage::None => return false,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="todoapp">
                <header class="header">
                    <h1>{ "todos" }</h1>
                    <input
                        type="text"
                        value=self.state.text
                        oninput=self.link.callback(|data: InputData| TodoMessage::ChangeInput(data.value))
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" { TodoMessage::Add } else { TodoMessage::None }
                        })
                        class="new-todo"
                        placeholder="What needs to be done?"
                    />
                </header>
                <section class="main">
                    <ul class="todo-list">
                        { self.state.list.iter().enumerate().map(|(i, item)| {
                            self.render_item(i, item)
                        }).collect::<Html>()}
                    </ul>
                </section>
                <footer class="footer">
                    <span class="todo-count">
                        <strong>{ self.state.clone().total() }</strong>
                        { " item(s) left" }
                    </span>
                    <button class="clear-completed" onclick=self.link.callback(|_| TodoMessage::ClearCompleted)>
                        { format!("Clear completed ({})", self.state.clone().total_completed()) }
                    </button>
                </footer>
            </section>
        }
    }
}

impl Todo {
    fn render_item(&self, index: usize, item: &TodoModel) -> Html {
        let mut class = Classes::from("todo");
        if item.complete {
            class.push(" completed");
        }
        html! {
            <li class=class>
                <div class="view">
                    <input
                        class="toggle"
                        type="checkbox"
                        checked=item.complete
                        onclick=self.link.callback(move |_| TodoMessage::Toggle(index))
                    />
                    <label>{item.body.to_owned()}</label>
                    <button class="destroy" onclick=self.link.callback(move |_| TodoMessage::Delete(index)) />
                </div>
            </li>
        }
    }
}

impl TodoState {
    fn total(self) -> i32 {
        self.list.into_iter().filter(|t| !t.complete).count() as i32
    }

    fn total_completed(self) -> i32 {
        self.completed
    }
}
