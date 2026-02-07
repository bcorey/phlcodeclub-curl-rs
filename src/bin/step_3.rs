fn main() {
    let mut args = std::env::args();

    let program_path = args.next().unwrap();
    let request_type = args.next().unwrap();
    let url = args.next().unwrap();
    println!("{program_path:?}, {request_type:?}, {url:?}");

    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).send().unwrap();
    println!("{}", response.status());

    response
        .copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}
