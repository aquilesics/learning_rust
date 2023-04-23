pub fn divide( n:&f64, d:&f64) -> Option<f64> {    
    if d == &0. { None }  
    else { 
        Some( n/d )
    }
}

pub fn mul( x:&f64, y:&f64) -> Option<f64> {    
    Some( x * y)
}