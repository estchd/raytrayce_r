use std::mem::MaybeUninit;
use std::ptr::{copy, null_mut};
use windows::core::GUID;
use windows::Win32::Graphics::Dxgi::{DXGI_INFO_QUEUE_MESSAGE, IDXGIInfoQueue};
use crate::rendering::win32::dxgidebug::{DXGIInfoQueueMessage, DXGIInfoQueueMessageCategory, DXGIInfoQueueMessageSeverity};

pub struct DXGIInfoQueue {
	info_queue: IDXGIInfoQueue
}

impl DXGIInfoQueue {
	pub fn clear_stored_messages(&self, producer: Option<GUID>) {
		let producer = producer.map(|item| item.into());

		unsafe {
			self.info_queue.ClearStoredMessages(producer);
		}
	}

	pub fn set_message_count_limit(&self, limit: u64, producer: Option<GUID>) -> windows::core::Result<()> {
		let producer = producer.map(|item| item.into());

		unsafe {
			self.info_queue.SetMessageCountLimit(producer, limit)
		}
	}

	pub fn get_message(&self, producer: Option<GUID>, index: u64) -> windows::core::Result<DXGIInfoQueueMessage> {
		let message_size = self.get_message_size(producer, index)?;

		let message = unsafe {
			self.get_message_with_size(producer, index, message_size)
		}?;

		Ok(message)
	}

	fn get_message_size(&self, producer: Option<GUID>, index: u64) -> windows::core::Result<usize> {
		let mut message_size = MaybeUninit::uninit();

		let result = unsafe {
			self.info_queue.GetMessage(producer, index, null_mut(), message_size.as_mut_ptr())
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

	unsafe fn get_message_with_size(&self, producer: Option<GUID>, index: u64, mut size: usize) -> windows::core::Result<DXGIInfoQueueMessage> {
		// The GetMessage function puts the description string right after the DXGI_INFO_QUEUE_MESSAGE struct.
		// For this reason, size is bigger than sizeof DXGI_INFO_QUEUE_MESSAGE
		// This means that we cannot just use a MaybeUninit<DXGI_INFO_QUEUE_MESSAGE>
		// Allocating the buffer here means we have to copy the whole message in this function, including the string
		let mut message_bytes = vec![0u8; size];

		self.info_queue.GetMessage(producer, index, message_bytes.as_mut_ptr() as *mut DXGI_INFO_QUEUE_MESSAGE, &mut size as *mut usize)?;

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

	pub fn get_num_stored_messages_allowed_by_retrieval_filters(&self, producer: Option<GUID>) -> u64 {
		unsafe {
			self.info_queue.GetNumStoredMessagesAllowedByRetrievalFilters(producer)
		}
	}

	pub fn get_num_stored_messages() {

	}

	pub fn get_num_messages_discarded_by_message_count_limit() {

	}

	pub fn get_message_count_limit() {

	}

	pub fn get_num_messages_allowed_by_storage_filter() {

	}

	pub fn get_num_messages_denied_by_storage_filter() {

	}

	pub fn add_message() {

	}

	pub fn add_application_message() {

	}

	pub fn set_break_on_category() {

	}

	pub fn set_break_on_severity() {

	}

	pub fn set_break_on_id() {

	}

	pub fn get_break_on_category() {

	}

	pub fn get_break_on_severity() {

	}

	pub fn get_break_on_id() {

	}

	pub fn set_mute_debug_output() {

	}

	pub fn get_mute_debug_output() {

	}
}

pub struct DXGIInfoQueueStorageFilters {
	info_queue: IDXGIInfoQueue
}

impl DXGIInfoQueueStorageFilters {
	pub fn add_storage_filter_entries() {

	}

	pub fn get_storage_filter() {

	}

	pub fn clear_storage_filter() {

	}

	pub fn push_empty_storage_filter() {

	}

	pub fn push_deny_all_storage_filter() {

	}

	pub fn push_copy_of_storage_filter() {

	}

	pub fn push_storage_filter() {

	}

	pub fn pop_storage_filter() {

	}

	pub fn get_storage_filter_stack_size() {

	}
}

pub struct DXGIInfoQueueRetrievalFilters {
	info_queue: IDXGIInfoQueue
}

impl DXGIInfoQueueRetrievalFilters {
	pub fn add_retrieval_filter_entries() {

	}

	pub fn get_retrieval_filter() {

	}

	pub fn clear_retrieval_filter() {

	}

	pub fn push_empty_retrieval_filter() {

	}

	pub fn push_deny_all_retrieval_filter() {

	}

	pub fn push_copy_of_retrieval_filter() {

	}

	pub fn push_retrieval_filter() {

	}

	pub fn pop_retrieval_filter() {

	}

	pub fn get_retrieval_filter_stack_size() {

	}
}