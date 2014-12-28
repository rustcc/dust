#![allow(non_snake_case)]

#![allow(unused_variables)]

use libc::{c_int,c_void};

use super::super::win::types::*;
use super::super::win::api::*;
use super::super::win::encode::*;
use super::super::{Dust,Wnd,TLS_DUST,hookWndCreate,UnHookWndCreate};


// 所有窗口 组件 都必须实现的接口。
// 部分方法 preTranslate wndProc 消息映射需要用到.
pub struct Window{
  hWnd: HWND,
  wndProc: WndProc
}

impl Wnd for Window{
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

extern "stdcall" fn defWindowProc(hWnd:HWND, msg: u32, wparam: c_int,lparam: c_int)->c_int{
  unsafe{
    DefWindowProcA(hWnd,msg,wparam,lparam)
  }
}
pub extern "stdcall" fn emptyWndProc(_a:HWND,_b: u32,_c: c_int,_d: c_int)->c_int{
  0
}

impl Window{
  pub fn new(title:&str, x:int, y:int, width:int, height:int,parent:Option<&Window>)->Box<Window>
  {
      let mut win = box Window{hWnd:0 as HWND, wndProc:emptyWndProc};
      let hWnd= if parent.is_some(){ parent.unwrap().hWnd}else{0 as HWND};
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

        hookWndCreate(&win);
        win.hWnd = CreateWindowExW(0, wndcls.as_ptr(), UTF82UCS2(title).as_ptr(), 13565952, 0, 0, 800, 600, 0 as HWND, 0, handle, 0);
        UnHookWndCreate();
        // 默认情况下 显示该窗口
        ShowWindow(win.hWnd, 5);

        if 0 as HWND != win.hWnd {
          TLS_DUST.with( | d | {
            d.borrow_mut().window_counter+=1;
          });
        }
      }
      win
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
