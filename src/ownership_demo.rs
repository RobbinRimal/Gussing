//Each value in Rust has a variable that’s called its owner.
//There can be only one owner at a time.
//When the owner goes out of scope, the value will be dropped.


fn main(){
     let name= String::from("rust");  
     take_ownership(name);

     let x_val:i32=56789;
     copy_ownership(x_val);
     println!("the value of x:{}",x_val);
     let language:String=give_ownership();
    let result= take_ownership_and_giveback(language);
    println!("final result:{}",result);


    //  println!("the name of the code is {}", name);
    // println!("{}",data());
    // heap_copy();
    // literal_copy();
    
}

fn copy_ownership(value:i32){
    println!("the value of x:{}",value);
}
fn take_ownership(name:String){
    println!("hello,{} !",name);

}

fn take_ownership_and_giveback(name:String)->String{
    name+ &String::from(" king")

}

fn give_ownership()->String{
    let name :String=String::from("rust");
    println!("hello,{} !",name);
    name

}


fn data()->String{
    let mut s:String=String::from("hello ");
    s.push_str(", ross!");
    s
    
}
fn literal_copy(){
    let x="hello world";
    let y=x;
    println!("the value of x :{} and value of y:{}",x,y);
    //this will work because its stored on stack 
    
}

fn heap_copy(){
    let x:String=String::from("lets copy this string");
    //let y:String=x; //this will not work cause it will create race condition 
    let mut y:String=x.clone();
    y.push_str(", ross!");

    //this will work cause it will create brand new clone even on the heap too
    println!("the value of x :{} and value of y:{}",x,y);

    //here changing one didn't affect the other they are two saperate copy on heap!!
}