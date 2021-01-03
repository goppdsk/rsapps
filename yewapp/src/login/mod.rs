use crate::utils::FetchError;
use crate::{App, AppMessage};
use gql::{login_with_username, sign_up};
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

pub enum LoadingState {
    Login,
    SignUp,
}

pub enum LoginMessage {
    SignUp,
    Login,
    Loading(LoadingState),
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
    is_loading: bool,
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
            is_loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            LoginMessage::SignUp => {
                self.is_loading = true;
                self.link
                    .send_message(LoginMessage::Loading(LoadingState::SignUp));
            }
            LoginMessage::Login => {
                self.is_loading = true;
                self.link
                    .send_message(LoginMessage::Loading(LoadingState::Login));
            }
            LoginMessage::Loading(LoadingState::SignUp) => {
                let username = self.state.username.to_owned();
                let password = self.state.password.to_owned();
                self.link.send_future(async move {
                    match sign_up(username, password).await {
                        Ok(jwt) => LoginMessage::Fetch(LoginFetchState::LoginSuccess(jwt)),
                        Err(err) => LoginMessage::Fetch(LoginFetchState::Failed(err)),
                    }
                })
            }
            LoginMessage::Loading(LoadingState::Login) => {
                let username = self.state.username.to_owned();
                let password = self.state.password.to_owned();
                self.link.send_future(async move {
                    match login_with_username(username, password).await {
                        Ok(jwt) => LoginMessage::Fetch(LoginFetchState::LoginSuccess(jwt)),
                        Err(err) => LoginMessage::Fetch(LoginFetchState::Failed(err)),
                    }
                })
            }
            LoginMessage::Fetch(LoginFetchState::LoginSuccess(jwt)) => {
                self.is_loading = false;
                self.props
                    .app_link
                    .send_message(AppMessage::Authenticated(jwt));
            }
            LoginMessage::ChangeUsername(username) => {
                self.state.username = username;
            }
            LoginMessage::ChangePassword(password) => {
                self.state.password = password;
            }
            LoginMessage::Fetch(LoginFetchState::Failed(err)) => {
                self.is_loading = false;
                let window = yew::utils::window();
                if let Some(msg) = err.err.as_string() {
                    window.alert_with_message(&msg).unwrap();
                }
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
            <div class="login-page">
                <div class="login-form">
                    {
                        if self.state.is_sign_up {
                            {self.render_sing_up()}
                        } else {
                            {self.render_login()}
                        }
                    }
                </div>
            </div>
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
                    <input
                        type="text"
                        placeholder="username"
                        name="uname"
                        required=true
                        oninput=self.link.callback(|data: InputData| LoginMessage::ChangeUsername(data.value))
                    />
                    <input
                        type="password"
                        placeholder="password"
                        name="psw"
                        required=true
                        oninput=self.link.callback(|data: InputData| LoginMessage::ChangePassword(data.value))
                    />
                    <button type="submit" disabled=self.is_loading>
                        {
                            if self.is_loading {
                                html! {
                                    <i class="fa fa-spinner fa-spin"></i>
                                }
                            } else {
                                html! { "Sign up" }
                            }
                        }
                    </button>
                        <p class="message">{"Do you have already account?"}
                            <a
                                href="#"
                                onclick=self.link.callback(|_| LoginMessage::ToggleLogin)
                            >
                                { "Login" }
                            </a>
                        </p>
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
                    <input
                        type="text"
                        placeholder="username"
                        name="uname"
                        required=true
                        oninput=self.link.callback(|data: InputData| LoginMessage::ChangeUsername(data.value))
                    />
                    <input
                        type="password"
                        placeholder="password"
                        name="psw"
                        required=true
                        oninput=self.link.callback(|data: InputData| LoginMessage::ChangePassword(data.value))
                    />
                    <button type="submit" disabled=self.is_loading>
                        {
                            if self.is_loading {
                                html! {
                                    <i class="fa fa-spinner fa-spin"></i>
                                }
                            } else {
                                html! { "Login" }
                            }
                        }
                    </button>
                        <p class="message">{"Do you sign up?"}
                            <a
                                href="#"
                                onclick=self.link.callback(|_| LoginMessage::ToggleSignUp)
                            >
                                { "Sign up" }
                            </a>
                        </p>
                </form>
            </div>
        }
    }
}
