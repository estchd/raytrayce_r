#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::mem::{MaybeUninit, size_of};
use std::ptr::null;
use bitflags::bitflags;
use widestring::U16CString;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Direct3D11::{D3D11_BIND_INDEX_BUFFER, D3D11_BIND_SHADER_RESOURCE, D3D11_BIND_VERTEX_BUFFER, D3D11_BUFFER_DESC, D3D11_CPU_ACCESS_WRITE, D3D11_CREATE_DEVICE_DEBUG, D3D11_CREATE_DEVICE_FLAG, D3D11_INPUT_ELEMENT_DESC, D3D11_MAP_WRITE_DISCARD, D3D11_MAPPED_SUBRESOURCE, D3D11_RESOURCE_MISC_FLAG, D3D11_SDK_VERSION, D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT, D3D11_USAGE_DYNAMIC, D3D11_VIEWPORT, D3D11CreateDevice, D3D11CreateDeviceAndSwapChain, ID3D11Buffer, ID3D11Device, ID3D11DeviceContext, ID3D11InputLayout, ID3D11PixelShader, ID3D11RenderTargetView, ID3D11Resource, ID3D11ShaderResourceView, ID3D11Texture2D, ID3D11VertexShader};
use windows::Win32::Graphics::Direct3D::{D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST, D3D11_SRV_DIMENSION_TEXTURE2D, D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_NULL, D3D_DRIVER_TYPE_REFERENCE, D3D_DRIVER_TYPE_SOFTWARE, D3D_DRIVER_TYPE_UNKNOWN, D3D_DRIVER_TYPE_WARP, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2, D3D_FEATURE_LEVEL_9_3};
use windows::Win32::Graphics::Dxgi::{DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT, DXGI_SWAP_EFFECT_DISCARD, DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL, DXGI_SWAP_EFFECT_SEQUENTIAL, IDXGISwapChain};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_420_OPAQUE, DXGI_FORMAT_A8_UNORM, DXGI_FORMAT_A8P8, DXGI_FORMAT_AI44, DXGI_FORMAT_AYUV, DXGI_FORMAT_B4G4R4A4_UNORM, DXGI_FORMAT_B5G5R5A1_UNORM, DXGI_FORMAT_B5G6R5_UNORM, DXGI_FORMAT_B8G8R8A8_TYPELESS, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_B8G8R8A8_UNORM_SRGB, DXGI_FORMAT_B8G8R8X8_TYPELESS, DXGI_FORMAT_B8G8R8X8_UNORM, DXGI_FORMAT_B8G8R8X8_UNORM_SRGB, DXGI_FORMAT_BC1_TYPELESS, DXGI_FORMAT_BC1_UNORM, DXGI_FORMAT_BC1_UNORM_SRGB, DXGI_FORMAT_BC2_TYPELESS, DXGI_FORMAT_BC2_UNORM, DXGI_FORMAT_BC2_UNORM_SRGB, DXGI_FORMAT_BC3_TYPELESS, DXGI_FORMAT_BC3_UNORM, DXGI_FORMAT_BC3_UNORM_SRGB, DXGI_FORMAT_BC4_SNORM, DXGI_FORMAT_BC4_TYPELESS, DXGI_FORMAT_BC4_UNORM, DXGI_FORMAT_BC5_SNORM, DXGI_FORMAT_BC5_TYPELESS, DXGI_FORMAT_BC5_UNORM, DXGI_FORMAT_BC6H_SF16, DXGI_FORMAT_BC6H_TYPELESS, DXGI_FORMAT_BC6H_UF16, DXGI_FORMAT_BC7_TYPELESS, DXGI_FORMAT_BC7_UNORM, DXGI_FORMAT_BC7_UNORM_SRGB, DXGI_FORMAT_D16_UNORM, DXGI_FORMAT_D24_UNORM_S8_UINT, DXGI_FORMAT_D32_FLOAT, DXGI_FORMAT_D32_FLOAT_S8X24_UINT, DXGI_FORMAT_FORCE_UINT, DXGI_FORMAT_G8R8_G8B8_UNORM, DXGI_FORMAT_IA44, DXGI_FORMAT_NV11, DXGI_FORMAT_NV12, DXGI_FORMAT_P010, DXGI_FORMAT_P016, DXGI_FORMAT_P208, DXGI_FORMAT_P8, DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM, DXGI_FORMAT_R10G10B10A2_TYPELESS, DXGI_FORMAT_R10G10B10A2_UINT, DXGI_FORMAT_R10G10B10A2_UNORM, DXGI_FORMAT_R11G11B10_FLOAT, DXGI_FORMAT_R16_FLOAT, DXGI_FORMAT_R16_SINT, DXGI_FORMAT_R16_SNORM, DXGI_FORMAT_R16_TYPELESS, DXGI_FORMAT_R16_UINT, DXGI_FORMAT_R16_UNORM, DXGI_FORMAT_R16G16_FLOAT, DXGI_FORMAT_R16G16_SINT, DXGI_FORMAT_R16G16_SNORM, DXGI_FORMAT_R16G16_TYPELESS, DXGI_FORMAT_R16G16_UINT, DXGI_FORMAT_R16G16_UNORM, DXGI_FORMAT_R16G16B16A16_FLOAT, DXGI_FORMAT_R16G16B16A16_SINT, DXGI_FORMAT_R16G16B16A16_SNORM, DXGI_FORMAT_R16G16B16A16_TYPELESS, DXGI_FORMAT_R16G16B16A16_UINT, DXGI_FORMAT_R16G16B16A16_UNORM, DXGI_FORMAT_R1_UNORM, DXGI_FORMAT_R24_UNORM_X8_TYPELESS, DXGI_FORMAT_R24G8_TYPELESS, DXGI_FORMAT_R32_FLOAT, DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS, DXGI_FORMAT_R32_SINT, DXGI_FORMAT_R32_TYPELESS, DXGI_FORMAT_R32_UINT, DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32_SINT, DXGI_FORMAT_R32G32_TYPELESS, DXGI_FORMAT_R32G32_UINT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32_SINT, DXGI_FORMAT_R32G32B32_TYPELESS, DXGI_FORMAT_R32G32B32_UINT, DXGI_FORMAT_R32G32B32A32_FLOAT, DXGI_FORMAT_R32G32B32A32_SINT, DXGI_FORMAT_R32G32B32A32_TYPELESS, DXGI_FORMAT_R32G32B32A32_UINT, DXGI_FORMAT_R32G8X24_TYPELESS, DXGI_FORMAT_R8_SINT, DXGI_FORMAT_R8_SNORM, DXGI_FORMAT_R8_TYPELESS, DXGI_FORMAT_R8_UINT, DXGI_FORMAT_R8_UNORM, DXGI_FORMAT_R8G8_B8G8_UNORM, DXGI_FORMAT_R8G8_SINT, DXGI_FORMAT_R8G8_SNORM, DXGI_FORMAT_R8G8_TYPELESS, DXGI_FORMAT_R8G8_UINT, DXGI_FORMAT_R8G8_UNORM, DXGI_FORMAT_R8G8B8A8_SINT, DXGI_FORMAT_R8G8B8A8_SNORM, DXGI_FORMAT_R8G8B8A8_TYPELESS, DXGI_FORMAT_R8G8B8A8_UINT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R8G8B8A8_UNORM_SRGB, DXGI_FORMAT_R9G9B9E5_SHAREDEXP, DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE, DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE, DXGI_FORMAT_UNKNOWN, DXGI_FORMAT_V208, DXGI_FORMAT_V408, DXGI_FORMAT_X24_TYPELESS_G8_UINT, DXGI_FORMAT_X32_TYPELESS_G8X24_UINT, DXGI_FORMAT_Y210, DXGI_FORMAT_Y216, DXGI_FORMAT_Y410, DXGI_FORMAT_Y416, DXGI_FORMAT_YUY2, DXGI_MODE_DESC, DXGI_MODE_SCALING, DXGI_MODE_SCALING_CENTERED, DXGI_MODE_SCALING_STRETCHED, DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER, DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST, DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST, DXGI_RATIONAL, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_CPU_ACCESS_DYNAMIC, DXGI_CPU_ACCESS_FIELD, DXGI_CPU_ACCESS_NONE, DXGI_CPU_ACCESS_READ_WRITE, DXGI_CPU_ACCESS_SCRATCH};
use windows::Win32::Graphics::Dxgi::{DXGI_USAGE_BACK_BUFFER, DXGI_USAGE_DISCARD_ON_PRESENT, DXGI_USAGE_READ_ONLY, DXGI_USAGE_RENDER_TARGET_OUTPUT, DXGI_USAGE_SHADER_INPUT, DXGI_USAGE_SHARED, DXGI_USAGE_UNORDERED_ACCESS};
use windows::Win32::Graphics::Dxgi::{DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING, DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY, DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER, DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT, DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO, DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE, DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED, DXGI_SWAP_CHAIN_FLAG_NONPREROTATED, DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER, DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT, DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS, DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO};
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::UI::WindowsAndMessaging::{AdjustWindowRectEx, CreateWindowExW, DefWindowProcW, HCURSOR, HICON, PostQuitMessage, RegisterClassExW, SHOW_WINDOW_CMD, ShowWindow, WINDOW_EX_STYLE, WINDOW_STYLE, WM_DESTROY, WNDCLASS_STYLES, WNDCLASSEXW};
use crate::rendering::win32::dxgidebug::dump_debug_messages;
use crate::rendering::win32::errhandlingapi::{get_last_error, set_last_error, WIN32Error};
use crate::rendering::win32::libloader::get_module_handle_w;
use crate::rendering::win32::winuser::WindowClassStyle;

