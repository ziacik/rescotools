use chrono::NaiveDate;
use futures::{future::FutureExt, TryFutureExt};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Presence {
	name: String,
	typ: String,
	until: String,
}

pub async fn get_presence(logged_in_client: &Client) -> Vec<Presence> {
	#[cfg(not(test))]
	let host = "http://example.com";

	// The host to be used in test compilation
	#[cfg(test)]
	let host = &mockito::server_url();

	let x = logged_in_client
		.get(format!("{}/rest/dashboard/absences", host))
		.send()
		.await
		.expect("asdfsdaf")
		.text()
		.await;

	dbg!(x);

	let result: Vec<Presence> = logged_in_client
		.get(format!("{}/rest/dashboard/absences", host))
		.send()
		.map(|resp| {
			resp.unwrap()
				.json::<Vec<Presence>>()
				.unwrap_or_else(|_| Vec::new())
		})
		.await
		.await;

	result
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
	fn mock_server() {
		let host = &mockito::server_url();

		let _m = mock("GET", format!("{}/rest/dashboard/absences", host).as_str())
			.with_status(200)
			.with_header("content-type", "application/json")
			.with_body(
				r#"
				[
					{
						"fullName": "Ján Mrkva",
						"type": "Dovolenka",
						"until": "2022-11-01T00:00:00.000Z",
						"photoImgSrc": "..."
					},
					{
						"fullName": "Filoména Krkvavá",
						"type": "Práca z domu",
						"until": "2022-12-05T00:00:00.000Z",
						"photoImgSrc": "..."
					}
				]"#,
			)
			.create();

		let expected1 = Presence {
			name: String::from("Ján Mrkva"),
			typ: String::from("Dovolenka"),
			until: String::from("2022-11-01T00:00:00.000Z"), //NaiveDate::from_ymd(2022, 11, 1),
		};

		let expected2 = Presence {
			name: String::from("Filoména Krkvavá"),
			typ: String::from("Práca z domu"),
			until: String::from("2022-12-05T00:00:00.000Z"), // NaiveDate::from_ymd(2022, 12, 5),
		};

		let client = Client::new();

		let actual = aw!(get_presence(&client));

		assert_eq!(&actual, &[expected1, expected2]);
	}
}
