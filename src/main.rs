use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ReplyMarkup};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HASHMAP: Arc<Mutex<HashMap<i64, u32>>> = {
        let m = Arc::new(Mutex::new(HashMap::new()));
        m
    };
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let bot = Bot::from_env().auto_send();

    // let db: Arc<Mutex<HashMap<i64, u32>>>  = Arc::new(Mutex::new(HashMap::new()));

    teloxide::repl(bot, |a| async move {
        let buttons = vec![KeyboardButton::new("🚰咚咚咚"), KeyboardButton::new("重置")];
        let keyboard_markup = KeyboardMarkup::new(vec![buttons]);
        let id = a.chat_id();
        if let Some(s) = a.update.text() {
            match s {
                "重置" => {
                    HASHMAP.lock().unwrap().insert(id, 0);
                }
                "🚰咚咚咚" => {
                    let prev = HASHMAP
                        .lock()
                        .unwrap()
                        .get(&id)
                        .map(|b| *b)
                        .unwrap_or_default();
                    HASHMAP.lock().unwrap().insert(id, prev + 1);
                }
                _ => {}
            }
        }

        let count = HASHMAP
            .lock()
            .unwrap()
            .get(&id)
            .map(|b| *b)
            .unwrap_or_default();
        a.answer(format!("{}杯", count))
            .reply_markup(ReplyMarkup::Keyboard(keyboard_markup))
            .await
            .unwrap();

        respond(())
    })
    .await;
}
