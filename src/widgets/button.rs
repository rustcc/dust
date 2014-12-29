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
pub struct Button{
  hWnd: HWND,
  wndProc: WndProc
}

impl Wnd for Button{
//  fn getSelf(&mut self)->&mut Self{self}
  fn preTranslate(&self,hWnd: HWND,msg:& mut MSG)->bool
  {
    unsafe{
      TranslateMessage (msg);
      DispatchMessageW (msg);
    }
    false
  }
  fn setHwnd(&mut self,h: HWND){self.hWnd=h; }
  fn getHwnd(&self)->HWND{self.hWnd}
  fn setwndProc(&mut self,p: WndProc){self.wndProc=p;}
  fn getWndProc(&self)->WndProc{self.wndProc }

  fn wndProc(&self, hWnd: HWND, msg:u32, _wparam:c_int, _lparam:c_int)->int
  {
    match msg{
      513=>{
        println!(" clicked ! ={}", self.GetText());
        self.SetText("我改了!");
      },
      _=>{}
    }
    unsafe{
      return CallWindowProcW(self.wndProc, hWnd, msg, _wparam, _lparam);
    }
  }
}

impl Button{
  pub fn new(parent:&Wnd, title:&str,x:int,y:int,w:int,h:int,id:int)->bool
  {
    let mut btn = box Button{hWnd:0 as HWND, wndProc:emptyWndProc};
    let mut hInst = 0i32;
    let mut hWnd = 0 as HWND;
    println!(">>>>>>>>>>Create Edit");
    hookWndCreate(btn);
    unsafe{
      hWnd = CreateWindowExW(
        0,
        UTF82UCS2("Button").as_ptr(), UTF82UCS2(title).as_ptr(),
        1409351680, x as c_int, y as c_int, w as c_int, h as c_int,
        parent.getHwnd(), id as i32, GetModuleHandleW(0 as  * const u16), 0);
      }
      UnHookWndCreate();

      if 0 as HWND != hWnd{
        true
        }else{
          false
        }
  }
}
