use std::env::Args;

use reqwest::blocking::Response;

fn main() {
    // convert our args to the input we expect
    let input: Input = std::env::args().into();
    println!("{:?}", std::env::args());
    println!("Fetching {}...", input.url);

    // create a http client
    let client = reqwest::blocking::Client::new();

    // choose what kind of request to make
    let request = match input.request_type {
        RequestType::Get => client.get(input.url),
        RequestType::Post => client.post(input.url),
    };

    // execute
    let response = request.send().expect("Failed to send request");

    println!(
        "Response status: {:?} {}",
        response.version(),
        response.status()
    );
    println!("Headers: {:#?}\n", response.headers());

    if input.print_body {
        print_body(response);
    }
}

fn print_body(mut response: Response) {
    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}

struct Input {
    request_type: RequestType,
    url: String,
    print_body: bool,
}

impl From<Args> for Input {
    fn from(mut args: Args) -> Self {
        let _program_path = args.next();
        let request_type = args.next().map_or(RequestType::default(), |arg| arg.into());
        let url = args.next().unwrap_or("https://phlcode.club".into());
        let print_body = args.next().map_or(false, |arg| arg == "--print-body");
        Self {
            request_type,
            url,
            print_body,
        }
    }
}

#[derive(Default, Debug)]
enum RequestType {
    #[default]
    Get,
    Post,
}

impl From<String> for RequestType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "get" => Self::Get,
            "post" => Self::Post,
            _ => Self::Get,
        }
    }
}
