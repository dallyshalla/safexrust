extern crate rustc_serialize;
use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};
use std::str::FromStr;


#[derive(RustcDecodable)]
pub struct Person {
	username: String,
	password: String,
	coinbalance: i32,
}

impl ToJson for Person {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		d.insert("username".to_string(), self.username.to_json());
		d.insert("password".to_string(), self.password.to_json());
		d.insert("coinbalance".to_string(), self.coinbalance.to_json());
		Json::Object(d)
	}
}
struct item;
impl item {
	
}

struct vault;
impl vault {

}

struct orderbook;
impl orderbook {

}



fn main() {
	let one = Person {
		username: "dally".to_string(),
		password: "shalla".to_string(),
		coinbalance: 32,
	};
	let json_obj: Json = one.to_json();
	let json_str: String = json_obj.to_string();
	println!("{}", json_str);
}
