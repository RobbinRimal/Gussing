#[derive(Debug)]
struct Dimension{
    height:u32,
    width:u32,
}

impl Dimension{
    fn area(&self)->u32{
        self.width*self.height
    }

    fn can_hold(&self,other:&Dimension)->bool{
        self.height>other.height && self.width>other.width
    }
}

impl Dimension{
    fn square(side:u32)->Dimension{
        Dimension{
            height:side,
            width:side,
        }
    }
}
fn main(){
   let dimension1= Dimension{
        height:70,
        width:60,
    };

     let rectangle= Dimension{
        height:30,
        width:50,
    };
     let rectangle1= Dimension{
        height:700,
        width:600,
    };
    let rectangle4=Dimension::square(3);
    
     println!("are of the given dimension form square is  {}",rectangle4.area()) ; 

    println!("can hold the rectangle ::{}",dimension1.can_hold(&rectangle));
    println!("can hold the rectangle ::{}",dimension1.can_hold(&rectangle1));

  println!("{:#?}" ,dimension1);
  let a=dimension1.area();
  println!("are of the given dimension is {}",a) ; 

}

