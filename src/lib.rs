#[macro_export]
macro_rules! main {
    ($($x:tt)+) => {
        fn main(){
            let now = std::time::Instant::now();
            let (p1, p2) = {$($x)+};
            let time = now.elapsed().as_millis();
            println!("Part one: {}", p1);
            println!("Part two: {}", p2);
            println!("Time: {} ms", time);
        }
    };
}
#[macro_export]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

pub mod utils;
