#![allow(non_snake_case)]
#![allow(unused_variables)]

use super::super::win::types::{WndProc,MSG,WPARAM,LPARAM};
use super::super::win::wnd::DWnd;
use super::super::win::api::{CallWindowProcW};
use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::HashMap;

/*
	每一个窗口都有一个自有的事件处理器.它们通过哈希表关联在一起.
*/
pub struct EventProcesser{
	pub defWindowProc:WndProc, //默认窗口过程. 当所有事件处理完毕，该过程将会被调用.
	//事件映射表,任意的.
//	pub events:HashMap<u32,RefCell<Box<Event + 'static>>>,	//事件对象表

	//WM_NOTIFY
//	notifers:HashMap<u32,RefCell<Box<Event + 'static>>>,	//通知事件对象表.

	//WM_COMMAND
//	commands:HashMap<u32,RefCell<Box<Event + 'static>>>,	//菜单事件对象表.
}

pub trait TEventProcesser
{
	// 消息进入队列之前.在这里处理.
	// 处理了 返回 false 返回 true 表示调用者需要自己处理消息.
	// 如果是窗体，且自己有快捷键表......
	fn preTranslateMsg(&self,msg:&mut MSG)->bool{
		true
	}
	fn setWndProc(&mut self,wproc:WndProc){}
	fn getWndProc(&self)->WndProc;

	fn setHwnd(&mut self, hWin:DWnd){}
	// 对象自有消息过程
	fn msgProcedure(&self,hWin:DWnd, msg:u32,wparam:WPARAM,lparam:LPARAM)->int{
		unsafe{
			return CallWindowProcW(self.getWndProc(), hWin, msg, wparam, lparam) as int;
		}
	}
}


trait Event{
	fn addEventEventListener(&self, evCallback:||->bool)->bool{
			true
	}
}


/*
	trait Button for HWND{
		fn Button(...)->HWND
		{
		let hWin;
			let mut event = WindowWdgetsProcesser::new();
			hookEventCreate(event)
			HWND = CreateWindowEx(.....);
			unHookEventCreate();
			hWin
		}
	}
	trait Window for HWND{
		fn Window(title:&str, x:int,y:int, w:int,h:int)->HWND
		{
			let hWin;
			let mut event = WindowEventProcesser::new();
			hookEventCreate(event)
			HWND = CreateWindowEx(.....);
			unHookEventCreate();
			hWin
		}
	}
	let window  = Window::Window("Hello Rust",style,0,0,400,560,|e|{
		// onCreate
	});
	let button = Button::new("Click Me", 0 , 0, 100, 40);

	window.on(EventType::Created,|e|{
		// you event code is here....
	});
	window.on(EventType::Size,|e|{
		// you code is here....
	});

	window.onCommand(button.getId(), |e|{
		// button clicked.
	});

*/
