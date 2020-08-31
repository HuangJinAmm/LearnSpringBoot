fn do_n_queen(count:&mut u32, size:usize,level:usize,mut q:[u32;32],uplim:u32,row:u32, ld:u32,rd:u32){
    if row != uplim {
        // *count += 1;
        let mut pos = uplim & !(row|ld|rd);
        while pos != 0 && level <= size{
            let p = pos&(!pos+1);
            q[level] = p;
            pos -= p;
            do_n_queen(count,size,level+1,q,uplim,row+p,(ld+p)<<1,(rd+p)>>1);
        }
    }else{
        // for r in 0..size {
        //     println!("{:>0width$b}",q[r],width=size);
        // }
        // println!("==================================");
        *count += 1;
    }
}

#[warn(dead_code)]
fn n_queen(size:usize){
    let qlist:[u32;32] = [0u32;32];
    let stats:u32 = ((1u64<<size)-1) as u32;
    let row:u32 = 0;
    let ld:u32 = 0;
    let rd:u32 = 0;
    let mut count = 0u32;
    do_n_queen(&mut count,size,0,qlist,stats,row,ld,rd);
    println!("{}",count);

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_n_q() {
        n_queen(16);
    }
}