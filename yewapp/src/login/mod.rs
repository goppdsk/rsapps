use crate::utils::FetchError;
use crate::{App, AppMessage};
use gql::sign_up;
use yew::events::{FocusEvent, InputData};
use yew::prelude::*;
use yewtil::future::LinkFuture;

mod gql;

pub struct FormState {
    username: String,
    password: String,
    is_sign_up: bool,
}

pub enum LoginFetchState {
    LoginSuccess(String),
    Failed(FetchError),
}

pub enum LoginMessage {
    SignUp,
    Login,
    Fetch(LoginFetchState),
    ChangeUsername(String),
    ChangePassword(String),
    ToggleLogin,
    ToggleSignUp,
}

#[derive(Properties, Clone)]
pub struct LoginAppProps {
    pub app_link: ComponentLink<App>,
}

pub struct LoginApp {
    props: LoginAppProps,
    state: FormState,
    link: ComponentLink<Self>,
}

impl Component for LoginApp {
    type Message = LoginMessage;
    type Properties = LoginAppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            state: FormState {
                username: "".to_string(),
                password: "".to_string(),
                is_sign_up: false,
            },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            LoginMessage::SignUp => {
                let username = self.state.username.to_owned();
                let password = self.state.password.to_owned();
                self.link.send_future(async move {
                    match sign_up(username, password).await {
                        Ok(jwt) => LoginMessage::Fetch(LoginFetchState::LoginSuccess(jwt)),
                        Err(err) => LoginMessage::Fetch(LoginFetchState::Failed(err)),
                    }
                })
            }
            LoginMessage::Login => {}
            LoginMessage::Fetch(LoginFetchState::LoginSuccess(jwt)) => {
                self.props.app_link.send_message(AppMessage::Authenticated);
            }
            LoginMessage::ChangeUsername(username) => {
                self.state.username = username;
            }
            LoginMessage::ChangePassword(password) => {
                self.state.password = password;
            }
            LoginMessage::Fetch(LoginFetchState::Failed(err)) => {
                yew::web_sys::console::log_1(&err.err);
            }
            LoginMessage::ToggleLogin => {
                self.state.is_sign_up = false;
            }
            LoginMessage::ToggleSignUp => {
                self.state.is_sign_up = true;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            if self.state.is_sign_up {
                {self.render_sing_up()}
            } else {
                {self.render_login()}
            }
        }
    }
}

impl LoginApp {
    fn render_sing_up(&self) -> Html {
        html! {
            <div>
                <form
                    onsubmit=self.link.callback(|e: FocusEvent| {
                        e.prevent_default();
                        LoginMessage::SignUp
                    })>
                    <label for="uname"><b>{"Username"}</b></label>
                    <input
                        type="text"
                        placeholder="Enter Username"
                        name="uname"
                        required=true
                        oninput=self.link.callback(|data: InputData| LoginMessage::ChangeUsername(data.value))
                    />
                    <label for="psw"><b>{"Password"}</b></label>
                    <input
                        type="password"
                        placeholder="Enter Password"
                        name="psw"
                        required=true
                        oninput=self.link.callback(|data: InputData| LoginMessage::ChangePassword(data.value))
                    />
                    <button type="submit">{"Sign Up"}</button>
                    <div>
                        <span>{"Do you have already account?"}</span>
                        <a
                            href="#"
                            onclick=self.link.callback(|_| LoginMessage::ToggleLogin)
                        >
                            {"Login"}
                        </a>
                    </div>
                </form>
            </div>
        }
    }

    fn render_login(&self) -> Html {
        html! {
            <div>
                <form
                    onsubmit=self.link.callback(|e: FocusEvent| {
                        e.prevent_default();
                        LoginMessage::Login
                    })>
                    <label for="uname"><b>{"Username"}</b></label>
                    <input
                        type="text"
                        placeholder="Enter Username"
                        name="uname"
                        required=true
                        oninput=self.link.callback(|data: InputData| LoginMessage::ChangeUsername(data.value))
                    />
                    <label for="psw"><b>{"Password"}</b></label>
                    <input
                        type="password"
                        placeholder="Enter Password"
                        name="psw"
                        required=true
                        oninput=self.link.callback(|data: InputData| LoginMessage::ChangePassword(data.value))
                    />
                    <button type="submit">{"Login"}</button>
                    <div>
                        <span>{"Do you sign up?"}</span>
                        <a
                            href="#"
                            onclick=self.link.callback(|_| LoginMessage::ToggleSignUp)
                        >
                            {"Sing up"}
                        </a>
                    </div>
                </form>
            </div>
        }
    }
}
