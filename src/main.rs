use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let input = "PING2";
    if input == "PING" {
        println!("{}", ping())
    }

    rpc_call()
}

fn ping() -> &'static str {
    return "PONG";
}
fn rpc_call() {
    let data = "{\"name\":\"Marco\",\"age\":\"30\"}";
    let data_bytes = data.as_bytes();

    println!("{:?}", data_bytes);

    facade(data_bytes, 1000)
}

fn facade(data: &[u8], ttl: i32) {

    // let id: uuid::Uuid::new_v4();

    let mut hm: HashMap<String, &[u8]> = HashMap::new();
    hm.insert(String::from("1a"), data);

    let x = hm;
}

fn service() {

}

