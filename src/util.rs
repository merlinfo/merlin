// print a nice error message on an error event

pub fn err_msg<T, E>(res: Result<T, E>, msg: &str) {
	if let Err(_) = res {
		eprintln!("merlin: {}", msg);
	}
}
