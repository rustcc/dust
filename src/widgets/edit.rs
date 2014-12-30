#![allow(non_snake_case)]

#![allow(unused_variables)]

use libc::{c_int,c_void};

use std::rc::Rc;
use std::cell::RefCell;

use super::super::win::types::*;
use super::super::win::api::*;
use super::super::win::encode::*;
use super::super::{Dust,Wnd,TLS_DUST,hookWndCreate,UnHookWndCreate,emptyWndProc};


// 所有窗口 组件 都必须实现的接口。
// 部分方法 preTranslate wndProc 消息映射需要用到.
pub struct Edit{
  hWnd: HWND,
  wndProc: WndProc
}

impl Wnd for Edit{
  //  fn getSelf(&mut self)->&mut Self{self}
  fn preTranslate(&self,hWnd: HWND,msg:& mut MSG)->bool
  {
    msg.TranslateMessage();
    msg.DispatchMessage();
    false
  }
  fn setHwnd(&mut self,h: HWND){self.hWnd=h; }
  fn getHwnd(&self)->HWND{self.hWnd}
  fn setwndProc(&mut self,p: WndProc){self.wndProc=p;}
  fn getWndProc(&self)->WndProc{self.wndProc }

  fn wndProc(&self, _hWnd: HWND, msg:u32, _wparam:WPARAM, _lparam:LPARAM)->int
  {
    match msg{
      513=>{
        println!(" clicked ! {}", _hWnd);
      },
      _=>{}
    }
    unsafe{
      return CallWindowProcW(self.wndProc, _hWnd, msg, _wparam, _lparam) as int;
    }
  }
}

impl Edit{
  pub fn new(parent:&Wnd, x:int, y:int, w:int, h:int,id:int)->bool
  {
    let mut btn = box Edit{hWnd:0 as HWND, wndProc:emptyWndProc};
    let mut hInst = C_NULL;
    let mut hWnd = C_NULL;
    println!(">>>>>>>>>>Create Edit");
    hookWndCreate(btn);
    unsafe{
      hWnd = CreateWindowExW(
        512,
        UTF82UCS2("Edit").as_ptr(), 0 as * const u16,
        1409351680, x, y, w, h,
        parent.getHwnd(), id as HMENU, GetModuleHandleW(0 as  * const u16), C_NULL);
    }
    UnHookWndCreate();

    if C_NULL != hWnd{
      true
    }else{
      false
    }
  }
}