pub mod win32;

pub fn create_and_register_window_class() -> Result<u16, windows::core::Error> {
	let instance_handle = get_module_handle_w(None)?;

	let class_style =
			WindowClassStyle::VerticalRedraw |
			WindowClassStyle::HorizontalRedraw;

	let class_name_string = U16CString::from_str("Raytrace Window").unwrap();

	let class_description = WNDCLASSEXW {
		cbSize: size_of::<WNDCLASSEXW>() as u32,
		style: WNDCLASS_STYLES(class_style.bits()),
		lpfnWndProc: Some(window_proc),
		cbClsExtra: 0,
		cbWndExtra: 0,
		hInstance: instance_handle.value,
		hIcon: HICON(0),
		hCursor: HCURSOR(0),
		hbrBackground: HBRUSH(0),
		lpszMenuName: PCWSTR(null()),
		lpszClassName: PCWSTR(class_name_string.as_ptr()),
		hIconSm: HICON(0)
	};

	set_last_error(WIN32Error::new_ok());

	let result = unsafe {
		RegisterClassExW(&class_description as *const WNDCLASSEXW)
	};

	return if result == 0 {
		get_last_error().ok().map(|_| 0)
	}
	else {
		Ok(result)
	}
}

unsafe extern "system" fn window_proc(wnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
	if msg == WM_DESTROY {
		PostQuitMessage(0);
		return LRESULT(0);
	}

	DefWindowProcW(wnd, msg, wparam, lparam)
}

pub fn create_and_show_window() -> Result<HWND, windows::core::Error> {
	let instance_handle = get_module_handle_w(None)?;

	let window_style =
				WindowStyle::Overlapped |
				WindowStyle::Caption |
				WindowStyle::SysMenu |
				WindowStyle::MinimizeBox;
	let extended_window_style = ExtendedWindowStyle::empty();


	set_last_error(WIN32Error::new_ok());

	let mut window_rect = RECT {
		left: 0 as i32,
		top: 0 as i32,
		right: 100 as i32,
		bottom: 100 as i32
	};

	let result = unsafe {
		AdjustWindowRectEx(
			&mut window_rect as *mut RECT,
			WINDOW_STYLE(window_style.bits),
			BOOL(0),
			WINDOW_EX_STYLE(extended_window_style.bits)
		)
	};

	if !result.as_bool() {
		let error = get_last_error();
		let error = error.ok();
		return error.map(|_| HWND(0));
	}

	let class_name = U16CString::from_str("raytrace_class").unwrap();
	let window_name = U16CString::from_str("Raytrace Window").unwrap();

	let class_name_ptr = class_name.as_ptr();
	let window_name_ptr = window_name.as_ptr();

	set_last_error(WIN32Error::new_ok());

	let result = unsafe {
		CreateWindowExW(
			WINDOW_EX_STYLE(extended_window_style.bits),
			PCWSTR(class_name.as_ptr()),
			PCWSTR(window_name.as_ptr()),
			WINDOW_STYLE(window_style.bits),
			window_rect.left, window_rect.top,
			window_rect.right - window_rect.left, window_rect.bottom - window_rect.top,
			None,
			None,
			instance_handle.value,
			null()
		)
	};

	drop(class_name_ptr);
	drop(class_name);
	drop(window_name_ptr);
	drop(window_name);

	let result = if result.0 == 0 {
		let error = get_last_error().ok();
		eprintln!("got a 0 hwnd, error code: {:?}", error);
		return error.map(|_| HWND(0));
	}
	else {
		result
	};

	let show_command = CommandShow::Show;

	unsafe {
		ShowWindow(result, SHOW_WINDOW_CMD(show_command.into()));
	}

	Ok(result)
}

