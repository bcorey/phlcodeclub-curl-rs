fn main() {
    let url = std::env::args().nth(1).unwrap_or("https://hyper.rs".into());

    println!("Fetching {url}...");
    let mut res = reqwest::blocking::get(url).expect("Request failed.");

    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())
        .expect("Failed to print response body");
}
