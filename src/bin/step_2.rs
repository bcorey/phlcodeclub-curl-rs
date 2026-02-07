fn main() {
    let mut args = std::env::args();

    let program_path = args.next();
    let request_type = args.next();
    let url = args.next();
    println!("{program_path:?}, {request_type:?}, {url:?}");
}
