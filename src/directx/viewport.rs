use windows::Win32::Graphics::Direct3D10::D3D10_VIEWPORT;
use windows::Win32::Graphics::Direct3D11::D3D11_VIEWPORT;
use windows::Win32::Graphics::Direct3D12::D3D12_VIEWPORT;
use serde_derive::{Serialize, Deserialize};

#[derive(Copy, Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct D3D10Viewport {
	top_left_x: i32,
	top_left_y: i32,
	width: u32,
	height: u32,
	min_depth: f32,
	max_depth: f32
}

impl Into<D3D10_VIEWPORT> for D3D10Viewport {
	fn into(self) -> D3D10_VIEWPORT {
		D3D10_VIEWPORT {
			TopLeftX: self.top_left_x,
			TopLeftY: self.top_left_y,
			Width: self.width,
			Height: self.height,
			MinDepth: self.min_depth,
			MaxDepth: self.max_depth
		}
	}
}

impl From<D3D10_VIEWPORT> for D3D10Viewport {
	fn from(item: D3D10_VIEWPORT) -> Self {
		Self {
			top_left_x: item.TopLeftX,
			top_left_y: item.TopLeftY,
			width: item.Width,
			height: item.Height,
			min_depth: item.MinDepth,
			max_depth: item.MaxDepth
		}
	}
}

#[derive(Copy, Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct D3D11Viewport {
	top_left_x: f32,
	top_left_y: f32,
	width: f32,
	height: f32,
	min_depth: f32,
	max_depth: f32
}

impl Into<D3D11_VIEWPORT> for D3D11Viewport {
	fn into(self) -> D3D11_VIEWPORT {
		D3D11_VIEWPORT {
			TopLeftX: self.top_left_x,
			TopLeftY: self.top_left_y,
			Width: self.width,
			Height: self.height,
			MinDepth: self.min_depth,
			MaxDepth: self.max_depth
		}
	}
}

impl From<D3D11_VIEWPORT> for D3D11Viewport {
	fn from(item: D3D11_VIEWPORT) -> Self {
		Self {
			top_left_x: item.TopLeftX,
			top_left_y: item.TopLeftY,
			width: item.Width,
			height: item.Height,
			min_depth: item.MinDepth,
			max_depth: item.MaxDepth
		}
	}
}

#[derive(Copy, Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct D3D12Viewport {
	top_left_x: f32,
	top_left_y: f32,
	width: f32,
	height: f32,
	min_depth: f32,
	max_depth: f32
}

impl Into<D3D12_VIEWPORT> for D3D12Viewport {
	fn into(self) -> D3D12_VIEWPORT {
		D3D12_VIEWPORT {
			TopLeftX: self.top_left_x,
			TopLeftY: self.top_left_y,
			Width: self.width,
			Height: self.height,
			MinDepth: self.min_depth,
			MaxDepth: self.max_depth
		}
	}
}

impl From<D3D12_VIEWPORT> for D3D12Viewport {
	fn from(item: D3D12_VIEWPORT) -> Self {
		Self {
			top_left_x: item.TopLeftX,
			top_left_y: item.TopLeftY,
			width: item.Width,
			height: item.Height,
			min_depth: item.MinDepth,
			max_depth: item.MaxDepth
		}
	}
}