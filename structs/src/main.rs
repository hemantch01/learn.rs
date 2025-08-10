#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle {
    fn calc_area (&self)->u32{
        self.width*self.height
    }
    fn can_hold(&self,other:&Rectangle)-> bool{
        if self.width>other.width && self.height>other.height {
            true
        }
        else{
            false
        }
    }
    fn square(side:u32)->Self{
        Rectangle {
             width: side ,
            height: side
         }
    }
    fn is_square(&self)-> bool{
        self.width==self.height 
    }
}
fn main() {
    let rect1  = Rectangle{
        width:30,
        height:40
    };
    let rect2  = Rectangle{
        width: 40,
        height:40
    };
    let mut s = String::from("hello");
    let sq = Rectangle::square(5);
    
    if sq.is_square() {
        println!("yes i am a square");
    }
    else{
        println!("i am not a square")
    }
    println!("the area of rect{:?} is {}",rect1, rect1.calc_area());
    println!("rect1 hold rect 2 is {} statement",rect1.can_hold(&rect2));
}

