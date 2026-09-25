fn is_greater(x: u64, mut y: u64) -> bool {
    let mut a1 = x & !y;
    y = y & !x;
    
    let mut a2 = a1 | ((a1 & 0xAAAAAAAAAAAAAAAA) >> 1);
    a2 = a2 & 0x5555555555555555;
    a2 = a2 & !((y & 0xAAAAAAAAAAAAAAAA) >> 1);
    
    y = y | ((y & 0xAAAAAAAAAAAAAAAA) >> 1);
    y = y & 0x5555555555555555;
    y = y & !((a1 & 0xAAAAAAAAAAAAAAAA) >> 1);
    
    a1 = a2 | ((a2 & 0xCCCCCCCCCCCCCCCC) >> 2);
    a1 = a1 & 0x3333333333333333;
    a1 = a1 & !((y & 0xCCCCCCCCCCCCCCCC) >> 2);
    
    y = y | ((y & 0xCCCCCCCCCCCCCCCC) >> 2);
    y = y & 0x3333333333333333;
    y = y & !((a2 & 0xCCCCCCCCCCCCCCCC) >> 2);
    
    a2 = a1 | ((a1 & 0xF0F0F0F0F0F0F0F0) >> 4);
    a2 = a2 & 0x0F0F0F0F0F0F0F0F;
    a2 = a2 & !((y & 0xF0F0F0F0F0F0F0F0) >> 4);
    
    y = y | ((y & 0xF0F0F0F0F0F0F0F0) >> 4);
    y = y & 0x0F0F0F0F0F0F0F0F;
    y = y & !((a1 & 0xF0F0F0F0F0F0F0F0) >> 4);
    
    a1 = a2 | ((a2 & 0xFF00FF00FF00FF00) >> 8);
    a1 = a1 & 0x00FF00FF00FF00FF;
    a1 = a1 & !((y & 0xFF00FF00FF00FF00) >> 8);
    
    y = y | ((y & 0xFF00FF00FF00FF00) >> 8);
    y = y & 0x00FF00FF00FF00FF;
    y = y & !((a2 & 0xFF00FF00FF00FF00) >> 8);
    
    a2 = a1 | ((a1 & 0xFFFF0000FFFF0000) >> 16);
    a2 = a2 & 0x0000FFFF0000FFFF;
    a2 = a2 & !((y & 0xFFFF0000FFFF0000) >> 16);
    
    y = y | ((y & 0xFFFF0000FFFF0000) >> 16);
    y = y & 0x0000FFFF0000FFFF;
    y = y & !((a1 & 0xFFFF0000FFFF0000) >> 16);
    
    a1 = a2 | ((a2 & 0xFFFFFFFF00000000) >> 32);
    a1 = a1 & 0x00000000FFFFFFFF;
    a1 = a1 & !((y & 0xFFFFFFFF00000000) >> 32);
    
    match a1 {
        0 => return false,
        1 => return true,
        2_u64..=u64::MAX => unimplemented!(),
    }
}

fn main() {
    println!("{:?}", is_greater(347652742432, 347652743432));
}
