use strum::IntoEnumIterator;
use yew::events::{InputData, KeyboardEvent};
use yew::prelude::*;
use yew::web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
struct TodoModel {
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

pub struct Todo {
    state: TodoState,
    link: ComponentLink<Self>,
    edit_ref: NodeRef,
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
                filter: Filter::All,
            },
            link,
            edit_ref: NodeRef::default(),
        }
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
                    .filter(|item| item.complete == false)
                    .map(|item| item.clone())
                    .collect::<Vec<TodoModel>>();
            }
            TodoMessage::Add => {
                let text = self.state.text.trim().to_owned();
                if text.is_empty() {
                    return false;
                }
                self.state.list.push(TodoModel {
                    body: text,
                    complete: false,
                    editing: false,
                });
                self.state.text = "".to_string();
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
                    {self.render_new_input()}
                </header>
                {self.render_main()}
            </section>
        }
    }
}

impl Todo {
    fn render_main(&self) -> Html {
        let list = self
            .state
            .list
            .iter()
            .filter(|item| self.state.filter.fits(*item))
            .map(|item| item.clone())
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
        self.list.into_iter().filter(|t| t.complete == true).count() as i32
    }

    fn is_all_completed(&self) -> bool {
        self.list.iter().all(|item| item.complete == true)
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
