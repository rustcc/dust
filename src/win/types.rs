#![allow(non_snake_case)]
#![allow(unused_assignments)]

use libc::{c_int,c_uint, uint32_t,c_void};

pub type HWND = * const c_void;
pub type LPARAM = * const c_void;
pub type WPARAM = * const c_void;

pub static C_NULL:* const c_void = 0 as * const c_void;

pub type WndProc =extern "stdcall" fn (HWND, u32, WPARAM,LPARAM)->c_int;
pub type WindowHookfn = extern "stdcall" fn(int,* const c_void, * const c_void)->c_int;

#[repr(C)]
pub struct __WIN_HANDLER{
  unused:int
}

pub type HMENU = * const __WIN_HANDLER;
pub type HINSTANCE = * const __WIN_HANDLER;

#[repr(C)]
pub struct WNDCLASSEXW{
  pub cbSize:uint32_t,
  pub style:uint32_t,
  pub lpfnWndProc:WndProc,
  pub cbClsExtra:c_int,
  pub cbWndExtra:c_int,
  pub hInstance:HINSTANCE,
  pub hIcon:c_int,
  pub hCursor:c_int,
  pub hbrBackground:c_int,
  pub lpszMenuName:* const u16,
  pub lpszClassName:* const u16,
  pub hIconSm:c_int,
}

#[repr(C)]
pub struct POINT{
  pub x:c_int,pub y:c_int
}

#[repr(C)]
pub struct MSG{
  pub handle: HWND,
  pub msg: c_uint,
  pub wparam:c_int,
  pub lparam:c_int,
  pub time:uint32_t,
  pub pt:POINT
}

#[repr(C)]
pub struct INITCOMMONCONTROLSEX{
  pub dwSize: u32,
  pub dwICC:u32
}
