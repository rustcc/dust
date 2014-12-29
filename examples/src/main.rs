extern crate dust;

use dust::window::window::Window;
use dust::widgets::button::Button;

use dust::win::types::{HWND,MSG,WndProc};
use dust::{msgloop};


fn main()
{
   if Window::new(
      "秀语言的 尘土库",
      0,0,640,480,
      None,
  ){
      msgloop();
  }
  /*
  let button = Button::new(main,
    "点击我",
    5,5,120,25
  );
  */
}
