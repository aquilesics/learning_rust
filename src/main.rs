#![allow(dead_code)]
fn main(){
    // println!("euclides algorithm:");
    // gcd(13980,20939);
    // variables();
    // shadowing();
    // slices()
    // string()
    // onwership()
    lifetimes()
}

fn lifetimes(){
    struct S<'a> {
        y: &'a i32 ,
        x: &'a i32
    }

    fn f<'a>(r:&i32, s:&S ) -> i32{
        r + s.x + s.y
    }

    let a:i32 = 40;
    let u = S{ x:&10,
                  y:&20
                };

    let b = f( &a, &u );
    
    print!("{}",&b );
    
}


fn onwership(){
    {
        let point = Box::new((0.25,2.22));//alloc here
        let label = format!("{:?}",point); //alloc here too

        assert_eq!(label,"(0.25, 2.22)")// true
    }
    //label and point are dropped here
    
    struct Person {
        name:String,
        age:u32
    }

    let mut composers = Vec::new();

    composers.push(Person { name: "Miles Davis".to_string(), age:65 } );
    composers.push(Person { name: "John Coltrane".to_string(), age:40 } );


    for c in &composers{
        println!("{} with age: {}",c.name, c.age)
    }


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