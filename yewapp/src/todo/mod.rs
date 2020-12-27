use yew::events::ChangeData;
use yew::prelude::*;

#[derive(Clone)]
struct TodoModel {
    body: String,
    complete: bool,
}

#[derive(Clone, Properties)]
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

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
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

#[derive(Clone, Properties)]
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

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
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
    Add(ChangeData),
}

pub struct Todo {
    text: String,
    list: Vec<TodoModel>,
    link: ComponentLink<Self>,
}

impl Component for Todo {
    type Message = TodoMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            text: "".to_owned(),
            list: vec![
                TodoModel {
                    body: "a".to_owned(),
                    complete: false,
                },
                TodoModel {
                    body: "b".to_owned(),
                    complete: false,
                },
            ],
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            TodoMessage::Add(data) => match data {
                ChangeData::Value(value) => {
                    self.text = value;
                }
                ChangeData::Files(_) => {}
                ChangeData::Select(_) => {}
            },
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
                        value=self.text
                        onchange=self.link.callback(|data| TodoMessage::Add(data))
                        class="new-todo"
                        placeholder="What needs to be done?"
                    />
                </header>
                <section class="main">
                    <TodoList list=self.list.clone() />
                </section>
            </section>
        }
    }
}
