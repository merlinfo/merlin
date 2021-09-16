// print a nice error message on an error event

pub const ERROR_PREFIX: &str = "merlin:";

pub fn err_msg<T, E>(res: Result<T, E>, msg: &str) {
	if res.is_err() {
		eprintln!("{} {}", ERROR_PREFIX, msg);
	}
}
