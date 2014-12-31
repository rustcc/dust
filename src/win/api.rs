#![allow(non_snake_case)]

#![allow(improper_ctypes)]

#![allow(unused_imports)]

use super::types::*;
use super::wnd::*;
use libc::{c_int, c_uint, c_void};

#[link(name = "user32")]
#[link(name = "comctl32")]
#[link(name = "gdi32")]


/*
  Type                        | S/U | x86    | x64
  ----------------------------+-----+--------+-------
  BYTE, BOOLEAN               | U   | 8 bit  | 8 bit
  ----------------------------+-----+--------+-------
  SHORT                       | S   | 16 bit | 16 bit
  USHORT, WORD                | U   | 16 bit | 16 bit
  ----------------------------+-----+--------+-------
  INT, LONG                   | S   | 32 bit | 32 bit
  UINT, ULONG, DWORD          | U   | 32 bit | 32 bit
  ----------------------------+-----+--------+-------
  INT_PTR, LONG_PTR, LPARAM   | S   | 32 bit | 64 bit
  UINT_PTR, ULONG_PTR, WPARAM | U   | 32 bit | 64 bit
  ----------------------------+-----+--------+-------
  LONGLONG                    | S   | 64 bit | 64 bit
  ULONGLONG, QWORD            | U   | 64 bit | 64 bit
*/
extern "stdcall"{
  pub fn GetModuleHandleW(n:* const u16) ->HINSTANCE;
  //pub fn RegisterClassExA(cls:* const WNDCLASSEX)->c_int;
  pub fn RegisterClassExW(cls:* const WNDCLASSEXW)->int;
  pub fn CreateWindowExW(exStyle:u32, name:* const u16, title: * const u16, style: u32 , x: int, y:int,w:int,h:int,parent:DWnd, hMenu:HMENU,hInstance:HINSTANCE,lParam:LPARAM) ->DWnd;

  pub fn DefWindowProcW(hWnd:DWnd, msg:u32, wparam:WPARAM, lparam:LPARAM)->c_int;

  pub fn ShowWindow(hWnd: DWnd, mode:c_int)->c_int;

  pub fn GetLastError()->u32;
  pub fn CallWindowProcW(wndProc:WndProc, hWnd:DWnd, msg:u32, wparam:WPARAM, lparam:LPARAM)->c_int;
  pub fn InitCommonControlsEx(icex:* const INITCOMMONCONTROLSEX);

  pub fn GetClassLongA(hWnd: DWnd, nindex:int)->int;

  pub fn GetCurrentThreadId()->c_int;
  // Window Hook
  pub fn SetWindowsHookExA(hookID: int, hookfn: WindowHookfn,hmod:c_int, threadId:c_int)->int;
  pub fn CallNextHookEx(hookID: int, code:int,wparam:* const c_void, lparam:* const c_void)->int;
  pub fn UnhookWindowsHookEx(hookID: int)->int;
//  WindowLong-- Only 32 bit
  #[cfg(target_word_size = "32")]
  pub fn GetWindowLongW(hwnd:DWnd, index: i32) -> * const c_void;
  #[cfg(target_word_size = "32")]
  pub fn SetWindowLongW(hwnd:DWnd, index: i32, value: * const c_void)->* const c_void;

  #[cfg(target_word_size = "64")]
  pub fn GetWindowLongPtrW(hwnd:DWnd, index: i32) -> * const c_void;
  #[cfg(target_word_size = "64")]
  pub fn SetWindowLongPtrW(hwnd:DWnd, index: i32, value: * const c_void) -> * const c_void;
  //
  pub fn GetActiveWindow()->DWnd;
  pub fn MessageBoxW(parent:DWnd,string:* const u16,title:* const u16, flags: u32)->int;
  //GDI
  pub fn GetStockObject(index:c_int)->c_int;

}
