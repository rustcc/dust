#![allow(non_snake_case)]
#![feature(globs)]
#![allow(dead_code)]

extern crate libc;

use win::types::{HWND,MSG,POINT,WndProc,INITCOMMONCONTROLSEX};
use win::api::*;
use libc::{c_void,c_int};
use window::window::Window;

use std::collections::HashMap;
use std::thread::Thread;
use std::cell::RefCell;

pub mod win;
pub mod window;
pub mod widgets;


trait Wnd{
  fn preTranslate(&self,_hWnd: HWND,_msg:& mut MSG)->bool{
    true
  }
  fn getWndProc(&self)->WndProc;

  fn wndProc(&self, hWnd: HWND, msg:u32, wparam:c_int, lparam:c_int)->int
  {
    unsafe {
      return CallWindowProcW(self.getWndProc(), hWnd, msg, wparam, lparam);
    }
  }
  fn CreateWindow(parent:HWND,exStyle:u32, name:* const u8, title: * const u8, style: u32 , x: int, y:int,w:int,h:int, hMenu:c_int,hInstance:c_int,lParam:c_int)->HWND
  {
    unsafe{
      //return CreateWindowExA(exStyle, name, title, style, x as u32, y as u32, w as u32, h as u32, parent, hMenu, hInstance, lParam)
    }
    h as HWND
  }
  fn CreateWindowX(&self, parent:HWND)->HWND
  {
    0 as HWND
  }
}


pub struct Dust{
  window_counter:int,
  hInstance:c_int,
  hookId: int,
  widgets:HashMap<HWND, Window>,
}

fn dust()->RefCell<Dust>{
  unsafe{
    let mut d = Dust{
      window_counter:0,
      hInstance:GetModuleHandleW(0 as  * const u16),
      hookId:0,
      widgets: HashMap::new(),
    };
    let mut icex=INITCOMMONCONTROLSEX{dwSize:8,dwICC:16383};
    return RefCell::new(d)
  }
}

pub thread_local!(static TLS_DUST: RefCell<Dust> = dust());

extern "stdcall" fn dust_defWindowProc(hWnd:HWND, msg: u32, wparam: c_int,lparam: c_int)->c_int{
  unsafe{
    match msg{
      130=>{ // WM_DESTROY 所有窗口或者组件销毁之前都会发送这个消息.
        TLS_DUST.with( | d | {
          let mut dust = d.borrow_mut();
            dust.window_counter-=1;
            PostQuitMessage(0);
        });
      },
      _=>{

      }
    }
    DefWindowProcA(hWnd,msg,wparam,lparam)
  }
}
extern "stdcall" fn window_oncreate(code:int,wparam:* const c_void,lparam: * const c_void)->c_int{
  let mut r=0i;
  println!(">>>On create");
  unsafe{
    TLS_DUST.with( | d | {
        if 3 != code{
          return CallNextHookEx(code ,0, wparam,lparam);
        }
        // 跳过 IME 创建.
        if 1 == (65536i & GetClassLongA(wparam as HWND, -26i)){
          return CallNextHookEx(code ,0, wparam,lparam);
        }
        // 修改默认窗口过程，在窗口过程中做消息映射.
        if GetWindowLongA(wparam as HWND, -4) != dust_defWindowProc as int{
          //  SetWindowLongA(wparam as HWND, -4, dust_defWindowProc as int);
        }
        r = CallNextHookEx(code ,0, wparam,lparam);
        UnhookWindowsHookEx(d.borrow().hookId)
    });
  }
  r as c_int
}

fn hookWndCreate(wnd: Box<&Wnd>)
{
  unsafe{
    TLS_DUST.with( | d | {
      let mut dust = d.borrow_mut();
      //dust.widgets.insert(-1i as HWND, wnd);
      dust.hookId = SetWindowsHookExA(5, window_oncreate, 0, GetCurrentThreadId());
    });
  }
}

fn UnHookWndCreate(){
  unsafe{
    TLS_DUST.with( | d | {
        let mut dust = d.borrow_mut();
        UnhookWindowsHookEx(dust.hookId);
        dust.hookId=0;
    });
  }
}
pub fn msgloop()->int
{
  let mut window_counter = 0i;

  TLS_DUST.with( | d | {
    window_counter = d.borrow().window_counter;
  });

  let mut msg = MSG {handle:0 as HWND, msg:0, wparam:0, lparam:0, time:0, pt: POINT{x:0,y:0}};
  while window_counter > 0{
    unsafe{
      while GetMessageW(&mut msg, 0 as HWND ,0, 0) {
        TranslateMessage(&mut msg);
        DispatchMessageW(&mut msg);
      }
    }

    TLS_DUST.with( | d | {
      window_counter = d.borrow().window_counter;
    });
  }
  0
}
