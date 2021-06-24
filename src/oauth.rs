extern crate keyring;

use egg_mode::auth::Token;
use std::env;
use std::io::Write;

pub async fn get_token() -> Token {
    let service_public = "crow-public";
    let service_private = "crow-private";
    let username = env::var("USER").unwrap();

    let keyring_public = keyring::Keyring::new(&service_public, &username);
    let keyring_private = keyring::Keyring::new(&service_private, &username);

    let api_key = match env::var("API") {
        Ok(key) => key,
        Err(_) => panic!("API key not found"),
    };

    let api_secret_key = match env::var("API_SECRET") {
        Ok(key) => key,
        Err(_) => panic!("API_SECRET key not found"),
    };

    let con_token = egg_mode::KeyPair::new(api_key, api_secret_key);

    match keyring_public.get_password() {
        Ok(public) => {
            let private = keyring_private.get_password().unwrap();
            let access_token = egg_mode::KeyPair::new(public, private);
            egg_mode::Token::Access {
                consumer: con_token,
                access: access_token,
            }
        },
        Err(_) => {
            // "oob" is needed for PIN-based auth; see docs for `request_token` for more info
            let request_token = egg_mode::auth::request_token(&con_token, "oob").await.unwrap();
            let auth_url = egg_mode::auth::authorize_url(&request_token);
        
            // give auth_url to the user, they can sign in to Twitter and accept your app's permissions.
            // they'll receive a PIN in return, they need to give this to your application
        
            let mut verifier = String::new();
            println!("Browse to: {} ", auth_url);
            print!("Enter PIN: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut verifier).unwrap();
        
            // note this consumes con_token; if you want to sign in multiple accounts, clone it here
            let (token, _user_id, _screen_name) = egg_mode::auth::access_token(con_token, &request_token, verifier).await.unwrap();

            if let Token::Access {consumer: _, ref access} = token {
                keyring_public.set_password(&access.key).unwrap();
                keyring_private.set_password(&access.secret).unwrap();
            }

            // token can be given to any egg_mode method that asks for a token
            // user_id and screen_name refer to the user who signed in        
            token
        }
    }
}