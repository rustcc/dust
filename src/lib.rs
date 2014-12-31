#![allow(non_snake_case)]
#![feature(globs)]
#![allow(dead_code)]

extern crate libc;

use win::types::{LPARAM,WPARAM,MSG,HINSTANCE};
use win::api::*;
use win::encode::*;
use win::wnd::{TWnd,DWnd};
use event::eventlistener::{TEventProcesser,EventProcesser};
use win::types::{
  INITCOMMONCONTROLSEX,C_NULL,WndProc,POINT,
  PostQuitMessage,PostMessageW
};
use libc::{c_void,c_int};

use std::collections::HashMap;

use std::rc::Rc;
use std::cell::{RefCell};
use std::mem;

pub mod win;
pub mod window;
pub mod widgets;
pub mod event;


static NULL_DWnd:DWnd = 0 as DWnd;


pub extern "stdcall" fn emptyWndProc(_a:DWnd,_b: u32,_c: WPARAM,_d: LPARAM)->c_int{0}
pub struct Dust{
  window_counter:int,
  hInstance:HINSTANCE,
  hookId: int,
  sysFont:c_int,
  widgets:HashMap<DWnd, Rc<RefCell<Box<TEventProcesser + 'static>>>>,
}

impl Dust {

  fn new() -> Dust {

    let icex=INITCOMMONCONTROLSEX{dwSize:8,dwICC:16383};
    let mut hInst = C_NULL as HINSTANCE;
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
extern "stdcall" fn dust_defWindowProc(hWnd:DWnd, msg: u32, wparam: WPARAM,lparam: LPARAM)->c_int{
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
              ret = (*wnd.as_unsafe_cell().get()).msgProcedure(hWnd,msg,wparam,lparam);
              (*d.as_unsafe_cell().get()).widgets.remove(&hWnd);
            };
          },
          _=>{
            //进入到对象内部的窗体过程.
            recvied = true;
            ret = unsafe{(*wnd.as_unsafe_cell().get()).msgProcedure(hWnd,msg,wparam,lparam)};
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
#[cfg(all(windows, target_word_size = "64"))]
fn set_window_proc(hWin:DWnd,callback:|WndProc|){
  unsafe{
    if GetWindowLongPtrW(hWin, -4) != dust_defWindowProc as * const c_void{
      callback(mem::transmute(SetWindowLongPtrW(hWin, -4, dust_defWindowProc as * const c_void)));
    }
  }
}

#[cfg(all(windows, target_word_size = "32"))]
fn set_window_proc(hWin:DWnd,callback:|WndProc|){
  unsafe{
    if GetWindowLongW(hWin, -4) != dust_defWindowProc as * const c_void{
      callback(mem::transmute(SetWindowLongW(hWin, -4, dust_defWindowProc as * const c_void)));
    }
  }
}

extern "stdcall" fn window_oncreate(code:int,wparam:* const c_void,lparam: * const c_void)->c_int{
  let mut r=0i;

  println!(">>>>>> Set Window Long ....{}",wparam);
  unsafe{
    TLS_DUST.with( | d |->int {
        if 3 != code{
          return CallNextHookEx(code ,0, wparam,lparam);
        }
        // 跳过 IME 创建.
        if 1 == (65536i & GetClassLongA(wparam as DWnd, -26i)){
          return CallNextHookEx(code ,0, wparam,lparam);
        }

        (*d.as_unsafe_cell().get()).window_counter+=1;
        let w = (*d.as_unsafe_cell().get()).widgets.remove(&NULL_DWnd);

        match w{
          Some(wnd)=>{
            let mut window = wnd.borrow_mut();
            window.setHwnd(wparam as DWnd); //存储句柄.
            // 修改默认窗口过程，在窗口过程中做消息映射.
            PostMessageW (wparam, 48, (*d.as_unsafe_cell().get()).sysFont as WPARAM, 1 as LPARAM);
            set_window_proc(wparam as DWnd,|w|
              window.setWndProc(w)
            );
/*
            if GetWindowLongPtrW(wparam as DWnd, -4) != dust_defWindowProc as * const c_void{
                //window.setwndProc(SetWindowLongPtrW(wparam as DWnd, -4, dust_defWindowProc as * const c_void));
                println!(">>>>>> Set Window Long ....{}",wparam);
            }

            // 修改默认窗口过程，在窗口过程中做消息映射.
            //#[cfg(target_word_size = "32")]
            if GetWindowLongW(wparam as DWnd, -4)  != dust_defWindowProc as * const c_void{
              let ptr = SetWindowLongW(wparam as DWnd, -4, dust_defWindowProc as * const c_void);
              //window.setwndProc( );
              println!(">>>>>> Set Window Long ....{}",wparam);
            }
*/
            (*d.as_unsafe_cell().get()).widgets.insert(wparam as DWnd, wnd.clone());
          },
          _=>{}
        }

        r = CallNextHookEx(code ,0, wparam,lparam);
        UnhookWindowsHookEx((*d.as_unsafe_cell().get()).hookId);
        //dust.hookId=0;
        0
    });
  }
  r as c_int
}

fn hookWndCreate(wnd :Box<TEventProcesser + 'static>)
{
  let r = Rc::new(RefCell::new(wnd));
  unsafe{
    TLS_DUST.with( | d | {
        let mut dust = d.borrow_mut();
        println!("Set Hook......");
        dust.widgets.insert(NULL_DWnd, r.clone());
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

  let mut msg = MSG {handle:0 as DWnd, msg:0, wparam:0, lparam:0, time:0, pt: POINT{x:0,y:0}};
  while window_counter > 0{
    unsafe{
      while msg.GetMessage(0 as DWnd ,0, 0) {
        let win = msg.handle;

        //如果窗口没有处理该消息,直接dispatch吧//
        if win.processMsg(&msg)
        {
          msg.TranslateMessage();
          msg.DispatchMessage();
        }
      }
    }

    TLS_DUST.with( | d | {
      window_counter = d.borrow().window_counter;
    });
  }
  0
}

pub fn MessageBox(title: &str,text:&str,flags:int)->int{
  let r;
  unsafe{
    let h = GetActiveWindow();
    r = MessageBoxW(h, UTF82UCS2(text).as_ptr(), UTF82UCS2(title).as_ptr(), flags as u32);
  }
  r
}
