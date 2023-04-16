fn main(){
    // println!("euclides algorithm:");
    // gcd(13980,20939);
    // variables();
    // shadowing();
    // slices()
    string()


}

fn string() {
    let text = "\"dsakjas\" sdl l \"abcdz\n break line\n";
    // like python raw strings
    let path = r"C:\Users\_noob\Desktop\partition.sql";
    //
    let b_txt = b"abc";
    let _str = "teste";

   

    print!("{}",text );
    print!("{}",path );
    println!(r###"
                This raw string started with 'r###"'.
                Therefore it does not end until we reach a quote mark ('"')
                followed immediately by three pound signs ('###'):
            "###);
    for l in b_txt {
        println!("{}", l )

        
    };
    println!("{}", &_str )
    
}

pub fn slices(){
    let _v: Vec<f64> = vec![ 0.1, 0.2, -1.9 ];
    let _a:[f64;4]  = [1.,3.,4.,5.];

    let _sv:&[f64] = &_v;
    let _sa:&[f64] = &_a;

    
    print(&_v);
    print(&_a);
    print(&_a[4..])


    // print!("{}{}",_v,_x)
}

fn print( slc: &[f64] ){
    for n in slc{
        print!("{}\n",n )
    }
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