#[derive(Debug)]


enum IpAddressKind{
    v4(u8,u8,u8,u8),
    V6(String),
}
#[derive(Debug)]
struct IpAddress{
    kind:IpAddressKind,
    address:String,
}

#[derive(Debug)]
enum Month{
    January, February, March, April, May, June, 
    July, August, September, October, November,  
    December
    
}
#[derive(Debug)]
struct Date{
    year:u32,
    month:Month,
    data:u8
}

#[derive(Debug)]
enum Message {
Quit,
Move { x: i32, y: i32 },
Write(String),
ChangeColor(i32, i32, i32),
}

fn main(){
     enum_test();
     enum_date();
     enum_message();
}


fn enum_message(){
    let close =Message::Quit;
    let reposition = Message::Move { x: 300, y: 45 };
    let reason=Message::Write(String::from("the sofa looks nice while moving. "));
    let color=Message::ChangeColor(234,23,56);

     let message_collection = vec![close, reposition, reason, color];
   
     for m in message_collection{
        match m{
            Message::Quit=>println!("closing the app"),
            Message::Move { x, y }=>println!("moving on x-coordinate:{} and y-coordinate :{} ",x,y),             Message::Write(x)=>println!("message:{}",x),
            Message::ChangeColor(r, g, b) => println!("Changing color to: ({}, {}, {})", r, g, b),
           _=>println!("error!")
            
    }
    }
}

fn enum_test(){
    let v4_address:IpAddressKind=IpAddressKind::v4(127 , 0 , 0 , 1 );
    let v6_address:IpAddressKind=IpAddressKind::V6(String::from("::1"));
    
     println!("{:#?}" ,v4_address);
     println!("{:#?}" ,v6_address);


     let Ip_address_log=IpAddress{
         kind:v4_address,
         address:String::from("address:localhost"),

     };
      let Ip_address_log1=IpAddress{
         kind:v6_address,
         address:String::from("address:localhost"),

     };
      let v6_google=IpAddress{
         kind:IpAddressKind::V6(String::from("2001:4860:4860::8844")),
         address:String::from("address:google"),

     };

      let v4_google=IpAddress{
         kind:IpAddressKind::v4(8 , 8 , 8 , 8),
         address:String::from("address:google"),

     };
      println!("{:#?}" ,Ip_address_log);
      println!("{:#?}" ,Ip_address_log1);
      println!("{:#?}" ,v6_google);
      println!("{:#?}" ,v4_google);
}

fn enum_date(){

     let Jan:Month=Month::January;
    let dob=Date{
    year:1946,
    month:Jan,
    data:15,
    };
     
    let marrige=Date{
    year:1970,
    month:Month::March,
    data:26,
    };
    let death=Date{
    year:2015,
    month:Month::November,
    data:6,
    };
    println!("{:#?}" ,dob);
    println!("{:#?}" ,marrige);
    println!("{:#?}" ,death);

}