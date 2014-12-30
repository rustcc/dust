#![allow(non_snake_case)]

#![allow(unused_variables)]

use libc::{c_int,c_void};

use std::rc::Rc;
use std::cell::RefCell;

use super::super::win::types::*;
use super::super::win::api::*;
use super::super::win::encode::*;
use super::super::widgets::button::Button;
use super::super::widgets::edit::Edit;
use super::super::{Dust,Wnd,TLS_DUST,hookWndCreate,UnHookWndCreate,emptyWndProc,MessageBox};

//use super::super::widgets::button::Button;

// 所有窗口 组件 都必须实现的接口。
// 部分方法 preTranslate wndProc 消息映射需要用到.

pub struct Window{
  hWnd: HWND,
  wndProc: WndProc
}

impl Wnd for Window{
//  fn getSelf(&mut self)->&mut Window{self}
  fn preTranslate(&self,hWnd: HWND,msg:& mut MSG)->bool
  {
    msg.TranslateMessage();
    msg.DispatchMessage();
    false
  }
  fn setHwnd(&mut self,h: HWND){self.hWnd=h; }
  fn getHwnd(&self)->HWND{self.hWnd}
  fn setwndProc(&mut self,p: WndProc){self.wndProc=p;}
  fn getWndProc(&self)->WndProc{self.wndProc}

  fn wndProc(&self, hWnd: HWND, msg:u32, wparam:WPARAM, lparam:LPARAM)->int
  {
  //  println!("HWND={}, msg={}, wparam={}, lparam={}", hWnd, msg, wparam, lparam);
    match msg{
      1=>{ //创建完毕
        Button::new(self, "点点点",10,10,200,25,100);
        Edit::new(self,220,10,200,25,101);
        Edit::new(self,10,45,200,25,102);
      },
      _=>{

      }
    }
    unsafe{
      return CallWindowProcW(self.wndProc, hWnd, msg, wparam, lparam) as int;
    }
  }
}
impl Drop for Window{
  fn drop(&mut self){
    println!("drop window");
  }
}

extern "stdcall" fn defWindowProc(hWnd:HWND, msg: u32, wparam: WPARAM,lparam: LPARAM)->c_int{
  unsafe{
    DefWindowProcW(hWnd,msg,wparam,lparam)
  }
}


impl Window{
  pub fn new(title:&str, x:int, y:int, w:int, h:int,parent:Option<&Window>)->bool
  {
      let mut win = box Window {hWnd:0 as HWND, wndProc:emptyWndProc};
      let hWnd= if parent.is_some(){ parent.unwrap().hWnd}else{0 as HWND};
      let mut mhWnd:HWND= 0 as HWND;
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

        mhWnd = CreateWindowExW(0, wndcls.as_ptr(), UTF82UCS2(title).as_ptr(), 13565952, x , y , w , h , hWnd, 0 as HMENU, handle, C_NULL);
        UnHookWndCreate();
        // 默认情况下 显示该窗口

        ShowWindow(mhWnd, 5);
      }
      true
  }
  pub fn hwnd(&self)->HWND{
    self.hWnd
  }
}


#[test]
fn testdust()
{
  let wnd = Window::new("title",0,0, 800,600, 0 as HWND);

}
