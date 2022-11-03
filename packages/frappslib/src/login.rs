use crate::{
	credentials::Credentials,
	error::{from_parser, from_reqwest, GenericError},
};
use chrono::{DateTime, NaiveDate, ParseError};
use futures::Future;
use reqwest::{Client, Response};
use serde::Deserialize;

pub async fn login(credentials: &Credentials) -> Result<Client, GenericError> {
	#[cfg(not(test))]
	let base = "https://resco.flapps.com";

	#[cfg(test)]
	let base = &mockito::server_url();

	let url = format!("{}/login_check", base);
	let client = Client::builder().cookie_store(true).build().unwrap();
	let response = client
		.post(url)
		.send()
		.await
		.map_err(|e| from_reqwest("Unable to get absences", e))?;

	Ok(client)
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockito::mock;
	use tokio_test::assert_ok;

	fn mock_login() -> mockito::Mock {
		mock("POST", "/login_check")
			.match_body("instance=resco&login=anicka.krkvava&password=krkvany.zuzol")
			.with_status(201)
			.with_header("set-cookie", "JSESSIONID=333")
			.create()
	}

	#[tokio::test]
	async fn should_login() {
		let _server = mock_login();

		let actual = login(&&Credentials("asd".to_string(), "adssad".to_string())).await;

		assert_ok!(actual);
	}
}
