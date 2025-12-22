use std::env::Args;

fn main() {
    let input: Input = std::env::args().into();
    println!("Fetching {}...", input.url);
    let client = reqwest::blocking::Client::new();
    let request = match input.request_type {
        RequestType::Get => client.get(input.url),
        RequestType::Post => client.post(input.url),
    };
    let mut response = request.send().expect("Failed to send request");

    println!("Response: {:?} {}", response.version(), response.status());
    println!("Headers: {:#?}\n", response.headers());

    // copy the response body directly to stdout
    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}

struct Input {
    request_type: RequestType,
    url: String,
}

impl From<Args> for Input {
    fn from(mut args: Args) -> Self {
        let request_type = args.next().map_or(RequestType::default(), |arg| arg.into());
        let url = args.next().unwrap_or("https://hyper.rs".into());
        println!("{request_type:?} {url}");
        Self { request_type, url }
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
