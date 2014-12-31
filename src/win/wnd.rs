#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use libc::{c_int,c_uint, c_void};
use super::encode::{UTF82UCS2,UCS2TOUTF8};
use super::types::{MSG};

pub type DWnd = * const c_void;

extern "stdcall"{
  //Window Text
  pub fn GetWindowTextLengthW(hWnd:DWnd)->c_int;
  pub fn GetWindowTextW(hWnd:DWnd,lp:*const u16,cch:u32);
  pub fn SetWindowTextW(hwnd:DWnd, text: * const u16);
}

pub trait TWnd
{
   // 返回真表示消息没被处理.
   // 它负责根据哈希表，找到对应的窗体的事件处理器.
   fn processMsg(&self, msg:& MSG)->bool{
     true
   }
    /*
      获取窗体文本
    */
    fn GetText(&self)->String;
    fn SetText(&self, text:&str);
}

impl TWnd for DWnd{
  // 返回真表示消息没被处理.
  // 它负责根据哈希表，找到对应的窗体的事件处理器.

  /*
  获取窗体文本
  */
  fn GetText(&self)->String{
    let sz;
    unsafe{
      sz = GetWindowTextLengthW(*self);
      let mut vec:Vec<u16> = Vec::with_capacity((1 +sz) as uint);
      vec.set_len((sz) as uint);
      GetWindowTextW(*self,vec.as_ptr(), ((1 +sz) *2) as u32);
      vec.push(0);
      println!("vec sz={} raw={}",sz, vec);
      return UCS2TOUTF8(&vec);
    }
    "".to_string()
  }

  fn SetText(&self, text:&str){
    unsafe{
      SetWindowTextW(*self,UTF82UCS2(text).as_ptr());
    }
  }
}
