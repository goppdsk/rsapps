use yew::events::{InputData, KeyboardEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct TodoModel {
    body: String,
    complete: bool,
}

#[derive(Clone, Properties, PartialEq)]
struct TodoItemProps {
    item: TodoModel,
}

struct TodoItem {
    props: TodoItemProps,
}

impl Component for TodoItem {
    type Message = ();
    type Properties = TodoItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <li class="todo">
                <div class="view">
                    <label>{self.props.item.body.to_owned()}</label>
                </div>
            </li>
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
struct TodoListProps {
    list: Vec<TodoModel>,
}

struct TodoList {
    props: TodoListProps,
}

impl Component for TodoList {
    type Message = ();
    type Properties = TodoListProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <ul class="todo-list">
                { self.props.list.iter().map(|item| {
                    html! {
                      <TodoItem item=item />
                    }
                }).collect::<Html>()}
            </ul>
        }
    }
}

pub enum TodoMessage {
    ChangeInput(String),
    ClearCompleted,
    Add,
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
                println!("aaaa");
                self.state.list.push(TodoModel {
                    body: self.state.text.to_owned(),
                    complete: false,
                });
                self.state.text = "".to_string();
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
                    <TodoList list=self.state.list.clone() />
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

impl TodoState {
    fn total(self) -> i32 {
        self.list.into_iter().filter(|t| !t.complete).count() as i32
    }

    fn total_completed(self) -> i32 {
        self.completed
    }
}
