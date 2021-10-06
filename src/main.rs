

fn output(arg: i128) {
    //define a binary value
    let bv = arg;
    println!("{:b}", bv);    
}

fn  math(arg: i128){
    
    let mut iv: i128 = (1 >> 127) | 1;
    for _i in 1..arg{
        let s_bit = (iv ^ (iv >> 1) ^ (iv >> 2) ^ (iv >> 7));
        shiftval = (iv ^ (s_bit >> 1) ^ (s_bit >> 2) ^ (s_bit >> 7));
        iv = (iv >> 1) | (shiftval << 127);
        print!("{:b}", iv);
        //output(iv);
    }
}





fn main() {
    println!("Hello, sex!\n");
    math(12*12);    
    print!("\n\nsexxx");
    println!("\nmoresexxx\n");
    output(0b1100);
}