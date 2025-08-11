#[derive(Debug)]
enum Iptype{
    V4,
    V6,
}
struct Ipaddress{
    address:String,
    varient:Iptype,

}
#[derive(Debug)]
enum IpWithStr{
    V4(String),
    V6(String),
}
impl Ipaddress{
    fn new(address: &str)->Self{
        Ipaddress {
             address: address.to_string(),
              varient: Iptype::V4
            }
    }
}

#[derive(Debug)]
struct Ipv4(u8,u8,u8,u8);
struct Ipv6(String);

#[derive(Debug)]
enum IpNew{
    V4(Ipv4),
    V6(Ipv6),
}
fn main() {
    let s = String::from("1.1.1.1");
    route_without_enum(&s);
    // let ip = Ipaddress {
    //     address:String::from("192.178.1.1"),
    //     varient:Iptype::V6,
    // };
    let another_ip = Ipaddress::new("1.2.3.4.5");
    route_with_enum(&another_ip);
    let ipwithout_stuct = IpWithStr::V4(String::from("173.4.5.1"));
    route_without_struct(&ipwithout_stuct);

    let newIpv4Data = IpNew::V4(Ipv4(172, 165, 1, 1));
    let newIpv6Data  = IpNew::V6(Ipv6(String::from("hello")));
}


fn route_without_enum(s:&str){
    println!("this is running on {}",s);
}

fn route_with_enum(ip:&Ipaddress){
    println!("this is running on ip {} is of type {:?}",ip.address,ip.varient);
}

fn route_without_struct(ip:&IpWithStr){
    println!("the ip is {:?}",ip);
}