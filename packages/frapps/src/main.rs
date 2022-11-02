use frappslib::login;

#[tokio::main]
async fn main() {
	let res = login("frantisek.ziacik", "kvakykvak100?").await;
	println!("{}", res)
}
