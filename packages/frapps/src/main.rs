use frappslib::{absences::get_absences, credentials::Credentials, login::login};

#[tokio::main]
async fn main() {
	println!("Wait for it.");

	let client = login(&Credentials("adsf".to_string(), "asdf".to_string()))
		.await
		.unwrap();

	let result = get_absences(&client).await.unwrap();

	dbg!(result);
}
