use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use telegram_bot::*;

fn main() {
    let mut bot = Bot::from_env().unwrap();

    let update_list = Arc::new(Mutex::new(Vec::new()));

    let update_list_clone = update_list.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        if let Ok(mut updates) = bot.get_updates(None, None) {
            if let Some(update) = updates.pop() {
                update_list_clone.lock().unwrap().push(update);
            }
        }
    });

    loop {
        bot.set_webhook(None).unwrap();
        bot.send(
            "https://api.telegram.org/bot1198290731:AAEQIj0t1Mb-f_BX0h_5C5BETp5p5zHwEQ/",
            None,
        )
        .unwrap();
        bot.set_webhook(Some("https://telegram-bot-rust.herokuapp.com/")).unwrap();
        thread::sleep(Duration::from_secs(3600));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