bitflags!{
	pub struct WindowStyle: u32 {
		const Border = 0x00800000;
		const Caption = 0x00C00000;
		const Child = 0x40000000;
		const ChildWindow = 0x40000000;
		const ClipChildren = 0x02000000;
		const ClipSiblings = 0x04000000;
		const Disabled = 0x08000000;
		const DialougeFrame = 0x00400000;
		const Group = 0x00020000;
		const HorizontalScroll = 0x00100000;
		const Iconic = 0x20000000;
		const Maximize = 0x01000000;
		const MaximizeBox = 0x00010000;
		const Minimize = 0x20000000;
		const MinimizeBox = 0x00020000;
		const Overlapped = 0x00000000;
		const Popup = 0x80000000;
		const SizeBox = 0x00040000;
		const SysMenu = 0x00080000;
		const TabStop = 0x00010000;
		const ThickFrame = 0x00040000;
		const Tiled = 0x00000000;
		const Visible = 0x10000000;
		const VerticalScroll = 0x00200000;
		const OverlappedWindow = Self::Overlapped.bits | Self::Caption.bits | Self::SysMenu.bits | Self::ThickFrame.bits | Self::MinimizeBox.bits | Self::MaximizeBox.bits;
		const PopupWindow = Self::Popup.bits | Self::Border.bits | Self::SysMenu.bits;
		const TiledWindow = Self::Overlapped.bits | Self::Caption.bits | Self::SysMenu.bits | Self::ThickFrame.bits | Self::MinimizeBox.bits | Self::MaximizeBox.bits;
	}
	pub struct ExtendedWindowStyle: u32 {
		const AcceptFiles = 0x00000010;
		const AppWindow = 0x00040000;
		const ClientEdge = 0x00000200;
		const Composited = 0x02000000;
		const ContextHelp = 0x00000400;
		const ControlPanel = 0x00010000;
		const DialougeModalFrame = 0x00000001;
		const Layered = 0x00080000;
		const LayoutRightToLeft = 0x00400000;
		const Left = 0x00000000;
		const LeftScrollBar = 0x00004000;
		const LeftToRightReading = 0x00000000;
		const MDIChild = 0x00000040;
		const NoActivate = 0x08000000;
		const NoInheritLayout = 0x00100000;
		const NoParentNotify = 0x00000004;
		const NoRedirectionBitmap = 0x00200000;
		const Right = 0x00001000;
		const RightScrollbar = 0x0000000;
		const RightToLeftReading = 0x00002000;
		const StaticEdge = 0x00020000;
		const ToolWindow = 0x00000080;
		const TopMost = 0x00000008;
		const Transparent = 0x00000020;
		const WindowEdge = 0x00000100;
		const OverlappedWindow = Self::WindowEdge.bits | Self::ClientEdge.bits;
		const PaletteWindow = Self::WindowEdge.bits | Self::ToolWindow.bits | Self::TopMost.bits;
	}
}

pub enum CommandShow {
	Hide,
	Normal,
	Minimized,
	Maximized,
	NormalNoActive,
	Show,
	Minimize,
	MinimizedNoActive,
	NoActive,
	Restore,
	Default,
	ForceMinimize,
}

impl Into<u32> for CommandShow {
	fn into(self) -> u32 {
		match self {
			CommandShow::Hide => 0,
			CommandShow::Normal => 1,
			CommandShow::Minimized => 2,
			CommandShow::Maximized => 3,
			CommandShow::NormalNoActive => 4,
			CommandShow::Show => 5,
			CommandShow::Minimize => 6,
			CommandShow::MinimizedNoActive => 7,
			CommandShow::NoActive => 8,
			CommandShow::Restore => 9,
			CommandShow::Default => 10,
			CommandShow::ForceMinimize => 11
		}
	}
}

pub enum D3DDriverType {
	Unknown,
	Hardware,
	Reference,
	Null,
	Software,
	Warp
}

impl Into<D3D_DRIVER_TYPE> for D3DDriverType {
	fn into(self) -> D3D_DRIVER_TYPE {
		match self {
			D3DDriverType::Unknown => D3D_DRIVER_TYPE_UNKNOWN,
			D3DDriverType::Hardware => D3D_DRIVER_TYPE_HARDWARE,
			D3DDriverType::Reference => D3D_DRIVER_TYPE_REFERENCE,
			D3DDriverType::Null => D3D_DRIVER_TYPE_NULL,
			D3DDriverType::Software => D3D_DRIVER_TYPE_SOFTWARE,
			D3DDriverType::Warp => D3D_DRIVER_TYPE_WARP
		}
	}
}

bitflags!{
	pub struct CreateDeviceFlag: u32 {
		const Singlethreaded = 0x1;
		const Debug = 0x2;
		const SwitchToRef = 0x4;
		const PreventInternalThreadingOptimizations = 0x8;
		const BRGA_Support = 0x20;
		const DeviceDebuggable = 0x40;
		const PreventAlteringLayerSettingsFromRegistry = 0x80;
		const DisableGPUTimeout = 0x100;
		const VideoSupport = 0x800;
	}
}

