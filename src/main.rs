use std::time::Duration;

use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode}
};
use terminal_colorsaurus::{color_scheme, QueryOptions};
use futures_util::StreamExt;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    enable_raw_mode().unwrap();

    let mut stream = EventStream::new();

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            println!("{:?}\r", color_scheme(QueryOptions::default()));
        }
    });

    loop {
        tokio::select! {
            event = stream.next() => {
                eprintln!("{:?}\r", event);

                if let Some(Ok(Event::Key(
                    KeyEvent { code: KeyCode::Char('c'), .. }
                ))) = event {
                    break;
                }
            }
        }
    }

    disable_raw_mode().unwrap();
}

