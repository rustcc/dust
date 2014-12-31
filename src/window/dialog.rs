#![allow(non_snake_case)]

/*
use super::super::win::types::*;
use super::super::win::api::*;
use super::super::Wnd;


// 所有窗口 组件 都必须实现的接口。
// 部分方法 preTranslate wndProc 消息映射需要用到.

struct Dialog{
  hWnd: HWND,
  wndProc: WndProc
}


impl Wnd for Dialog{
//  fn getSelf(&mut self)->&mut Self{self}
  fn preTranslate(&self,hWnd: HWND,msg:& mut MSG)->bool
  {
      if msg.msg == 256 && msg.wparam == 27 {
        msg.handle = hWnd;
        return false;
      }
      unsafe{
        if  msg.IsDialogMessage(hWnd) {
          msg.TranslateMessage();
          msg.DispatchMessage();
        }
      }
      false
  }
  fn getWndProc(&self)->WndProc{
    self.wndProc
  }
}

*/
