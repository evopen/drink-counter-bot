use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |a| async move { respond(()) }).await;
}
