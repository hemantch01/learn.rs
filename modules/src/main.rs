use std::io;

#[derive(Debug)]
enum Coins {
    OneRs,
    TwoRs,
    FiveRs,
    TenRs
}
fn main() {

// lets study some mod, pub , fn 


    //    how to use modules and if let 
//     course of action i have to use if let statement 
    let rupees = Option::Some(Coins::OneRs);
    let value =  match rupees {
         Some(Coins::OneRs) => 1,
         Some(Coins::FiveRs) => 5,
         _=> 0,
     };
     println!("{}",value);
    let mut rs = String::new();
    io::stdin().read_line(&mut rs).expect("msg"); // returns number of bytes returned

    let _s = match rs.trim().parse::<u32>(){
        Ok(i)=> i,
        Err(err)=> {
            println!("there is error in this {} ",err);
            0
        },
    };

    let rs= match rs.trim().parse::<u32>().ok(){
        Some(int)=> int,
        None => 0,
    };

    println!("{rs}") ;



//    if let syntax 
  // make a option enum of i32 
   let s: Option<i32> = Option::Some(32);

   if let Some(v) = s {
    println!(" there is valid value here{v}");
   } 
   else {
    println!("'nothing here '")
   }


    }
