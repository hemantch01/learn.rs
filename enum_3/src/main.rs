#[derive(Debug)]
enum Iptype{
    V4,
    V6,
}
struct Ipaddress{
    address:String,
    varient:Iptype,

}
fn main() {
    let s = String::from("1.1.1.1");
    route_without_enum(&s);
    let ip = Ipaddress {
        address:String::from("192.178.1.1"),
        varient:Iptype::V6,
    };
    route_with_enum(&ip);
}


fn route_without_enum(s:&str){
    println!("this is running on {}",s);
}

fn route_with_enum(ip:&Ipaddress){
    println!("this is running on ip {} is of type {:?}",ip.address,ip.varient);
}