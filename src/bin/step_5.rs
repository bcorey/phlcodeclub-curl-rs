fn main() {
    let args = std::env::args();
    let input = Input::from_args(args);

    let client = reqwest::blocking::Client::new();

    let mut response = match input.request_type {
        RequestType::Get => client.get(input.url).send().unwrap(),
        RequestType::Post => client.post(input.url).send().unwrap(),
    };

    println!("{}", response.status());

    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}

enum RequestType {
    Get,
    Post,
}

impl RequestType {
    fn from_string(arg: String) -> RequestType {
        if arg == "post" {
            return RequestType::Post;
        }
        return RequestType::Get;
    }
}

struct Input {
    request_type: RequestType,
    url: String,
}

impl Input {
    fn from_args(mut args: std::env::Args) -> Input {
        let _program_path = args.next();
        let request_type = RequestType::from_string(args.next().unwrap());
        let url = args.next().unwrap();
        Input { request_type, url }
    }
}
