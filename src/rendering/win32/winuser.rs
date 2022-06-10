use bitflags::bitflags;
use windows::Win32::UI::WindowsAndMessaging::WNDPROC;
use crate::rendering::win32::libloader::InstanceHandle;

pub struct WindowClassHandle {

}

pub struct IconHandle {

}

pub struct CursorHandle {

}

pub struct BrushHandle {

}

bitflags! {
	pub struct WindowClassStyle: u32 {
		const VerticalRedraw = 0x0001;
		const HorizontalRedraw = 0x0002;
		const DoubleClicks = 0x0008;
		const OwnDeviceContext = 0x0020;
		const ClassDeviceContext = 0x0040;
		const ParentDeviceContext = 0x0080;
		const NoClose = 0x0200;
		const SaveBits = 0x0800;
		const ByteAlignClient = 0x1000;
		const ByteAlignWindow = 0x2000;
		const GlobalClass = 0x4000;
		const DropShadow = 0x00020000;
	}
}

pub struct WindowClassDescriptionA {
	style: WindowClassStyle,
	window_procedure: WNDPROC,
	class_extra_bytes: u32,
	window_extra_bytes: u32,
	instance_handle: InstanceHandle,
}

pub struct WindowClassDescriptionW {
	style: WindowClassStyle,
	window_procedure: WNDPROC,
	class_extra_bytes: u32,
	window_extra_bytes: u32,
	instance_handle: InstanceHandle,
}

pub struct WindowClassDescriptionExA {
	style: WindowClassStyle,
	window_procedure: WNDPROC,
	class_extra_bytes: u32,
	window_extra_bytes: u32,
	instance_handle: InstanceHandle,
}

pub struct WindowClassDescriptionExW {
	style: WindowClassStyle,
	window_procedure: WNDPROC,
	class_extra_bytes: u32,
	window_extra_bytes: u32,
	instance_handle: InstanceHandle,
}