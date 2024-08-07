use rand::Rng;
use std::io::*;


fn main() {
    let secret_numb = rand::thread_rng().gen_range(1, 101);
   
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn fun1() -> Result {
    Ok(())

    
}

fn fun2() -> IoResult<()> {
    Ok(())
}
