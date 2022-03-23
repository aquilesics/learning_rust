fn main(){
    println!("euclides algorithm:");
    gcd(13980,20939);

}

pub fn gcd(mut n:u64, mut m:u64 ) {
    assert!(n != 0 && m != 0  );
    while m != 0{
        if m < n{
            let t: u64 = m;
                m = n;
                n = t;
            }
            m = m % n;
            println!("{}",m);
        }
        println!("{}",n);   
    
}
