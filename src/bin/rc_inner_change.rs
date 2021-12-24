use std::{cell::{Cell, RefCell}};

#[derive(Debug)]
struct MyStr {
    str: String,
}

#[derive(Debug)]
struct MyCellStr {
    str:Cell<u32>,
}

#[derive(Debug)]
struct MyRefCellStr {
    str:RefCell<String>,
}

fn main() {
    /* =================
    * 可变绑定和不可变绑定
    * ================= */
    //不可变绑定
    let immutBind  = MyStr {
        str: String::from("test immut") 
    };
    //可变绑定
    let mut mutBind  = MyStr {
        str: String::from("test immut") 
    };
    // 不可变绑定不能出借 可变的权限
    // let t = &mut immutBind;
    let t = &mut mutBind;
    //不能改变绑定
    // t = &mut mutBind2;
    t.str.push_str(" world");
    println!("{:?}", t);

    /* =================
    * 内部可变和引用
    * ================= */

    let tc = MyCellStr {
        str:Cell::new(0)
    };

    let tcell = &tc.str;
   
    tcell.set(1);

    println!("{:#?}",tc);
    
    
}