pub enum DXGIFormat {
	Unknown,
	R32G32B32A32_Typeless,
	R32G32B32A32_Float,
	R32G32B32A32_UInt,
	R32G32B32A32_SInt,
	R32G32B32_Typeless,
	R32G32B32_Float,
	R32G32B32_UInt,
	R32G32B32_Sint,
	R16G16B16A16_Typeless,
	R16G16B16A16_Float,
	R16G16B16A16_UNorm,
	R16G16B16A16_UInt,
	R16G16B16A16_SNorm,
	R16G16B16A16_SInt,
	R32G32_Typeless,
	R32G32_Float,
	R32G32_UInt,
	R32G32_SInt,
	R32G8X24_Typeless,
	D32_Float_S8X24_UInt,
	R32_Float_X8X24_Typeless,
	X32_Typeless_G8X24_UInt,
	R10G10B10A2_Typeless,
	R10G10B10A2_UNorm,
	R10G10B10A2_UInt,
	R11G11B10_Float,
	R8G8B8A8_Typeless,
	R8G8B8A8_UNorm,
	R8G8B8A8_UNorm_SRGB,
	R8G8B8A8_UInt,
	R8G8B8A8_SNorm,
	R8G8B8A8_SInt,
	R16G16_Typeless,
	R16G16_Float,
	R16G16_UNorm,
	R16G16_UInt,
	R16G16_SNorm,
	R16G16_SInt,
	R32_Typeless,
	D32_Float,
	R32_Float,
	R32_UInt,
	R32_SInt,
	R24G8_Typeless,
	D24_UNorm_S8_UInt,
	R24_UNorm_X8_Typeless,
	X24_Typeless_G8_UInt,
	R8G8_Typeless,
	R8G8_UNorm,
	R8G8_UInt,
	R8G8_SNorm,
	R8G8_SInt,
	R16_Typeless,
	R16_Float,
	D16_UNorm,
	R16_UNorm,
	R16_UInt,
	R16_SNorm,
	R16_SInt,
	R8_Typeless,
	R8_UNorm,
	R8_UInt,
	R8_SNorm,
	R8_SInt,
	A8_UNorm,
	R1_UNorm,
	R9G9B9E5_SharedExp,
	R8G8_B8G8_UNorm,
	G8R8_G8B8_UNorm,
	BC1_Typeless,
	BC1_UNorm,
	BC1_UNorm_SRGB,
	BC2_Typeless,
	BC2_UNorm,
	BC2_UNorm_SRGB,
	BC3_Typeless,
	BC3_UNorm,
	BC3_UNorm_SRGB,
	BC4_Typeless,
	BC4_UNorm,
	BC4_SNorm,
	BC5_Typeless,
	BC5_UNorm,
	BC5_SNorm,
	B5G6R5_UNorm,
	B5G5R5A1_UNorm,
	B8G8R8A8_UNorm,
	B8G8R8X8_UNorm,
	R10G10B10_XR_Bias_A2_UNorm,
	B8G8R8A8_Typeless,
	B8G8R8A8_UNorm_SRGB,
	B8G8R8X8_Typeless,
	B8G8R8X8_UNorm_SRGB,
	BC6H_Typeless,
	BC6H_UF16,
	BC6H_SF16,
	BC7_Typeless,
	BC7_UNorm,
	BC7_UNorm_SRGB,
	AYUV,
	Y410,
	Y416,
	NV12,
	P010,
	P016,
	Opaque_420,
	YUY2,
	Y210,
	Y216,
	NV11,
	AI44,
	IA44,
	P8,
	A8P8,
	B4G4R4A4_UNorm,
	P208,
	V208,
	V408,
	Sampler_Feedback_Min_Mip_Opaque,
	Sampler_Feedback_Mip_Region_Used_Opague,
	Force_UInt
}

