use std::mem::MaybeUninit;
use std::ptr::{null};
use ascii::AsciiStr;
use widestring::{U16CStr};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetModuleHandleExA, GetModuleHandleExW, GetModuleHandleW};
use crate::rendering::win32::errhandlingapi::{get_last_error, set_last_error, WIN32Error};
use windows::core::{PCSTR,PCWSTR};
use bitflags::bitflags;

pub struct InstanceHandle {
	pub(crate) value: HINSTANCE
}

pub fn get_module_handle_a(module_name: Option<&AsciiStr>) -> Result<InstanceHandle, windows::core::Error> {
	let module_name_ptr = match module_name {
		None => {
			PCSTR(null())
		}
		Some(string) => {
			PCSTR(string.as_ptr() as *const u8)
		}
	};

	set_last_error(WIN32Error::new_ok());

	let result = unsafe {
		GetModuleHandleA(module_name_ptr)
	};

	result.map(|handle| {
		InstanceHandle {
			value: handle
		}
	})
}

pub fn get_module_handle_w(module_name: Option<&U16CStr>) -> Result<InstanceHandle, windows::core::Error> {
	let module_name_ptr = match module_name {
		None => {
			PCWSTR(null())
		}
		Some(string) => {
			PCWSTR(string.as_ptr())
		}
	};

	set_last_error(WIN32Error::new_ok());

	let result = unsafe {
		GetModuleHandleW(module_name_ptr)
	};

	result.map(|handle| {
		InstanceHandle {
			value: handle
		}
	})
}

pub enum ModuleNameA<'a> {
	None,
	Address(*const u8),
	Name(&'a AsciiStr)
}

pub enum ModuleNameW<'a> {
	None,
	Address(*const u16),
	Name(&'a U16CStr)
}

bitflags! {
	pub struct GetModuleFlags: u32 {
		const Pin = 0x00000001;
		const UnchangedRefcount = 0x00000002;
		const FromAddress = 0x00000004;
	}
}

pub fn get_module_handle_ex_a(
	flags: GetModuleFlags,
	module_name: ModuleNameA
) -> Result<InstanceHandle, windows::core::Error> {
	let flag_bits = flags.bits;

	let module_name_ptr = match module_name {
		ModuleNameA::None => {
			PCSTR(null())
		}
		ModuleNameA::Address(address) => {
			PCSTR(address)
		}
		ModuleNameA::Name(string) => {
			PCSTR(string.as_ptr() as *const u8)
		}
	};

	let mut instance_handle = MaybeUninit::<HINSTANCE>::uninit();

	set_last_error(WIN32Error::new_ok());

	let result = unsafe {
		GetModuleHandleExA(flag_bits, module_name_ptr, instance_handle.as_mut_ptr())
	};

	if result.as_bool() {
		let instance_handle = unsafe {
			instance_handle.assume_init()
		};
		let instance_handle = InstanceHandle {
			value: instance_handle
		};
		return Ok(instance_handle);
	}
	else {
		let error = get_last_error();
		match error.ok() {
			Ok(_) => {
				panic!("Got Ok Error!");
			}
			Err(err) => {
				return Err(err);
			}
		}
	}
}

pub fn get_module_handle_ex_w(
	flags: GetModuleFlags,
	module_name: ModuleNameW
) -> Result<InstanceHandle, windows::core::Error> {
	let flag_bits = flags.bits;

	let module_name_ptr = match module_name {
		ModuleNameW::None => {
			PCWSTR(null())
		}
		ModuleNameW::Address(address) => {
			PCWSTR(address)
		}
		ModuleNameW::Name(string) => {
			PCWSTR(string.as_ptr())
		}
	};

	let mut instance_handle = MaybeUninit::<HINSTANCE>::uninit();

	set_last_error(WIN32Error::new_ok());

	let result = unsafe {
		GetModuleHandleExW(flag_bits, module_name_ptr, instance_handle.as_mut_ptr())
	};

	if result.as_bool() {
		let instance_handle = unsafe {
			instance_handle.assume_init()
		};
		let instance_handle = InstanceHandle {
			value: instance_handle
		};
		Ok(instance_handle)
	}
	else {
		let error = get_last_error();
		match error.ok() {
			Ok(_) => {
				panic!("Got Ok Error!");
			}
			Err(err) => {
				return Err(err);
			}
		}
	}
}