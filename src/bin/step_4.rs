fn main() {
    let args = std::env::args();
    let input = Input::from_args(args);

    let client = reqwest::blocking::Client::new();
    let mut response = client.get(input.url).send().unwrap();
    println!("{}", response.status());

    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}

struct Input {
    request_type: String,
    url: String,
}

impl Input {
    fn from_args(mut args: std::env::Args) -> Input {
        let _program_path = args.next();
        let request_type = args.next().unwrap();
        let url = args.next().unwrap();
        Input { request_type, url }
    }
}