impl Into<DXGI_FORMAT> for DXGIFormat {
	fn into(self) -> DXGI_FORMAT {
		match self {
			DXGIFormat::Unknown => DXGI_FORMAT_UNKNOWN,
			DXGIFormat::R32G32B32A32_Typeless => DXGI_FORMAT_R32G32B32A32_TYPELESS,
			DXGIFormat::R32G32B32A32_Float => DXGI_FORMAT_R32G32B32A32_FLOAT,
			DXGIFormat::R32G32B32A32_UInt => DXGI_FORMAT_R32G32B32A32_UINT,
			DXGIFormat::R32G32B32A32_SInt => DXGI_FORMAT_R32G32B32A32_SINT,
			DXGIFormat::R32G32B32_Typeless => DXGI_FORMAT_R32G32B32_TYPELESS,
			DXGIFormat::R32G32B32_Float => DXGI_FORMAT_R32G32B32_FLOAT,
			DXGIFormat::R32G32B32_UInt => DXGI_FORMAT_R32G32B32_UINT,
			DXGIFormat::R32G32B32_Sint => DXGI_FORMAT_R32G32B32_SINT,
			DXGIFormat::R16G16B16A16_Typeless => DXGI_FORMAT_R16G16B16A16_TYPELESS,
			DXGIFormat::R16G16B16A16_Float => DXGI_FORMAT_R16G16B16A16_FLOAT,
			DXGIFormat::R16G16B16A16_UNorm => DXGI_FORMAT_R16G16B16A16_UNORM,
			DXGIFormat::R16G16B16A16_UInt => DXGI_FORMAT_R16G16B16A16_UINT,
			DXGIFormat::R16G16B16A16_SNorm => DXGI_FORMAT_R16G16B16A16_SNORM,
			DXGIFormat::R16G16B16A16_SInt => DXGI_FORMAT_R16G16B16A16_SINT,
			DXGIFormat::R32G32_Typeless => DXGI_FORMAT_R32G32_TYPELESS,
			DXGIFormat::R32G32_Float => DXGI_FORMAT_R32G32_FLOAT,
			DXGIFormat::R32G32_UInt => DXGI_FORMAT_R32G32_UINT,
			DXGIFormat::R32G32_SInt => DXGI_FORMAT_R32G32_SINT,
			DXGIFormat::R32G8X24_Typeless => DXGI_FORMAT_R32G8X24_TYPELESS,
			DXGIFormat::D32_Float_S8X24_UInt => DXGI_FORMAT_D32_FLOAT_S8X24_UINT,
			DXGIFormat::R32_Float_X8X24_Typeless => DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS,
			DXGIFormat::X32_Typeless_G8X24_UInt => DXGI_FORMAT_X32_TYPELESS_G8X24_UINT,
			DXGIFormat::R10G10B10A2_Typeless => DXGI_FORMAT_R10G10B10A2_TYPELESS,
			DXGIFormat::R10G10B10A2_UNorm => DXGI_FORMAT_R10G10B10A2_UNORM,
			DXGIFormat::R10G10B10A2_UInt => DXGI_FORMAT_R10G10B10A2_UINT,
			DXGIFormat::R11G11B10_Float => DXGI_FORMAT_R11G11B10_FLOAT,
			DXGIFormat::R8G8B8A8_Typeless => DXGI_FORMAT_R8G8B8A8_TYPELESS,
			DXGIFormat::R8G8B8A8_UNorm => DXGI_FORMAT_R8G8B8A8_UNORM,
			DXGIFormat::R8G8B8A8_UNorm_SRGB => DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
			DXGIFormat::R8G8B8A8_UInt => DXGI_FORMAT_R8G8B8A8_UINT,
			DXGIFormat::R8G8B8A8_SNorm => DXGI_FORMAT_R8G8B8A8_SNORM,
			DXGIFormat::R8G8B8A8_SInt => DXGI_FORMAT_R8G8B8A8_SINT,
			DXGIFormat::R16G16_Typeless => DXGI_FORMAT_R16G16_TYPELESS,
			DXGIFormat::R16G16_Float => DXGI_FORMAT_R16G16_FLOAT,
			DXGIFormat::R16G16_UNorm => DXGI_FORMAT_R16G16_UNORM,
			DXGIFormat::R16G16_UInt => DXGI_FORMAT_R16G16_UINT,
			DXGIFormat::R16G16_SNorm => DXGI_FORMAT_R16G16_SNORM,
			DXGIFormat::R16G16_SInt => DXGI_FORMAT_R16G16_SINT,
			DXGIFormat::R32_Typeless => DXGI_FORMAT_R32_TYPELESS,
			DXGIFormat::D32_Float => DXGI_FORMAT_D32_FLOAT,
			DXGIFormat::R32_Float => DXGI_FORMAT_R32_FLOAT,
			DXGIFormat::R32_UInt => DXGI_FORMAT_R32_UINT,
			DXGIFormat::R32_SInt => DXGI_FORMAT_R32_SINT,
			DXGIFormat::R24G8_Typeless => DXGI_FORMAT_R24G8_TYPELESS,
			DXGIFormat::D24_UNorm_S8_UInt => DXGI_FORMAT_D24_UNORM_S8_UINT,
			DXGIFormat::R24_UNorm_X8_Typeless => DXGI_FORMAT_R24_UNORM_X8_TYPELESS,
			DXGIFormat::X24_Typeless_G8_UInt => DXGI_FORMAT_X24_TYPELESS_G8_UINT,
			DXGIFormat::R8G8_Typeless => DXGI_FORMAT_R8G8_TYPELESS,
			DXGIFormat::R8G8_UNorm => DXGI_FORMAT_R8G8_UNORM,
			DXGIFormat::R8G8_UInt => DXGI_FORMAT_R8G8_UINT,
			DXGIFormat::R8G8_SNorm => DXGI_FORMAT_R8G8_SNORM,
			DXGIFormat::R8G8_SInt => DXGI_FORMAT_R8G8_SINT,
			DXGIFormat::R16_Typeless => DXGI_FORMAT_R16_TYPELESS,
			DXGIFormat::R16_Float => DXGI_FORMAT_R16_FLOAT,
			DXGIFormat::D16_UNorm => DXGI_FORMAT_D16_UNORM,
			DXGIFormat::R16_UNorm => DXGI_FORMAT_R16_UNORM,
			DXGIFormat::R16_UInt => DXGI_FORMAT_R16_UINT,
			DXGIFormat::R16_SNorm => DXGI_FORMAT_R16_SNORM,
			DXGIFormat::R16_SInt => DXGI_FORMAT_R16_SINT,
			DXGIFormat::R8_Typeless => DXGI_FORMAT_R8_TYPELESS,
			DXGIFormat::R8_UNorm => DXGI_FORMAT_R8_UNORM,
			DXGIFormat::R8_UInt => DXGI_FORMAT_R8_UINT,
			DXGIFormat::R8_SNorm => DXGI_FORMAT_R8_SNORM,
			DXGIFormat::R8_SInt => DXGI_FORMAT_R8_SINT,
			DXGIFormat::A8_UNorm => DXGI_FORMAT_A8_UNORM,
			DXGIFormat::R1_UNorm => DXGI_FORMAT_R1_UNORM,
			DXGIFormat::R9G9B9E5_SharedExp => DXGI_FORMAT_R9G9B9E5_SHAREDEXP,
			DXGIFormat::R8G8_B8G8_UNorm => DXGI_FORMAT_R8G8_B8G8_UNORM,
			DXGIFormat::G8R8_G8B8_UNorm => DXGI_FORMAT_G8R8_G8B8_UNORM,
			DXGIFormat::BC1_Typeless => DXGI_FORMAT_BC1_TYPELESS,
			DXGIFormat::BC1_UNorm => DXGI_FORMAT_BC1_UNORM,
			DXGIFormat::BC1_UNorm_SRGB => DXGI_FORMAT_BC1_UNORM_SRGB,
			DXGIFormat::BC2_Typeless => DXGI_FORMAT_BC2_TYPELESS,
			DXGIFormat::BC2_UNorm => DXGI_FORMAT_BC2_UNORM,
			DXGIFormat::BC2_UNorm_SRGB => DXGI_FORMAT_BC2_UNORM_SRGB,
			DXGIFormat::BC3_Typeless => DXGI_FORMAT_BC3_TYPELESS,
			DXGIFormat::BC3_UNorm => DXGI_FORMAT_BC3_UNORM,
			DXGIFormat::BC3_UNorm_SRGB => DXGI_FORMAT_BC3_UNORM_SRGB,
			DXGIFormat::BC4_Typeless => DXGI_FORMAT_BC4_TYPELESS,
			DXGIFormat::BC4_UNorm => DXGI_FORMAT_BC4_UNORM,
			DXGIFormat::BC4_SNorm => DXGI_FORMAT_BC4_SNORM,
			DXGIFormat::BC5_Typeless => DXGI_FORMAT_BC5_TYPELESS,
			DXGIFormat::BC5_UNorm => DXGI_FORMAT_BC5_UNORM,
			DXGIFormat::BC5_SNorm => DXGI_FORMAT_BC5_SNORM,
			DXGIFormat::B5G6R5_UNorm => DXGI_FORMAT_B5G6R5_UNORM,
			DXGIFormat::B5G5R5A1_UNorm => DXGI_FORMAT_B5G5R5A1_UNORM,
			DXGIFormat::B8G8R8A8_UNorm => DXGI_FORMAT_B8G8R8A8_UNORM,
			DXGIFormat::B8G8R8X8_UNorm => DXGI_FORMAT_B8G8R8X8_UNORM,
			DXGIFormat::R10G10B10_XR_Bias_A2_UNorm => DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM,
			DXGIFormat::B8G8R8A8_Typeless => DXGI_FORMAT_B8G8R8A8_TYPELESS,
			DXGIFormat::B8G8R8A8_UNorm_SRGB => DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
			DXGIFormat::B8G8R8X8_Typeless => DXGI_FORMAT_B8G8R8X8_TYPELESS,
			DXGIFormat::B8G8R8X8_UNorm_SRGB => DXGI_FORMAT_B8G8R8X8_UNORM_SRGB,
			DXGIFormat::BC6H_Typeless => DXGI_FORMAT_BC6H_TYPELESS,
			DXGIFormat::BC6H_UF16 => DXGI_FORMAT_BC6H_UF16,
			DXGIFormat::BC6H_SF16 => DXGI_FORMAT_BC6H_SF16,
			DXGIFormat::BC7_Typeless => DXGI_FORMAT_BC7_TYPELESS,
			DXGIFormat::BC7_UNorm => DXGI_FORMAT_BC7_UNORM,
			DXGIFormat::BC7_UNorm_SRGB => DXGI_FORMAT_BC7_UNORM_SRGB,
			DXGIFormat::AYUV => DXGI_FORMAT_AYUV,
			DXGIFormat::Y410 => DXGI_FORMAT_Y410,
			DXGIFormat::Y416 => DXGI_FORMAT_Y416,
			DXGIFormat::NV12 => DXGI_FORMAT_NV12,
			DXGIFormat::P010 => DXGI_FORMAT_P010,
			DXGIFormat::P016 => DXGI_FORMAT_P016,
			DXGIFormat::Opaque_420 => DXGI_FORMAT_420_OPAQUE,
			DXGIFormat::YUY2 => DXGI_FORMAT_YUY2,
			DXGIFormat::Y210 => DXGI_FORMAT_Y210,
			DXGIFormat::Y216 => DXGI_FORMAT_Y216,
			DXGIFormat::NV11 => DXGI_FORMAT_NV11,
			DXGIFormat::AI44 => DXGI_FORMAT_AI44,
			DXGIFormat::IA44 => DXGI_FORMAT_IA44,
			DXGIFormat::P8 => DXGI_FORMAT_P8,
			DXGIFormat::A8P8 => DXGI_FORMAT_A8P8,
			DXGIFormat::B4G4R4A4_UNorm => DXGI_FORMAT_B4G4R4A4_UNORM,
			DXGIFormat::P208 => DXGI_FORMAT_P208,
			DXGIFormat::V208 => DXGI_FORMAT_V208,
			DXGIFormat::V408 => DXGI_FORMAT_V408,
			DXGIFormat::Sampler_Feedback_Min_Mip_Opaque => DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE,
			DXGIFormat::Sampler_Feedback_Mip_Region_Used_Opague => DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE,
			DXGIFormat::Force_UInt => DXGI_FORMAT_FORCE_UINT,
		}
	}
}

