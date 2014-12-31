#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(unused_imports)]

use libc::c_void;

extern "stdcall"{
  //Encoding
  pub fn MultiByteToWideChar(codePage:uint, flags: u32, raw:* const u8,  len_raw: int, out: * const u16, len_buff: int)->int;
  pub fn WideCharToMultiByte(codePage:uint, flags: u32, raw:* const u16,  len_raw: int, out: * const u8, len_buff: int,def:* const c_void,useDef:bool)->int;
}

//将 UTF8 编码转换为 Unicode 编码
pub fn UTF82UCS2(string: &str)->Vec<u16>
{
  unsafe{
    let l = string.len() as int;
    let sz=MultiByteToWideChar(65001u, 0, string.as_ptr() as * const u8, l, 0 as * const u16, 0);

    if sz > 0{
      let mut out:Vec<u16> = Vec::with_capacity((2*(sz+1)) as uint);
        out.set_len((sz) as uint);
        let ret = MultiByteToWideChar(65001u,0, string.as_ptr(), l, out.as_mut_ptr(), sz*2 +2);
        out.push(0);
        return out;
    }
  }
  vec![0u16]
}

pub fn UCS2TOUTF8(arg:&Vec<u16>)->String
{
  let l = arg.len() as int;
  unsafe{
    let sz = WideCharToMultiByte(65001u,0, arg.as_ptr(), -1, 0 as * const u8, 0, 0 as * const c_void, false);
    if sz > 0{
        let mut out= String::with_capacity((sz+1) as uint);
        WideCharToMultiByte(65001u,0, arg.as_ptr(), l, out.as_ptr(), sz, 0 as * const c_void, false);
        out.as_mut_vec().set_len((sz) as uint);
        println!("need size={}, {}",sz,out.as_slice());
        return out;
    }
  }
  "".to_string()
}
