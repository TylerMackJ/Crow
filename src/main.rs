use std::env;

struct Flags {
    text: Option<String>,
    photo: Option<String>,
}

fn main() {
    //parse_args(env::args().collect());

    match env::var("API") {
        Ok(val) => println!("{:?}", val),
        Err(e) => println!("{}", e),
    }
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
            p if args.len() > 0 && p == "-p" => ret.text = Some(args.remove(0)),
            // Handle text
            text if ret.text == None => ret.text = Some(text),
            // Panic
            a => panic!("Unhandled Argument: {}", a),
        }
    }

    ret
}