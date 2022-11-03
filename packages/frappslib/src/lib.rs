pub mod absences;
pub mod credentials;
pub mod error;
pub mod login;

// use reqwest::{redirect::Policy, Client};

// pub async fn login(user: &str, pass: &str) -> String {
// 	let client = Client::builder()
// 		.redirect(Policy::default())
// 		.cookie_store(true)
// 		.build()
// 		.unwrap();

// 	let params = [("instance", "resco"), ("login", user), ("password", pass)];
// 	let r = client
// 		.post("https://resco.flapps.com/login_check")
// 		.form(&params)
// 		// .header("Content-Type", "application/x-www-form-urlencoded")
// 		// .body("instance=resco&login=frantisek.ziacik&password=kvakykvak100?")
// 		.send()
// 		.await;

// 	let r = client
// 		.get("https://resco.flapps.com/rest/dashboard/absences")
// 		// .header("Content-Type", "application/x-www-form-urlencoded")
// 		// .body("instance=resco&login=frantisek.ziacik&password=kvakykvak100?")
// 		.send()
// 		.await;
// 	r.unwrap().text().await.unwrap()

// 	// r.unwrap()
// 	// 	.headers()
// 	// 	.get("location")
// 	// 	.unwrap()
// 	// 	.to_str()
// 	// 	.unwrap()
// 	// 	.to_string()
// }

// pub async fn get_login_cookie() -> String {
// 	#[cfg(not(test))]
// 	let url = "https://resco.flapps.com";
// 	#[cfg(test)]
// 	let url = &mockito::server_url();
// 	let text = reqwest::get(url).await.unwrap().text().await.unwrap();
// 	text
// }

// #[cfg(test)]
// mod tests {
// 	use mockito::mock;

// 	use super::*;

// 	macro_rules! aw {
// 		($e:expr) => {
// 			tokio_test::block_on($e)
// 		};
// 	}

// 	#[test]
// 	fn it_works() {
// 		let _m = mock("GET", "/")
// 			.with_status(201)
// 			.with_header("content-type", "text/plain")
// 			.with_header("x-api-key", "1234")
// 			.with_header("set-cookie", "JSESSIONID:12345")
// 			.with_body("12345")
// 			.create();

// 		assert_eq!(aw!(get_login_cookie()), "12345".to_string());
// 	}
// }
