mod gql;

use gql::{all_todos, create_new_todo, create_todo, fetch_all_todos, FetchError};

use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use yew::events::{InputData, KeyboardEvent};
use yew::prelude::*;
use yew::web_sys::HtmlInputElement;
use yewtil::future::LinkFuture;

pub enum TodoFetchState {
    FetchAllTodosSuccess(Vec<all_todos::AllTodosTodos>),
    CreateTodoSuccess(create_new_todo::CreateNewTodoCreateTodo),
    Failed(FetchError),
}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

#[derive(Clone, PartialEq)]
struct TodoModel {
    id: i64,
    body: String,
    complete: bool,
    editing: bool,
}

pub enum TodoMessage {
    ChangeNewInput(String),
    ClearCompleted,
    Add,
    Toggle(usize),
    Delete(usize),
    Edit(usize),
    ChangeEditInput(usize, String),
    Update(usize),
    ToggleAll,
    SetFilter(Filter),
    CancelEdit(usize),
    Focus,
    Fetch(TodoFetchState),
    None,
}

#[derive(ToString, EnumIter, Clone, PartialEq, Copy)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn as_href(&self) -> &str {
        match self {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }

    fn fits(&self, item: &TodoModel) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !item.complete,
            Filter::Completed => item.complete,
        }
    }
}

#[derive(Clone)]
pub struct TodoState {
    text: String,
    list: Vec<TodoModel>,
    completed: i32,
    filter: Filter,
}

pub struct TodoApp {
    state: TodoState,
    link: ComponentLink<Self>,
    edit_ref: NodeRef,
}

