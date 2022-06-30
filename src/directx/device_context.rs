use std::ptr::null_mut;
use windows::Win32::Graphics::Direct3D11::{D3D11_VIEWPORT, ID3D11DeviceContext};
use crate::directx::viewport::D3D11Viewport;

pub struct D3D11DeviceContext {
	context: ID3D11DeviceContext
}

impl D3D11DeviceContext {
	pub fn rs_set_viewports(&self, viewports: &[D3D11Viewport]) {
		let viewports = viewports
			.into_iter()
			.map(|item| {
				let item: D3D11_VIEWPORT = (*item).into();
				item
			})
			.collect::<Vec<D3D11_VIEWPORT>>();

		unsafe {
			self.context.RSSetViewports(&viewports);
		}
	}

	pub fn rs_set_viewport(&self, viewport: D3D11Viewport) {
		let viewports = [viewport.into()];

		unsafe {
			self.context.RSSetViewports(&viewports);
		}
	}

	pub fn rs_get_viewports(&self, limit: Option<u32>) -> Vec<D3D11Viewport> {
		let viewport_count_before = limit.unwrap_or_else(|| {
			self.rs_get_viewport_count()
		});

		let mut viewport_count = viewport_count_before;
		let mut viewports = Vec::with_capacity(viewport_count as usize);

		unsafe {
			self.context.RSGetViewports(&mut viewport_count, viewports.as_mut_ptr());
		}

		if viewport_count >= viewport_count_before {
			return viewports.into_iter().map(|item| item.into()).collect();
		}

		viewports.truncate(viewport_count as usize);

		viewports.into_iter().map(|item| item.into()).collect()
	}

	pub fn rs_get_viewport_count(&self) -> u32 {
		let mut num_viewports = 0;
		unsafe {
			self.context.RSGetViewports(&mut num_viewports, null_mut());
		}

		num_viewports
	}
}