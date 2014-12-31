#![allow(non_snake_case)]

#![allow(unused_variables)]

use libc::{c_int,c_void};

use std::rc::Rc;
use std::cell::RefCell;

use super::super::win::wnd::{TWnd,DWnd};
use super::super::event::eventlistener::{TEventProcesser,EventProcesser};
use super::super::win::types::*;
use super::super::win::api::*;
use super::super::win::encode::*;
use super::super::event::*;
//use super::super::widgets::button::Button;
//use super::super::widgets::edit::Edit;
use super::super::{Dust,TLS_DUST,hookWndCreate,UnHookWndCreate,emptyWndProc,MessageBox};

//use super::super::widgets::button::Button;

// 所有窗口 组件 都必须实现的接口。
// 部分方法 preTranslate wndProc 消息映射需要用到.

pub struct Window{
  defWindowProc: WndProc
}

impl TEventProcesser for Window{
//  fn getSelf(&mut self)->&mut Window{self}
  fn preTranslateMsg(&self,msg:&mut MSG)->bool
  {
    msg.TranslateMessage();
    msg.DispatchMessage();
    false
  }
  fn setWndProc(&mut self,wproc:WndProc){self.defWindowProc=wproc;}
  fn getWndProc(&self)->WndProc{self.defWindowProc}

  fn msgProcedure(&self, hWin: DWnd, msg:u32, wparam:WPARAM, lparam:LPARAM)->int
  {
  //  println!("DWnd={}, msg={}, wparam={}, lparam={}", hWnd, msg, wparam, lparam);
    match msg{
      1=>{ //创建完毕

        println!("Window On Created! {} {}", hWin.GetText(),0i);
//        Button::new(self, "点点点",10,10,200,25,100);
//        Edit::new(self,220,10,200,25,101);
//        Edit::new(self,10,45,200,25,102);
      },
      _=>{

      }
    }
    unsafe{
      return CallWindowProcW(self.defWindowProc, hWin, msg, wparam, lparam) as int;
    }
  }
}
impl Drop for Window{
  fn drop(&mut self){
    println!("drop window");
  }
}

extern "stdcall" fn defWindowProc(hWnd:DWnd, msg: u32, wparam: WPARAM,lparam: LPARAM)->c_int{
  unsafe{
    DefWindowProcW(hWnd,msg,wparam,lparam)
  }
}


impl Window{
  pub fn new(title:&str, x:int, y:int, w:int, h:int, hWndParent: DWnd)->DWnd
  {
      let mut win = box Window {defWindowProc:emptyWndProc};

      let mut mhWnd:DWnd= 0 as DWnd;
      let wndcls = UTF82UCS2("rust-window");
      unsafe{
        // InitCommonControls/();
        let handle =GetModuleHandleW(0 as * const u16);
        let cls = WNDCLASSEXW{
            cbSize: 48,
            style:8,
            lpfnWndProc: defWindowProc,
            cbClsExtra:0,
            cbWndExtra:0,
            hInstance:handle,
            hIcon:0,
            hCursor:0,
            hbrBackground:16,
            lpszMenuName: 0 as * const u16,
            lpszClassName:wndcls.as_ptr(),
            hIconSm:0
        };


        RegisterClassExW(&cls);
        hookWndCreate(win);

        mhWnd = CreateWindowExW(0, wndcls.as_ptr(), UTF82UCS2(title).as_ptr(), 13565952, x , y , w , h , hWndParent, 0 as HMENU, handle, C_NULL);
        UnHookWndCreate();
        // 默认情况下 显示该窗口

        ShowWindow(mhWnd, 5);
      }
      mhWnd
  }

}


#[test]
fn testdust()
{
  let wnd = Window::new("title",0,0, 800,600, 0 as DWnd);

}
