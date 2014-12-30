#![allow(non_snake_case)]
#![allow(unused_assignments)]

use libc::{c_int,c_uint, uint32_t,c_void};


pub type HWND = * const c_void;
pub type LPARAM = * const c_void;
pub type WPARAM = * const c_void;

pub static C_NULL:* const c_void = 0 as * const c_void;

pub type WndProc =extern "stdcall" fn (HWND, u32, WPARAM, LPARAM)->c_int;
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

extern "stdcall"{
//Messages.
pub fn PostMessageW(hWnd:HWND, msg:u32, wparam:WPARAM, lparam:LPARAM)->c_int;
pub fn PostQuitMessage(exitCode:c_int)->c_int;
fn GetMessageW(lpMsg:* mut MSG, hWnd:HWND, wMsgFilterMin:u32, wMsgFilterMax:u32)->bool;
fn TranslateMessage(lpMsg:* mut MSG)->c_int;
fn DispatchMessageW(lpMsg:* mut MSG)->c_int;
pub fn IsDialogMessage(hWnd:HWND, lpMsg:* const MSG)->bool;
}

impl MSG{
  pub fn GetMessage(&mut self,hWin:HWND,wMsgFilterMin:u32, wMsgFilterMax:u32)->bool{
    unsafe{GetMessageW(self, hWin, wMsgFilterMin, wMsgFilterMax)}
  }
  pub fn TranslateMessage(&mut self)->int{
    unsafe{
      TranslateMessage(self) as int
    }
  }
  pub fn DispatchMessage(&mut self)->int{
    unsafe{
      DispatchMessageW(self)as int
    }
  }
  pub fn IsDialogMessage(&self,hWin:HWND)->bool{
    unsafe{
      IsDialogMessage(hWin,self)
    }
  }
}


#[repr(C)]
pub struct INITCOMMONCONTROLSEX{
  pub dwSize: u32,
  pub dwICC:u32
}
