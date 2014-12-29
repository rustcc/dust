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

use std::rc::Rc;
use std::cell::{RefCell,UnsafeCell};

pub mod win;
pub mod window;
pub mod widgets;


static NULL_HWND:HWND = 0 as HWND;

pub trait Wnd{
//  fn getSelf(&mut self)->&mut Self;
  fn preTranslate(&self,_hWnd: HWND,_msg:& mut MSG)->bool{
    true
  }
  fn getWndProc(&self)->WndProc;
  fn setHwnd(&mut self,h: HWND){}
  fn getHwnd(&self)->HWND{NULL_HWND}
  fn setwndProc(&mut self,p: WndProc){}

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
}

pub extern "stdcall" fn emptyWndProc(_a:HWND,_b: u32,_c: c_int,_d: c_int)->c_int{0}
pub struct Dust{
  window_counter:int,
  hInstance:c_int,
  hookId: int,
  sysFont:c_int,
  widgets:HashMap<HWND, Rc<RefCell<Box<Wnd + 'static>>>>,
}

impl Dust {

  fn new() -> Dust {

    let icex=INITCOMMONCONTROLSEX{dwSize:8,dwICC:16383};
    let mut hInst = 0 as c_int;
    let font;
    unsafe{
      font= GetStockObject(17);
      InitCommonControlsEx(&icex);
      hInst = GetModuleHandleW(0 as  * const u16);
    }

    Dust{
      window_counter:0,
      hInstance:hInst,
      hookId:0,
      sysFont:font,
      widgets: HashMap::new(),
    }
  }

  fn dust() -> RefCell<Dust> {
    RefCell::new(Dust::new())
  }
}

pub thread_local!(static TLS_DUST: RefCell<Dust> = Dust::dust());

// 所有窗体，组建 都要进入到这个消息过程，
// 它负责将事件映射到对象自身的窗体中.
extern "stdcall" fn dust_defWindowProc(hWnd:HWND, msg: u32, wparam: c_int,lparam: c_int)->c_int{
  let mut recvied = false;
  let mut ret = 0i;
  TLS_DUST.with( | d | {
    let w  = unsafe{ (*d.as_unsafe_cell().get()).widgets.get(&hWnd)};
    match w {
      Some(wnd)=>{
        match(msg){
          130=>{ //WM_DESTROY
            unsafe{
              (*d.as_unsafe_cell().get()).window_counter-=1;
              PostQuitMessage(0);
              recvied = true;
              ret = unsafe{(*wnd.as_unsafe_cell().get()).wndProc(hWnd,msg,wparam,lparam)};
              (*d.as_unsafe_cell().get()).widgets.remove(&hWnd);
            };
          },
          _=>{
            //进入到对象内部的窗体过程.
            recvied = true;
            ret = unsafe{(*wnd.as_unsafe_cell().get()).wndProc(hWnd,msg,wparam,lparam)};
          }
        };
      },
      _=>{}
    };
  });
  if !recvied{
    unsafe{
      ret = DefWindowProcW(hWnd,msg,wparam,lparam) as int;
    }
  }
  ret as c_int
}
extern "stdcall" fn window_oncreate(code:int,wparam:* const c_void,lparam: * const c_void)->c_int{
  let mut r=0i;

  unsafe{
    TLS_DUST.with( | d |->int {
        if 3 != code{
          return CallNextHookEx(code ,0, wparam,lparam);
        }
        // 跳过 IME 创建.
        if 1 == (65536i & GetClassLongA(wparam as HWND, -26i)){
          return CallNextHookEx(code ,0, wparam,lparam);
        }
        let mut dust = d.borrow_mut();

        dust.window_counter+=1;
        let w = dust.widgets.remove(&NULL_HWND);

        match w{
          Some(wnd)=>{
            let mut window = wnd.borrow_mut();
            window.setHwnd(wparam as HWND); //存储句柄.
            // 修改默认窗口过程，在窗口过程中做消息映射.
            PostMessageW (wparam, 48, dust.sysFont, 1);
            if GetWindowLongW(wparam as HWND, -4) != dust_defWindowProc as int{
              window.setwndProc(SetWindowLongW(wparam as HWND, -4, dust_defWindowProc as int));
              //println!(">>>>>> Set Window Long ....");
            }
            dust.widgets.insert(wparam as HWND, wnd.clone());
          },
          _=>{}
        }

        r = CallNextHookEx(code ,0, wparam,lparam);
        UnhookWindowsHookEx(dust.hookId);
        //dust.hookId=0;
        0
    });
  }
  r as c_int
}

fn hookWndCreate(wnd :Box<Wnd + 'static>)
{
  let r = Rc::new(RefCell::new(wnd));
  unsafe{
    TLS_DUST.with( | d | {
        let mut dust = d.borrow_mut();
        dust.hookId = SetWindowsHookExA(5, window_oncreate, 0, GetCurrentThreadId());
        dust.widgets.insert(NULL_HWND, r.clone());
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