impl Component for TodoApp {
    type Message = TodoMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let app = TodoApp {
            state: TodoState {
                text: "".to_owned(),
                list: vec![],
                completed: 0,
                filter: Filter::All,
            },
            link,
            edit_ref: NodeRef::default(),
        };
        app.link.send_future(fetch_all());
        app
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            TodoMessage::ChangeNewInput(value) => {
                self.state.text = value;
            }
            TodoMessage::ClearCompleted => {
                self.state.list = self
                    .state
                    .list
                    .iter()
                    .filter(|item| !item.complete)
                    .cloned()
                    .collect::<Vec<TodoModel>>();
            }
            TodoMessage::Add => {
                let text = self.state.text.trim().to_owned();
                if text.is_empty() {
                    return false;
                }
                self.link.send_future(async {
                    match create_todo(text).await {
                        Ok(new_todo) => {
                            TodoMessage::Fetch(TodoFetchState::CreateTodoSuccess(new_todo))
                        }
                        Err(err) => TodoMessage::Fetch(TodoFetchState::Failed(err)),
                    }
                });
            }
            TodoMessage::Toggle(index) => {
                let item = self.state.list.get_mut(index).unwrap();
                item.complete = !item.complete;
            }
            TodoMessage::Delete(index) => {
                self.state.remove(index);
            }
            TodoMessage::Edit(index) => {
                let item = self.state.list.get_mut(index).unwrap();
                item.editing = true;
            }
            TodoMessage::ChangeEditInput(index, value) => {
                let item = self.state.list.get_mut(index).unwrap();
                let text = value.trim().to_owned();
                item.body = text;
            }
            TodoMessage::Update(index) => {
                let body = self
                    .state
                    .list
                    .get_mut(index)
                    .unwrap()
                    .body
                    .trim()
                    .to_owned();
                if body.is_empty() {
                    self.state.remove(index);
                }
                let item = self.state.list.get_mut(index).unwrap();
                item.editing = false;
            }
            TodoMessage::ToggleAll => self.state.list.iter_mut().for_each(|item| {
                item.complete = !item.complete;
            }),
            TodoMessage::SetFilter(filter) => {
                self.state.filter = filter;
            }
            TodoMessage::CancelEdit(index) => {
                let item = self.state.list.get_mut(index).unwrap();
                item.editing = false;
            }
            TodoMessage::Focus => {
                if let Some(elem) = self.edit_ref.cast::<HtmlInputElement>() {
                    elem.focus().unwrap();
                }
            }
            TodoMessage::Fetch(TodoFetchState::FetchAllTodosSuccess(todos)) => {
                self.state.list = todos
                    .iter()
                    .map(|todo| TodoModel {
                        id: todo.id,
                        body: todo.body.to_owned(),
                        complete: todo.complete,
                        editing: false,
                    })
                    .collect::<Vec<TodoModel>>();
            }
            TodoMessage::Fetch(TodoFetchState::CreateTodoSuccess(_)) => {
                self.state.text = "".to_string();
                self.link.send_future(fetch_all());
            }
            TodoMessage::Fetch(TodoFetchState::Failed(err)) => {
                yew::web_sys::console::log_1(&err.err);
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
            <div id="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        {self.render_new_input()}
                    </header>
                    {self.render_main()}
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/goppdsk/" target="_blank">{ "ここ" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

async fn fetch_all() -> TodoMessage {
    match fetch_all_todos().await {
        Ok(todos) => TodoMessage::Fetch(TodoFetchState::FetchAllTodosSuccess(todos)),
        Err(err) => TodoMessage::Fetch(TodoFetchState::Failed(err)),
    }
}

impl TodoApp {
    fn render_main(&self) -> Html {
        let list = self
            .state
            .list
            .iter()
            .filter(|item| self.state.filter.fits(*item))
            .cloned()
            .collect::<Vec<TodoModel>>();

        html! {
            <>
                {
                    if !list.is_empty() {
                        html! {
                            <section class="main">
                                {self.render_toggle_all()}
                                <ul class="todo-list">
                                    { list.iter().enumerate().map(|(i, item)| {
                                        self.render_item(i, item)
                                    }).collect::<Html>()}
                                </ul>
                            </section>
                        }
                    } else {
                        html!{<></>}
                    }
                }
                {
                    if self.state.total() > 0 {
                        html! {
                            <footer class="footer">
                                <span class="todo-count">
                                    <strong>{ self.state.clone().total() }</strong>
                                    { " item(s) left" }
                                </span>
                                <ul class="filters">
                                    {Filter::iter().map(|filter| self.render_filter(filter)).collect::<Html>()}
                                </ul>
                                {
                                    if self.state.clone().total_completed() > 0 {
                                        html! {
                                            <button class="clear-completed" onclick=self.link.callback(|_| TodoMessage::ClearCompleted)>
                                                { format!("Clear completed ({})", self.state.clone().total_completed()) }
                                            </button>
                                        }
                                    } else {
                                        html! {<></>}
                                    }
                                }
                            </footer>
                        }
                    } else {
                        html!{<></>}
                    }
                }
            </>
        }
    }

    fn render_toggle_all(&self) -> Html {
        html! {
            <>
                <input
                    type="checkbox"
                    class="toggle-all"
                    id="toggle-all"
                    checked=self.state.is_all_completed()
                    onclick=self.link.callback(|_| TodoMessage::ToggleAll)
                />
                <label for="toggle-all" />
            </>
        }
    }

    fn render_new_input(&self) -> Html {
        html! {
            <input
                type="text"
                value=self.state.text
                oninput=self.link.callback(|data: InputData| TodoMessage::ChangeNewInput(data.value))
                onkeypress=self.link.callback(|e: KeyboardEvent| {
                    if e.key() == "Enter" { TodoMessage::Add } else { TodoMessage::None }
                })
                class="new-todo"
                placeholder="What needs to be done?"
            />
        }
    }

    fn render_item(&self, index: usize, item: &TodoModel) -> Html {
        let mut class = Classes::from("todo");
        if item.complete {
            class.push(" completed");
        }
        if item.editing {
            class.push(" editing");
        }
        html! {
            <li class=class>
                {
                    if item.editing {
                        self.render_editing_list(index, item)
                    }  else {
                        self.render_list(index, item)
                    }
                }
            </li>
        }
    }

    fn render_list(&self, index: usize, item: &TodoModel) -> Html {
        html! {
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    checked=item.complete
                    onclick=self.link.callback(move |_| TodoMessage::Toggle(index))
                />
                <label ondblclick=self.link.callback(move |_| TodoMessage::Edit(index))>{item.body.to_owned()}</label>
                <button class="destroy" onclick=self.link.callback(move |_| TodoMessage::Delete(index)) />
            </div>
        }
    }

    fn render_editing_list(&self, index: usize, item: &TodoModel) -> Html {
        html! {
            <input
                ref=self.edit_ref.clone()
                class="edit"
                value=item.body
                oninput=self.link.callback(move |data: InputData| TodoMessage::ChangeEditInput(index, data.value))
                onkeypress=self.link.callback(move |e: KeyboardEvent| {
                    if e.key() == "Enter" { TodoMessage::Update(index) } else { TodoMessage::None }
                })
                // Force move forcus.
                onmouseover=self.link.callback(|_| TodoMessage::Focus)
                onblur=self.link.callback(move |_| TodoMessage::CancelEdit(index))
            />
        }
    }

    fn render_filter(&self, filter: Filter) -> Html {
        let mut class = "";
        if self.state.filter == filter {
            class = "selected";
        }
        html! {
            <li>
                <a
                    href=filter.as_href()
                    class=class onclick=self.link.callback(move |_| TodoMessage::SetFilter(filter))
                >
                    {filter}
                </a>
            </li>
        }
    }
}

impl TodoState {
    fn total(&self) -> i32 {
        self.list.len() as i32
    }

    fn total_completed(self) -> i32 {
        self.list.into_iter().filter(|t| t.complete).count() as i32
    }

    fn is_all_completed(&self) -> bool {
        self.list.iter().all(|item| item.complete)
    }

    fn remove(&mut self, index: usize) {
        let list = self
            .list
            .iter()
            .enumerate()
            .filter(|&(_, item)| self.filter.fits(item))
            .collect::<Vec<_>>();
        let &(index, _) = list.get(index).unwrap();
        self.list.remove(index);
    }
}
