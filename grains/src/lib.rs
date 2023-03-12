pub fn validate_square(s: u32) -> () 
{
    if s < 1 || s > 64 
    {
        panic!("Square must be between 1 and 64");
    }
}

pub fn square(s: u32) -> u64 
{
    validate_square(s);
    let base : u64 = 2;
    let grains : u64 = base.pow(s - 1);
    grains
}

pub fn total() -> u64 
{
    let mut sum : u64 = 1;
    let base : u64 = 2;
    for i in 1..64
    {
        sum += base.pow(i) as u64;
    }
    sum
}