pub enum DXGIScanlineOrder {
	Unspecified,
	Progressive,
	UpperFieldFirst,
	LowerFieldFirst
}

impl Into<DXGI_MODE_SCANLINE_ORDER> for DXGIScanlineOrder {
	fn into(self) -> DXGI_MODE_SCANLINE_ORDER {
		match self {
			DXGIScanlineOrder::Unspecified => DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
			DXGIScanlineOrder::Progressive => DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
			DXGIScanlineOrder::UpperFieldFirst => DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST,
			DXGIScanlineOrder::LowerFieldFirst => DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST
		}
	}
}

pub enum DXGIScaling {
	Unspecified,
	Centered,
	Stretched
}

impl Into<DXGI_MODE_SCALING> for DXGIScaling {
	fn into(self) -> DXGI_MODE_SCALING {
		match self {
			DXGIScaling::Unspecified => DXGI_MODE_SCALING_UNSPECIFIED,
			DXGIScaling::Centered => DXGI_MODE_SCALING_CENTERED,
			DXGIScaling::Stretched => DXGI_MODE_SCALING_STRETCHED
		}
	}
}

bitflags!{
	pub struct DXGIUsage: u32 {
		const CPUAccessNone = DXGI_CPU_ACCESS_NONE;
		const CPUAccessDynamic = DXGI_CPU_ACCESS_DYNAMIC;
		const CPUAccessReadWrite = DXGI_CPU_ACCESS_READ_WRITE;
		const CPUAccessScratch = DXGI_CPU_ACCESS_SCRATCH;
		const CPUAccessField = DXGI_CPU_ACCESS_FIELD;
		const BackBuffer = DXGI_USAGE_BACK_BUFFER;
		const DiscardOnPresent = DXGI_USAGE_DISCARD_ON_PRESENT;
		const ReadOnly = DXGI_USAGE_READ_ONLY;
		const RenderTargetOutput = DXGI_USAGE_RENDER_TARGET_OUTPUT;
		const ShaderInput = DXGI_USAGE_SHADER_INPUT;
		const Shared = DXGI_USAGE_SHARED;
		const UnorderedAccess = DXGI_USAGE_UNORDERED_ACCESS;
	}
}

pub enum DXGISwapEffect {
	Discard,
	Sequential,
	FlipSequential,
	FlipDiscard,
}

impl Into<DXGI_SWAP_EFFECT> for DXGISwapEffect {
	fn into(self) -> DXGI_SWAP_EFFECT {
		match self {
			DXGISwapEffect::Discard => DXGI_SWAP_EFFECT_DISCARD,
			DXGISwapEffect::Sequential => DXGI_SWAP_EFFECT_SEQUENTIAL,
			DXGISwapEffect::FlipSequential => DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
			DXGISwapEffect::FlipDiscard => DXGI_SWAP_EFFECT_FLIP_DISCARD
		}
	}
}

bitflags!{
	pub struct DXGISwapChainFlag: u32 {
		const NonPreRotated = DXGI_SWAP_CHAIN_FLAG_NONPREROTATED.0 as u32;
		const AllowModeSwitch = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32;
		const GDICompatible = DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE.0 as u32;
		const RestrictedContent = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT.0 as u32;
		const RestrictSharedResourceDriver = DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER.0 as u32;
		const DisplayOnly = DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY.0 as u32;
		const FrameLatencyWaitableObject = DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT.0 as u32;
		const ForegroundLayer = DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER.0 as u32;
		const FullscreenVideo = DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO.0 as u32;
		const YUVVideo = DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO.0 as u32;
		const HWProtected = DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED.0 as u32;
		const AllowTearing = DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0 as u32;
		const RestrictedToAllHolographicDisplays = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS.0 as u32;
	}
}

