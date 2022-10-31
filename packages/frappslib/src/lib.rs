use reqwest::Client;

pub async fn login(user: &str, pass: &str) -> String {
	let client = Client::new();
	let r = client
		.get("https://resco.flapps.com/login.jsp")
		.send()
		.await;
	r.unwrap().text().await.unwrap()
}

pub async fn get_login_cookie() -> String {
	#[cfg(not(test))]
	let url = "https://resco.flapps.com";
	#[cfg(test)]
	let url = &mockito::server_url();
	let text = reqwest::get(url).await.unwrap().text().await.unwrap();
	text
}

#[cfg(test)]
mod tests {
	use mockito::mock;

	use super::*;

	macro_rules! aw {
		($e:expr) => {
			tokio_test::block_on($e)
		};
	}

	#[test]
	fn it_works() {
		let _m = mock("GET", "/")
			.with_status(201)
			.with_header("content-type", "text/plain")
			.with_header("x-api-key", "1234")
			.with_header("set-cookie", "JSESSIONID:12345")
			.with_body("12345")
			.create();

		assert_eq!(aw!(get_login_cookie()), "12345".to_string());
	}
}
