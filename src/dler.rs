use askama::Template;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{ChatId, Requester};
use teloxide::types::ParseMode;
use teloxide::Bot;

struct Account {
    email: String,
    password: String,
}

impl Account {
    pub fn new<S: Into<String>>(email: S, password: S) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    pub ret: i32,
    pub msg: String,
    pub data: Data,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Data {
    Login {
        token: String,
        token_expire: String,
        plan: String,
        plan_time: String,
        money: String,
        aff_money: String,
        today_used: String,
        used: String,
        unused: String,
        traffic: String,
        integral: String,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Body {
    Login {
        email: String,
        passwd: String,
        token_expire: i32,
    },

    Logout {
        access_token: String,
    },
}

impl From<&Account> for Body {
    fn from(account: &Account) -> Self {
        Self::Login {
            email: account.email.to_owned(),
            passwd: account.password.to_owned(),
            token_expire: 1,
        }
    }
}

impl From<&Data> for Body {
    fn from(data: &Data) -> Self {
        match data {
            Data::Login { token, .. } => Body::Logout {
                access_token: token.to_owned(),
            },
        }
    }
}

struct App {
    client: Client,
}

impl App {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn login(&self, account: &Account) -> Option<Response> {
        let body = Body::from(account);
        let response = self
            .client
            .post("https://dler.cloud/api/v1/login")
            .json(&body)
            .send()
            .await;

        match response {
            Ok(res) => match res.json().await {
                Ok(data) => Some(data),
                Err(err) => {
                    tracing::error!(target: "App::login", ?err);
                    None
                }
            },
            Err(err) => {
                tracing::error!(target: "App::login", ?err);
                None
            }
        }
    }

    pub async fn logout(&self, data: &Data) -> bool {
        let body = Body::from(data);
        let response = self
            .client
            .post("https://dler.cloud/api/v1/logout")
            .json(&body)
            .send()
            .await;

        if let Err(err) = response {
            tracing::error!(target: "App::logout", ?err);
            false
        } else {
            true
        }
    }
}

#[derive(Template)]
#[template(path = "dler_message.txt")]
struct Message {
    plan: String,
    plan_time: String,
    today_used: String,
    used: String,
    unused: String,
}

impl From<&Data> for Message {
    fn from(data: &Data) -> Self {
        match data {
            Data::Login {
                plan,
                plan_time,
                today_used,
                used,
                unused,
                ..
            } => Self {
                plan: plan.to_owned(),
                plan_time: plan_time.to_owned(),
                today_used: today_used.to_owned(),
                used: used.to_owned(),
                unused: unused.to_owned(),
            },
        }
    }
}

pub async fn run(email: String, passwd: String, bot: &Bot, chat_id: i64) {
    let app = App::new();
    let account = Account::new(email, passwd);

    if let Some(res) = app.login(&account).await {
        let msg = Message::from(&res.data);
        let msg = msg.render().unwrap();
        if let Err(err) = bot
            .send_message(ChatId(chat_id), &msg)
            .parse_mode(ParseMode::MarkdownV2)
            .await
        {
            tracing::error!(target: "Bot::send_message", ?err);
        }
        app.logout(&res.data).await;
    }
}
