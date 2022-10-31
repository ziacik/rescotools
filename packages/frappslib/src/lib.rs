pub fn frappslib() -> String {
	"frappslib".into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(frappslib(), "frappslib".to_string());
	}
}
