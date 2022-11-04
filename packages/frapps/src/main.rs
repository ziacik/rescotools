use frappslib::{absences::get_absences, credentials::Credentials, login::login};

#[tokio::main]
async fn main() {
	println!("Wait for it.");

	let client = login(Credentials(
		"resco".to_string(),
		"frantisek.ziacik".to_string(),
		"???".to_string(),
	))
	.await
	.unwrap();

	let result = get_absences(&client).await.unwrap();

	dbg!(result);
}
