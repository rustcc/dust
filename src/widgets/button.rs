#![allow(non_snake_case)]

#![allow(unused_variables)]

use libc::{c_int,c_void};

use super::super::win::types::*;
use super::super::win::api::*;
use super::super::win::encode::*;
use super::super::window::window::*;
use super::super::{Dust,Wnd,TLS_DUST,hookWndCreate,UnHookWndCreate};


// 所有窗口 组件 都必须实现的接口。
// 部分方法 preTranslate wndProc 消息映射需要用到.
pub struct Button{
  hWnd: HWND,
  wndProc: WndProc
}

impl Wnd for Button{
  fn preTranslate(&self,hWnd: HWND,msg:& mut MSG)->bool
  {
    unsafe{
      TranslateMessage (msg);
      DispatchMessageW (msg);
    }
    false
  }
  fn getWndProc(&self)->WndProc{
    self.wndProc
  }

  fn wndProc(&self, _hWnd: HWND, _msg:u32, _wparam:c_int, _lparam:c_int)->int
  {
    unsafe{
      return CallWindowProcW(self.wndProc, _hWnd, _msg, _wparam, _lparam);
    }
  }
}

impl Button{
  pub fn new(parent:&Window, title:&str,x:int,y:int,w:int,h:int)->Box<Button>
  {
    let mut btn = box Button{hWnd:0 as HWND, wndProc:emptyWndProc};
    let mut hInst = 0i32;

    unsafe{
      TLS_DUST.with( | dust | {
        let d = dust.borrow();
        hInst = d.hInstance;
      });
      hookWndCreate(&btn);
      btn.hWnd = CreateWindowExW(
        0,
        UTF82UCS2("button").as_ptr(), UTF82UCS2(title).as_ptr(),
        65536 | 1409286144, x as c_int, y as c_int, w as c_int, h as c_int,
        parent.hwnd(), 100, hInst, 0);
      UnHookWndCreate();

      println!("Got it Button {}, parent={} GetLastError() = {}",btn.hWnd,parent.hwnd(), GetLastError());
    }

    btn
  }
}
