use std::fmt::{Display, Formatter};
use std::mem::{MaybeUninit};
use std::ptr::{copy, null_mut};
use windows::core::GUID;
use windows::Win32::Graphics::Dxgi::{DXGI_DEBUG_ALL, DXGI_INFO_QUEUE_MESSAGE, DXGI_INFO_QUEUE_MESSAGE_CATEGORY, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING, DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE, DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING, DXGIGetDebugInterface1, IDXGIInfoQueue};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DXGIInfoQueueMessageCategory {
	Unknown,
	Miscellaneous,
	Initialization,
	Cleanup,
	Compilation,
	StateCreation,
	StateSetting,
	StateGetting,
	ResourceManipulation,
	Execution,
	Shader
}

impl TryFrom<DXGI_INFO_QUEUE_MESSAGE_CATEGORY> for DXGIInfoQueueMessageCategory {
	type Error = ();

	fn try_from(value: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> Result<Self, Self::Error> {
		if value ==	DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN {
			Ok(Self::Unknown)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS {
			Ok(Self::Miscellaneous)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION {
			Ok(Self::Initialization)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP {
			Ok(Self::Cleanup)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION {
			Ok(Self::Compilation)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION {
			Ok(Self::StateCreation)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING {
			Ok(Self::StateSetting)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING {
			Ok(Self::StateGetting)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION {
			Ok(Self::ResourceManipulation)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION {
			Ok(Self::Execution)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER {
			Ok(Self::Shader)
		}
		else {
			Err(())
		}
	}
}

impl Display for DXGIInfoQueueMessageCategory {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			DXGIInfoQueueMessageCategory::Unknown => {
				f.write_str("UNKNOWN")
			}
			DXGIInfoQueueMessageCategory::Miscellaneous => {
				f.write_str("MISCELLANEOUS")
			}
			DXGIInfoQueueMessageCategory::Initialization => {
				f.write_str("INITIALIZATION")
			}
			DXGIInfoQueueMessageCategory::Cleanup => {
				f.write_str("CLEANUP")
			}
			DXGIInfoQueueMessageCategory::Compilation => {
				f.write_str("COMPILATION")
			}
			DXGIInfoQueueMessageCategory::StateCreation => {
				f.write_str("STATE_CREATION")
			}
			DXGIInfoQueueMessageCategory::StateSetting => {
				f.write_str("STATE_SETTING")
			}
			DXGIInfoQueueMessageCategory::StateGetting => {
				f.write_str("STATE_GETTING")
			}
			DXGIInfoQueueMessageCategory::ResourceManipulation => {
				f.write_str("RESOURCE_MANIPULATION")
			}
			DXGIInfoQueueMessageCategory::Execution => {
				f.write_str("EXECUTION")
			}
			DXGIInfoQueueMessageCategory::Shader => {
				f.write_str("SHADER")
			}
		}
	}
}

pub enum DXGIInfoQueueMessageSeverity {
	Corruption,
	Error,
	Warning,
	Info,
	Message,
}

impl TryFrom<DXGI_INFO_QUEUE_MESSAGE_SEVERITY> for DXGIInfoQueueMessageSeverity {
	type Error = ();

	fn try_from(value: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> Result<Self, ()> {
		if value == DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION {
			Ok(Self::Corruption)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR {
			Ok(Self::Error)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING {
			Ok(Self::Warning)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO {
			Ok(Self::Info)
		} else if value == DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE {
			Ok(Self::Message)
		}
		else {
			Err(())
		}
	}
}

impl Display for DXGIInfoQueueMessageSeverity {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			DXGIInfoQueueMessageSeverity::Corruption => {
				f.write_str("CORRUPTION")
			}
			DXGIInfoQueueMessageSeverity::Error => {
				f.write_str("ERROR")
			}
			DXGIInfoQueueMessageSeverity::Warning => {
				f.write_str("WARNING")
			}
			DXGIInfoQueueMessageSeverity::Info => {
				f.write_str("INFO")
			}
			DXGIInfoQueueMessageSeverity::Message => {
				f.write_str("MESSAGE")
			}
		}
	}
}

pub fn dump_debug_messages() {
	println!("Debug Messages:");
	let messages = get_debug_messages();

	println!("Message Count: {}", messages.len());

	for message in messages {
		println!("{}", message);
	}
}

pub fn get_debug_messages() -> Vec<String> {
	let info_queue: IDXGIInfoQueue = unsafe {
		DXGIGetDebugInterface1::<IDXGIInfoQueue>(0).unwrap()
	};

	let producer = DXGI_DEBUG_ALL;

	let message_count = get_message_count(producer, &info_queue);

	let mut messages = Vec::new();

	for i in 0u64..message_count {
		let message = get_message(producer, i, &info_queue).unwrap();

		let description = String::from_utf8_lossy(&message.description);

		let string = format!("{}:\t{:?}\t{}\t{}\t{:?}", message.severity, message.producer, message.category, message.id, description);
		messages.push(string);
	}
	messages
}

fn get_message_size(producer: GUID, index: u64, info_queue: &IDXGIInfoQueue) -> Result<usize, windows::core::Error> {
	let mut message_size = MaybeUninit::uninit();

	let result = unsafe {
		info_queue.GetMessage(producer, index, null_mut(), message_size.as_mut_ptr())
	};

	match result {
		Ok(_) => {
			Ok(unsafe {
				message_size.assume_init()
			})
		}
		Err(err) => {
			Err(err)
		}
	}
}

unsafe fn get_message_with_size(producer: GUID, index: u64, mut size: usize, info_queue: &IDXGIInfoQueue) -> Result<DXGIInfoQueueMessage, windows::core::Error> {
	// The GetMessage function puts the description string right after the DXGI_INFO_QUEUE_MESSAGE struct.
	// For this reason, size is bigger than sizeof DXGI_INFO_QUEUE_MESSAGE
	// This means that we cannot just use a MaybeUninit<DXGI_INFO_QUEUE_MESSAGE>
	// Allocating the buffer here means we have to copy the whole message in this function, including the string
	let mut message_bytes = vec![0u8; size];

	info_queue.GetMessage(producer, index, message_bytes.as_mut_ptr() as *mut DXGI_INFO_QUEUE_MESSAGE, &mut size as *mut usize)?;

	let message = message_bytes.as_ptr() as *const DXGI_INFO_QUEUE_MESSAGE;
	let message = message.as_ref().unwrap();

	let producer = message.Producer;
	let severity = DXGIInfoQueueMessageSeverity::try_from(message.Severity).unwrap();
	let category = DXGIInfoQueueMessageCategory::try_from(message.Category).unwrap();
	let id = message.ID;

	let mut description = Vec::<u8>::with_capacity(message.DescriptionByteLength);

	copy(message.pDescription, description.as_mut_ptr(), message.DescriptionByteLength);
	description.set_len(message.DescriptionByteLength);

	let message = DXGIInfoQueueMessage {
		producer,
		category,
		severity,
		id,
		description
	};
	
	Ok(message)
}

pub fn get_message(producer: GUID, index: u64, info_queue: &IDXGIInfoQueue) -> Result<DXGIInfoQueueMessage, windows::core::Error> {
	let message_size = get_message_size(producer, index, info_queue)?;

	let message = unsafe {
		get_message_with_size(producer, index, message_size, info_queue)
	};

	let message = message?;
	Ok(message)
}

pub fn get_message_count(producer: GUID, info_queue: &IDXGIInfoQueue) -> u64 {
	unsafe {
		info_queue.GetNumStoredMessagesAllowedByRetrievalFilters(producer)
	}
}

pub struct DXGIInfoQueueMessage {
	pub producer: GUID,
	pub category: DXGIInfoQueueMessageCategory,
	pub severity: DXGIInfoQueueMessageSeverity,
	pub id: i32,
	pub description: Vec<u8>
}