pub fn setup_directx_device_and_swapchain(width: u32, height: u32, window_handle: HWND) -> Result<(D3D_FEATURE_LEVEL, IDXGISwapChain, ID3D11Device, ID3D11DeviceContext), windows::core::Error> {
	let refresh_rate = DXGI_RATIONAL {
		Numerator: 1,
		Denominator: 60
	};

	let buffer_format = DXGIFormat::R8G8B8A8_UNorm;
	let scanline_order = DXGIScanlineOrder::Progressive;
	let scaling = DXGIScaling::Unspecified;

	let buffer_description = DXGI_MODE_DESC {
		Width: width,
		Height: height,
		RefreshRate: refresh_rate,
		Format: buffer_format.into(),
		ScanlineOrdering: scanline_order.into(),
		Scaling: scaling.into()
	};

	let sample_description = DXGI_SAMPLE_DESC {
		Count: 1,
		Quality: 0
	};

	let buffer_usage = DXGIUsage::RenderTargetOutput;
	let swap_effect = DXGISwapEffect::FlipDiscard;
	let flags = DXGISwapChainFlag::empty();

	let swap_chain_description = DXGI_SWAP_CHAIN_DESC {
		BufferDesc: buffer_description,
		SampleDesc: sample_description,
		BufferUsage: buffer_usage.bits,
		BufferCount: 2,
		OutputWindow: window_handle,
		Windowed: BOOL::from(true),
		SwapEffect: swap_effect.into(),
		Flags: flags.bits
	};

	let mut swap_chain = MaybeUninit::<Option<IDXGISwapChain>>::uninit();
	let mut device = MaybeUninit::<Option<ID3D11Device>>::uninit();
	let mut device_context = MaybeUninit::<Option<ID3D11DeviceContext>>::uninit();
	let mut feature_level = MaybeUninit::<D3D_FEATURE_LEVEL>::uninit();

	set_last_error(WIN32Error::new_ok());

	let result = unsafe {
		D3D11CreateDeviceAndSwapChain(
			None,
			D3DDriverType::Hardware.into(),
			None,
			D3D11_CREATE_DEVICE_FLAG((CreateDeviceFlag::Debug).bits()),
			&[
				D3D_FEATURE_LEVEL_11_0,
				D3D_FEATURE_LEVEL_10_1,
				D3D_FEATURE_LEVEL_10_0,
				D3D_FEATURE_LEVEL_9_3,
				D3D_FEATURE_LEVEL_9_2,
				D3D_FEATURE_LEVEL_9_1
			],
			D3D11_SDK_VERSION,
			&swap_chain_description as *const DXGI_SWAP_CHAIN_DESC,
			swap_chain.as_mut_ptr(),
			device.as_mut_ptr(),
			feature_level.as_mut_ptr(),
			device_context.as_mut_ptr()
		)
	};

	if let Err(err) = result {
		dump_debug_messages();
		eprintln!("err: {:?}", err);
		let error = get_last_error();
		panic!("last error: {:?}", error);
	}

	let (feature_level, swap_chain, device, device_context) = unsafe {
		(
			feature_level.assume_init(),
			swap_chain.assume_init().unwrap(),
			device.assume_init().unwrap(),
			device_context.assume_init().unwrap()
		)
	};

	Ok((feature_level, swap_chain, device, device_context))
}

pub fn create_device() -> Result<(D3D_FEATURE_LEVEL, ID3D11Device, ID3D11DeviceContext), windows::core::Error> {
	let mut feature_level = MaybeUninit::<D3D_FEATURE_LEVEL>::uninit();
	let mut device = MaybeUninit::<Option<ID3D11Device>>::uninit();
	let mut device_context = MaybeUninit::<Option<ID3D11DeviceContext>>::uninit();

	unsafe {
		D3D11CreateDevice(
			None,
			D3D_DRIVER_TYPE_HARDWARE,
			None,
			D3D11_CREATE_DEVICE_DEBUG,
			&[
				D3D_FEATURE_LEVEL_11_0,
				D3D_FEATURE_LEVEL_10_1,
				D3D_FEATURE_LEVEL_10_0,
				D3D_FEATURE_LEVEL_9_3,
				D3D_FEATURE_LEVEL_9_2,
				D3D_FEATURE_LEVEL_9_1
			],
			D3D11_SDK_VERSION,
			device.as_mut_ptr(),
			feature_level.as_mut_ptr(),
			device_context.as_mut_ptr(),
		)
	}?;

	let (feature_level, device, device_context) = unsafe {
		(
			feature_level.assume_init(),
			device.assume_init().unwrap(),
			device_context.assume_init().unwrap()
		)
	};

	Ok((feature_level, device, device_context))
}

pub fn create_back_buffer(swap_chain: &IDXGISwapChain) -> Result<ID3D11Resource, windows::core::Error> {
	unsafe {
		swap_chain.GetBuffer::<ID3D11Resource>(0)
	}
}

pub fn create_render_target_view(back_buffer: &ID3D11Resource, device: &ID3D11Device) -> Result<ID3D11RenderTargetView, windows::core::Error> {
	unsafe {
		device.CreateRenderTargetView(
			back_buffer,
			null()
		)
	}
}

pub fn bind_render_target_view(render_target_view: ID3D11RenderTargetView, device_context: &ID3D11DeviceContext) -> ID3D11RenderTargetView {
	let render_target_view = [Some(render_target_view)];

	unsafe {
		device_context.OMSetRenderTargets(&render_target_view, None);
	}

	render_target_view.into_iter().next().unwrap().unwrap()
}

pub fn set_viewport(viewport: D3D11_VIEWPORT, device_context: &ID3D11DeviceContext) -> D3D11_VIEWPORT {
	let viewport = [viewport];

	unsafe {
		device_context.RSSetViewports(&viewport);
	}

	viewport.into_iter().next().unwrap()
}

pub fn create_index_buffer(data: &[u32], device: &ID3D11Device) -> Result<ID3D11Buffer, windows::core::Error> {
	let buffer_description = D3D11_BUFFER_DESC {
		ByteWidth: (size_of::<u32>() * data.len()) as u32,
		Usage: D3D11_USAGE_DEFAULT,
		BindFlags: D3D11_BIND_INDEX_BUFFER.0,
		CPUAccessFlags: 0,
		MiscFlags: 0,
		StructureByteStride: 0
	};

	let buffer_subresource_data = D3D11_SUBRESOURCE_DATA {
		pSysMem: data.as_ptr() as *const c_void,
		SysMemPitch: 0,
		SysMemSlicePitch: 0
	};

	unsafe {
		device.CreateBuffer(&buffer_description as *const D3D11_BUFFER_DESC, &buffer_subresource_data as *const D3D11_SUBRESOURCE_DATA)
	}
}

