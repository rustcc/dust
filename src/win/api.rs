#![allow(non_snake_case)]

#![allow(improper_ctypes)]

#![allow(unused_imports)]

use super::types::*;
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
  pub fn CreateWindowExW(exStyle:u32, name:* const u16, title: * const u16, style: u32 , x: int, y:int,w:int,h:int,parent:HWND, hMenu:HMENU,hInstance:HINSTANCE,lParam:LPARAM) ->HWND;

  pub fn DefWindowProcW(hWnd:HWND, msg:u32, wparam:WPARAM, lparam:LPARAM)->c_int;

  pub fn ShowWindow(hWnd: HWND, mode:c_int)->c_int;

  //Window Text
  pub fn GetWindowTextLengthW(hWnd:HWND)->c_int;
  pub fn GetWindowTextW(hWnd:HWND,lp:*const u16,cch:u32);
  pub fn SetWindowTextW(hwnd:HWND, text: * const u16);

  pub fn GetLastError()->u32;
  pub fn CallWindowProcW(wndProc:WndProc, hWnd:HWND, msg:u32, wparam:WPARAM, lparam:LPARAM)->c_int;
  pub fn InitCommonControlsEx(icex:* const INITCOMMONCONTROLSEX);

  pub fn GetClassLongA(hWnd: HWND, nindex:int)->int;

  pub fn GetCurrentThreadId()->c_int;
  // Window Hook
  pub fn SetWindowsHookExA(hookID: int, hookfn: WindowHookfn,hmod:c_int, threadId:c_int)->int;
  pub fn CallNextHookEx(hookID: int, code:int,wparam:* const c_void, lparam:* const c_void)->int;
  pub fn UnhookWindowsHookEx(hookID: int)->int;
//  WindowLong-- Only 32 bit
  #[cfg(target_word_size = "32")]
  pub fn GetWindowLongW(hwnd:HWND, index: i32) -> * const c_void;
  #[cfg(target_word_size = "32")]
  pub fn SetWindowLongW(hwnd:HWND, index: i32, value: * const c_void)->* const c_void;

  #[cfg(target_word_size = "64")]
  pub fn GetWindowLongPtrW(hwnd:HWND, index: i32) -> * const c_void;
  #[cfg(target_word_size = "64")]
  pub fn SetWindowLongPtrW(hwnd:HWND, index: i32, value: * const c_void) -> * const c_void;
  //
  pub fn GetActiveWindow()->HWND;
  pub fn MessageBoxW(parent:HWND,string:* const u16,title:* const u16, flags: u32)->int;
  //GDI
  pub fn GetStockObject(index:c_int)->c_int;

  //Encoding
  pub fn MultiByteToWideChar(codePage:uint, flags: u32, raw:* const u8,  len_raw: int, out: * const u16, len_buff: int)->int;
  pub fn WideCharToMultiByte(codePage:uint, flags: u32, raw:* const u16,  len_raw: int, out: * const u8, len_buff: int,def:* const c_void,useDef:bool)->int;
}
