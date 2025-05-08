fn main(){
let text:String=String::from("I love to program on rust language.");

let ( text,value)=calculate_length(text);
println!("text:{}  value:{}",text,value);

//lets do the above task with refrence 

let  text1:String=String::from("I am just a regular kid form Nepal!!");
let value1:usize=calculate_text_length(&text1);
println!("text:{}  value:{}",text1,value1);



let mut x:String=String::from("this is america");
println!("{}",update(&mut x));

let girl=String::from("girl are the most amazing things in creation which are equally misterious.");
let words=first_word(&girl);

println!("this is the first word:{}",words);

}
fn update(text :&mut String)->String{
    text.push_str(" and its president is Trump.");
    text.clone()
}

fn calculate_length(text:String)->(String,usize){
   let val= text.len();
   (text,val)
}

fn calculate_text_length(text:&String)->usize{
    text.len()
    
 }
 
 fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
    return &s[0..i];
    }
    }
    &s[..]
    }