pub fn bind_index_buffer(buffer: &ID3D11Buffer, device_context: &ID3D11DeviceContext) {
	unsafe {
		device_context.IASetIndexBuffer(buffer, DXGI_FORMAT_R32_UINT, 0);
	}
}

pub fn create_vertex_buffer<V>(data: &[V], device: &ID3D11Device) -> Result<ID3D11Buffer, windows::core::Error> {
	let buffer_description = D3D11_BUFFER_DESC {
		ByteWidth: (size_of::<V>() * data.len()) as u32,
		Usage: D3D11_USAGE_DEFAULT,
		BindFlags: D3D11_BIND_VERTEX_BUFFER.0,
		CPUAccessFlags: 0,
		MiscFlags: 0,
		StructureByteStride: 0
	};

	let buffer_subresource_data = D3D11_SUBRESOURCE_DATA {
		pSysMem: data.as_ptr() as *const c_void,
		SysMemPitch: 0,
		SysMemSlicePitch: 0
	};

	unsafe {
		device.CreateBuffer(&buffer_description as *const D3D11_BUFFER_DESC, &buffer_subresource_data as *const D3D11_SUBRESOURCE_DATA)
	}
}

pub fn bind_vertex_buffer<V>(buffer: ID3D11Buffer, device_context: &ID3D11DeviceContext) -> ID3D11Buffer {
	let buffer = Some(buffer);

	let buffer_stride = size_of::<V>() as u32;
	let buffer_offset = 0u32;

	unsafe {
		device_context.IASetVertexBuffers(0,1, &buffer as *const Option<ID3D11Buffer>, &buffer_stride as *const u32, &buffer_offset as *const u32)
	}

	buffer.unwrap()
}

pub fn create_input_layout(descriptions: &[D3D11_INPUT_ELEMENT_DESC], shader_data: &[u8], device: &ID3D11Device) -> Result<ID3D11InputLayout, windows::core::Error> {
	unsafe {
		device.CreateInputLayout(&descriptions, &shader_data)
	}
}

pub fn bind_input_layout(layout: &ID3D11InputLayout, device_context: &ID3D11DeviceContext) {

	unsafe {
		device_context.IASetInputLayout(layout)
	}
}

pub fn set_primitive_topology(device_context: &ID3D11DeviceContext) {
	unsafe {
		device_context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
	}
}

pub fn create_vertex_shader(bytecode: &[u8], device: &ID3D11Device) -> Result<ID3D11VertexShader, windows::core::Error> {
	unsafe {
		device.CreateVertexShader(
			&bytecode,
			None
		)
	}
}

pub fn bind_vertex_shader(shader: &ID3D11VertexShader, device_context: &ID3D11DeviceContext) {
	let class_instances = [];

	unsafe {
		device_context.VSSetShader(shader, &class_instances);
	}
}

pub fn create_pixel_shader(bytecode: &[u8], device: &ID3D11Device) -> Result<ID3D11PixelShader, windows::core::Error> {
	unsafe {
		device.CreatePixelShader(
			&bytecode,
			None
		)
	}
}

pub fn bind_pixel_shader(shader: &ID3D11PixelShader, device_context: &ID3D11DeviceContext) {
	let class_instances = [];

	unsafe {
		device_context.PSSetShader(shader, &class_instances)
	}
}

pub fn clear_render_target_view(clear_color: [f32; 4], render_target_view: &ID3D11RenderTargetView, device_context: &ID3D11DeviceContext) {
	let color_ptr = &clear_color as *const f32;

	unsafe {
		device_context.ClearRenderTargetView(
			render_target_view,
			color_ptr
		);
	}
}

pub fn draw(vertex_count: u32, start_vertex_location: u32, device_context: &ID3D11DeviceContext) {
	unsafe {
		device_context.Draw(vertex_count, start_vertex_location);
	}
}

pub fn draw_indexed(index_count: u32, start_index_location: u32, base_vertex_location: i32, device_context: &ID3D11DeviceContext) {
	unsafe {
		device_context.DrawIndexed(index_count, start_index_location, base_vertex_location);
	}
}

pub fn present(sync_interval: u32, flags: u32, swap_chain: &IDXGISwapChain) -> Result<(), windows::core::Error> {
	unsafe {
		swap_chain.Present(sync_interval,flags)
	}
}

pub fn create_background_texture(width: u32, height: u32, device: &ID3D11Device) -> Result<ID3D11Texture2D, windows::core::Error> {
	let sample_description = DXGI_SAMPLE_DESC {
		Count: 1,
		Quality: 0
	};

	let texture_description = D3D11_TEXTURE2D_DESC {
		Width: width,
		Height: height,
		MipLevels: 1,
		ArraySize: 1,
		Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
		SampleDesc: sample_description,
		Usage: D3D11_USAGE_DYNAMIC,
		BindFlags: D3D11_BIND_SHADER_RESOURCE,
		CPUAccessFlags: D3D11_CPU_ACCESS_WRITE,
		MiscFlags: D3D11_RESOURCE_MISC_FLAG(0)
	};

	unsafe {
		device.CreateTexture2D(&texture_description,null())
	}
}

pub fn map_background_texture(texture: &ID3D11Resource, device_context: &ID3D11DeviceContext) -> Result<D3D11_MAPPED_SUBRESOURCE, windows::core::Error> {
	unsafe {
		device_context.Map(texture, 0, D3D11_MAP_WRITE_DISCARD, 0)
	}
}

pub fn unmap_background_texture(mapped_subresource: D3D11_MAPPED_SUBRESOURCE, texture: &ID3D11Resource, device_context: &ID3D11DeviceContext) {
	drop(mapped_subresource);
	unsafe {
		device_context.Unmap(texture, 0)
	}
}

pub fn create_background_texture_resource_view(texture: &ID3D11Resource, device: &ID3D11Device) -> Result<ID3D11ShaderResourceView, windows::core::Error> {
	let _resource_view_description = D3D11_SHADER_RESOURCE_VIEW_DESC {
		Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
		ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
		Anonymous: Default::default()
	};

	unsafe {
		device.CreateShaderResourceView(
			texture,
			null()
		)
	}
}

pub fn bind_background_texture(texture_resource_view: ID3D11ShaderResourceView, device_context: &ID3D11DeviceContext) -> ID3D11ShaderResourceView {
	let resource_view = [Some(texture_resource_view)];
	unsafe {
		device_context.PSSetShaderResources(0,&resource_view);
	}
	resource_view.into_iter().next().unwrap().unwrap()
}