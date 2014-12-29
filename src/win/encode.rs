#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

use libc::{c_int,c_void};

use super::types::*;
use super::api::*;

//将 UTF8 编码转换为 Unicode 编码
pub fn UTF82UCS2(string: &str)->Vec<u16>
{
  unsafe{
    let l = string.len() as int;
    let sz=MultiByteToWideChar(65001u, 0, string.as_ptr() as * const u8, l, 0 as * const u16, 0);

    if sz > 0{
      let mut out:Vec<u16> = Vec::with_capacity((sz+1) as uint);
        out.set_len((sz+1) as uint);
        let ret = MultiByteToWideChar(65001u,0, string.as_ptr(), l, out.as_mut_ptr(), sz*2 +2);
        //println!("tit={}", out);
        return out;
    }
  }
  Vec::new()
}
