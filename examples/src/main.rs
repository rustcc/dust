extern crate dust;

use dust::win::wnd::{DWnd,TWnd};
use dust::window::window::Window;


//use dust::win::types::{DWnd};
use dust::{msgloop};


fn main()
{
  let win = Window::new(
      "秀语言的 尘土库",
      0,0,640,480,
      0 as DWnd
    );
    win.SetText("test it");
    msgloop();
  /*
  let button = Button::new(main,
    "点击我",
    5,5,120,25
  );
  */
}
