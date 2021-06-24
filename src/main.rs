#[cfg(test)]
mod tests;
mod oauth;

use egg_mode::auth::Token;
use egg_mode::media::{upload_media, media_types};
use egg_mode::tweet::DraftTweet;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Flags {
    text: Option<String>,
    photo: Option<String>,
}

#[tokio::main]
async fn main() {
    let tweet_data: Flags = parse_args(env::args().collect());

    if tweet_data.text != None || tweet_data.photo != None {
        let token = oauth::get_token().await;
        match draft_tweet(tweet_data, &token).await {
            Ok(tweet) => {
                match tweet.send(&token).await {
                    Ok(_) => {},
                    Err(_) => panic!("Error sending tweet!"),
                }
            },
            Err(s) => panic!("{}", s),
        };
    }
    return
}

fn parse_args(mut args: Vec<String>) -> Flags {
    let mut ret: Flags = Flags {
        text: None,
        photo: None,
    };

    // Remove binary 
    args.remove(0);

    // Remove and parse options
    while args.len() > 0 {
        match args.remove(0) {
            // Handle -p
            p if args.len() > 0 && p == "-p" => ret.photo = Some(args.remove(0)),
            // Handle text
            text if ret.text == None => ret.text = Some(text),
            // Panic
            a => panic!("Unhandled Argument: {}", a),
        }
    }

    ret
}

async fn draft_tweet(tweet_data: Flags, token: &Token) -> Result<DraftTweet, String> {
    let mut tweet: DraftTweet;

    match tweet_data.text {
        Some(text) => tweet = DraftTweet::new(text),
        None => tweet = DraftTweet::new("".to_string()),
    }

    match tweet_data.photo {
        Some(photo) => {
            let mut img_file = File::open(photo).unwrap();
            let mut img_bytes = Vec::new();
            img_file.read_to_end(&mut img_bytes).unwrap();
            let media = upload_media(&img_bytes, &media_types::image_png(), &token).await.unwrap();
            tweet.add_media(media.id);
        },
        None => {},
    }
    Ok(tweet)
}