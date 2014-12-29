#![allow(non_snake_case)]

#![allow(improper_ctypes)]

#![allow(unused_imports)]

use super::types::*;
use libc::{c_int,c_void};

#[link(name = "user32")]
#[link(name = "comctl32")]
#[link(name = "gdi32")]

extern "stdcall"{
  pub fn GetModuleHandleW(n:* const u16) ->c_int;
  //pub fn RegisterClassExA(cls:* const WNDCLASSEX)->c_int;
  pub fn RegisterClassExW(cls:* const WNDCLASSEXW)->int;
  pub fn CreateWindowExW(exStyle:u32, name:* const u16, title: * const u16, style: u32 , x: c_int, y:c_int,w:c_int,h:c_int,parent:HWND, hMenu:c_int,hInstance:c_int,lParam:c_int) ->HWND;

  pub fn DefWindowProcW(hWnd:HWND, msg:u32, wparam:c_int, lparam:c_int)->c_int;
  //Messages.
  pub fn PostMessageW(hWnd:HWND, msg:u32, wparam:c_int, lparam:c_int)->c_int;
  pub fn PostQuitMessage(exitCode:c_int)->c_int;
  pub fn GetMessageW(lpMsg:* mut MSG, hWnd:HWND, wMsgFilterMin:c_int, wMsgFilterMax:c_int)->bool;
  pub fn TranslateMessage(lpMsg:* mut MSG)->c_int;
  pub fn DispatchMessageW(lpMsg:* mut MSG)->c_int;
  pub fn IsDialogMessage(hWnd:HWND, lpMsg:* const MSG)->bool;

  pub fn ShowWindow(hWnd: HWND, mode:c_int)->c_int;

  //Window Text
  pub fn GetWindowTextLengthW(hWnd:HWND)->c_int;
  pub fn GetWindowTextW(hWnd:HWND,lp:*const u16,cch:u32);
  pub fn SetWindowTextW(hwnd:HWND, text: * const u16);

  pub fn GetLastError()->u32;
  pub fn CallWindowProcW(wndProc:WndProc, hWnd:HWND, msg:u32, wparam:c_int, lparam:c_int)->int;
  pub fn InitCommonControlsEx(icex:* const INITCOMMONCONTROLSEX);

  pub fn GetClassLongA(hWnd: HWND, nindex:int)->int;

  pub fn GetCurrentThreadId()->c_int;
  // Window Hook
  pub fn SetWindowsHookExA(hookID: int, hookfn: WindowHookfn,hmod:c_int, threadId:c_int)->int;
  pub fn CallNextHookEx(hookID: int, code:int,wparam:* const c_void, lparam:* const c_void)->int;
  pub fn UnhookWindowsHookEx(hookID: int)->int;
  // WindowLong.
  pub fn GetWindowLongW(hwnd:HWND, index:int) -> int;
  pub fn SetWindowLongW(hwnd:HWND, index: int, value: int) -> WndProc;

  //GDI
  pub fn GetStockObject(index:c_int)->c_int;

  //Encoding
  pub fn MultiByteToWideChar(codePage:uint, flags: u32, raw:* const u8,  len_raw: int, out: * const u16, len_buff: int)->int;
  pub fn WideCharToMultiByte(codePage:uint, flags: u32, raw:* const u16,  len_raw: int, out: * const u8, len_buff: int,def:* const c_void,useDef:bool)->int;
}
