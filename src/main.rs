use educlient::Educlient;
// use serde_json;

fn main() {
    // argv
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <username> <password> <domain>", args[0]);
        return;
    }
    let username = args[1].clone();
    let password = args[2].clone();
    let domain = args[3].clone();

    // login
    let mut client = Educlient::new(username, password, domain);
    client.login().unwrap();
    println!("\n\n\n\n\n\n\n\n\n\n\nRozdelovac\n{}", client.data);
}