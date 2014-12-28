extern crate dust;

use dust::window::window::Window;
use dust::widgets::button::Button;

use dust::win::types::{HWND,MSG,WndProc};
use dust::{msgloop};


fn main()
{
  let main = Window::new(
      "Dust for rust example",
      0,0,800,600,
      None
  );
  let button = Button::new(&main,
    "点击我",
    5,5,120,25
  );
  msgloop();
}
