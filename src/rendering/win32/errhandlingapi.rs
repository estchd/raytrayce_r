use windows::Win32::Foundation::{GetLastError, SetLastError, WIN32_ERROR};
use windows::core::Result;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct WIN32Error {
	value: WIN32_ERROR
}

impl WIN32Error {
	pub fn is_ok(&self) -> bool {
		self.value.is_ok()
	}

	pub fn is_err(&self) -> bool {
		self.value.is_err()
	}

	pub fn ok(&self) -> Result<()> {
		self.value.ok()
	}

	pub fn new_ok() -> Self {
		Self {
			value: WIN32_ERROR(0)
		}
	}
}

pub fn get_last_error() -> WIN32Error{
	let value = unsafe {
		GetLastError()
	};
	WIN32Error {
		value
	}
}

pub fn set_last_error(value: WIN32Error) {
	let value = value.value;
	unsafe {
		SetLastError(value);
	}
}