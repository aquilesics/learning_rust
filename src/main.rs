fn main(){
    // println!("euclides algorithm:");
    // gcd(13980,20939);
    // variables();
    shadowing();

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

pub fn variables(){
    const CONST_PI:f32 = 3.14; //por convensao as constantes sao uper case
    assert_eq!(CONST_PI,3.14);
    println!("{}",&CONST_PI);

    let a = 10;
    let x = a;
    println!("{}",x)

}

pub fn shadowing() {
    let x: i32 = 22;

    let x = x + 8;

    {
        let x = x * 2;
        println!("inner scope {}",x);

    }
    
    println!("{}",x);
}