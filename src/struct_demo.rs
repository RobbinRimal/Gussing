struct Student{
    name :String,
    email:String,
    active:bool,
    login_count:u64,

}

//touple struct
struct color{u64,u64,u64}
struct point{i32,i32,i32}

fn main(){
    
  let mut student_1=  Student{
        name:String::from("ross"),
        email:String::from("mailtoxyz@northmail.com"),
        active:true,
        login_count:76
    };
    println!("name:{} email:{} active:{},login:{}"
    ,student_1.name,
    student_1.email,
    student_1.active,
    student_1.login_count
);

student_1.name=String::from("Mike Tison");
student_1.email=String::from("mailtomiketison@gmail.com");

 println!("name:{} email:{} active:{},login:{}"
    ,student_1.name,
    student_1.email,
    student_1.active,
    student_1.login_count
);
let student_2=build_student(String::from("solona"),String::from("solonaxyz@gmail.com"));
println!("name:{} email:{} active:{},login:{}",
    student_2.name,
    student_2.email,
    student_2.active,
    student_2.login_count
);

let student_3=build_student_1(String::from("somnia"),String::from("somniaxyz@gmail.com"));
println!("name:{} email:{} active:{},login:{}",
    student_3.name,
    student_3.email,
    student_3.active,
    student_3.login_count
);

let student_4=Student{
    name:student_1.name,
    ..student_2
};
println!("name:{} email:{} active:{},login:{}",
    student_4.name,
    student_4.email,
    student_4.active,
    student_4.login_count
);


}



fn build_student(name:String,email:String)->Student{
    Student{
        name:name,
        email:email,
        active:false,
        login_count:0

    }
}
fn build_student_1(name:String,email:String)->Student{
    Student{
        name,
        email,
        active:true,
        login_count:69

    }
}