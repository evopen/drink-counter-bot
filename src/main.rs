use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tbot::contexts::fields::Message;
use tbot::contexts::methods::ChatMethods;
use tbot::proxy::{Intercept, Proxy};

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
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("starting");
    dotenv::dotenv().ok();

    let bot = match std::env::var("HTTPS_PROXY") {
        Ok(proxy) => {
            let proxy = Proxy::new(Intercept::All, proxy.parse().unwrap());
            tbot::Bot::with_proxy(std::env::var("TOKEN").unwrap(), proxy)
        }
        Err(_) => tbot::Bot::new(std::env::var("TOKEN").unwrap()),
    };
    let mut bot = bot.event_loop();
    println!("qqaa");

    bot.text(|context| async move {
        let id = context.chat().id.0;
        match context.text.value.as_str() {
            "重置" => {
                HASHMAP.lock().unwrap().insert(id, 0);
            }
            "🚰吨吨吨" => {
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

        let count = HASHMAP
            .lock()
            .unwrap()
            .get(&id)
            .map(|b| *b)
            .unwrap_or_default();
        context
            .send_message_in_reply(&format!("{}杯", count))
            .call()
            .await
            .unwrap();
    });

    bot.polling().start().await.unwrap();
}
