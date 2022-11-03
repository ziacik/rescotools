use futures::{future::FutureExt, Future, TryFutureExt};
use reqwest::{Client, Error, Response};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Presence {
	fullName: String,
	r#type: String,
	until: String,
}

async fn kvak() {
	let future = async { 1 };
	let new_future = future.map(|x| x + 3);
	assert_eq!(new_future.await, 4);
}

async fn parse_response(response: Response) -> impl Future<Output = Result<Vec<Presence>, Error>> {
	response.json::<Vec<Presence>>()
}

pub async fn get_presence(logged_in_client: &Client) -> Vec<Presence> {
	#[cfg(not(test))]
	let base = "http://example.com";

	// The host to be used in test compilation
	#[cfg(test)]
	let base = &mockito::server_url();
	let url = format!("{}/rest/dashboard/absences", base);

	let r = logged_in_client.get(url).send();
	let m = r.map(|it| parse_response(it.unwrap()));

	// let result: Vec<Presence> = r
	// 	.map(|response_or_error| response_or_error.unwrap().json::<Vec<Presence>>())
	// 	.then(|it| {
	// 		let x = it.unwrap_or_else(|_| Vec::new::Presence());
	// 		Presence {
	// 			fullName: it.unwrap_or_else(|_| "xxx").fullName,
	// 			r#type: it.r#type,
	// 			until: it.until,
	// 		}
	// 	})
	// 	.await
	// 	.await
	// 	.unwrap();

	// result

	m
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
		let _m = mock("GET", "/rest/dashboard/absences")
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
			fullName: String::from("Ján Mrkva"),
			r#type: String::from("Dovolenka"),
			until: String::from("2022-11-01T00:00:00.000Z"), //NaiveDate::from_ymd(2022, 11, 1),
		};

		let expected2 = Presence {
			fullName: String::from("Filoména Krkvavá"),
			r#type: String::from("Práca z domu"),
			until: String::from("2022-12-05T00:00:00.000Z"), // NaiveDate::from_ymd(2022, 12, 5),
		};

		let client = Client::new();
		let actual = block_on(get_presence(&client));

		assert_eq!(&actual, &[expected1, expected2]);
	}
}
