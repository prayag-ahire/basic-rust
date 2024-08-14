use std::{f32::consts::PI, f64::consts, fmt::Debug, fs, result};


fn main() {

    // making varibles 
    let num: i8 = 127; //-128 to 127 //u8 0 to 255
    let num2: f32 = 1052.44; 
    let mut _num3: i8 = 100; // mutable varible where you can change the var value
    println!("Hello, world! {} this is a secand number {}",num,num2);

    _num3 = 15;
    println!("{}",_num3);


    // loops 
    for _i in 0..10{
        _num3 = _num3 + 1;
    }
    print!("hii{}",_num3);

    //boolen varible
    let is_good: bool = true;
    let is_bad: bool = false;

    // conditional statment is normal ok 

    if is_good {
        println!("this is a good idea {}",_num3);
    }
    else{
        println!("this is very bad");
    }

    if is_good && is_bad{
        println!("both is good byy..");
    }


    //this is how string define and be print
    let greeting = String::from("hello world");
    println!("{}",greeting);

    // here the varible is maybe string or maybe not
    let char1 = greeting.chars().nth(2);


    //this is good method to print string if exist
    match char1 {
        Some(c)=>print!("{}",c),
        None=>print!("no charater at index 1000"),
    }

    // from this you can also print string  from varible
    print!("{}",char1.unwrap());


    // making string and function to print string where space identifi 
    let santance  = String::from("my name is prayag");
    let first_word = get_first_word(santance);
    print!("first word is : {}",first_word);
    
    let  x = 0;
    let mut s1 = String::from("hello world");
    for x in 0..60{
        s1.push_str("hii hwo  arre you");
        println!("capacity:{}, length:{}, pointer{:p}",s1.capacity(),s1.len(),s1.as_ptr());
    }
    let mut my_string = String::from("hello");
    my_string = function_string(my_string);
    print!("{}",my_string);
    // so in rust you can not point to same heap memory eith two variblse 

    //rafreance this how it work (s2 -> s1 -> heap)
    let mut s1 = String::from("hello how are you?");
    let s2 = &s1;
    let s3 = &s1;
    let s4 = &s1;
    // you can have multiple borrower when you don't have some hamki panki borrower

    println!("{}",s2);
    println!("{}",s3);
    println!("{}",s4);
    
    let ans2 = &mut s1;
    let _ans1 = function_string1(&mut s1);
    println!("{}",s1);
    println!("{}",_ans1);
    // println!("{}",ans2);
    // print!("{}",_ans);

    // print!("{}",s2);

    // println!("{}",s2);

    let s2 = &mut s1;
    s2.push_str(" hello this is new ");
    println!("{}",s2);
    println!("{}",s1);
    
    // let s5 = &s1;
    //you can do this when you borro var do somthing and unborro

    // println!("{}",s1);
    // println!("{}",s2);
    //you can't do this , you can't  do somthing when you use owner so imidetly tranfer ownership




    let user1 = function_struct();
    println!("{} is {} year old and avilable {}",user1.name,user1.print_detail(),user1.avilabe);

    //tuple struct and unite struct
    //lifetime 
    // string slices(&str)
    //traits


    let user2 = my_details{
        name:String::from("prayag ahire"),
        age:21,
        avilabe:true
    };

    println!("this is a new struct {} and{}",user2.age,user2.print_detail());
    println!("is this  year old and avilable {:?}",user2);


    move_arounnd(Direction::left);
    


    //enum with values function
    let circal = shape::Circal(5.0);
    let square = shape::Square(6.0);
    let rect = shape::Ractangle(4.0,5.0);



    println!("calculate area : {}",calculate_area(rect));



// error handaling
let res = fs::read_to_string("exaple.txt");
match res {
    Ok(content) => println!("this is a {}",content),
    Err(err) => println!("this is a good error {}",err),
}

// res.unwrap();

println!("hii i am after the  read file");
}



// function for string
fn get_first_word(santance: String) -> String{
    let mut ans = String::new();
    for char in santance.chars(){
        // ans.push_str(char.to_string().as_str());
        ans.push(char);
        if char == ' '{
            break;
        }
    };
    return ans;
}

// when varible pass from main function two other function it will pass ownershipe so the
//var in main function no longer have any value.
fn function_string(my_string: String)-> String{
    print!("{}",my_string);
    return my_string
}

fn function_string1(s: &mut String) -> String{
    s.push_str("prayag is here");
    return s.to_string();
}


// here you can create struchar you can create varible
struct my_details{
    name: String,
    age: u8,
    avilabe:bool
}

// here you can create function for struct and you can use struct var
// this will be use for multiple same time of struct
impl Debug for my_details {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"the ractangle prints like this {}",self.age * 12)
    }
}

impl  my_details {
    fn print_detail(&self)->u8{
        return self.age + 10;
    }
}

//here i have created like similar but it is spacific not genric 
//here i have created struct inside of function
fn function_struct() -> my_details{
let user = my_details{
    name: String::from("prayag"),
    age: 20,
    avilabe: true
};

return user;
}



//this is enum one time one value can be access
enum Direction{
    left,
    right,
    up,
    down
}


fn move_arounnd(direcation:Direction){

    //you can also use if else if   
   match direcation {
       Direction::down => println!("this is a down"),
       Direction::left => println!("this is a left"),
       Direction::right => println!("this is a right"),
       Direction::up => println!("this is a up")
   }
}





enum shape{
    Circal(f64),
    Square(f64),
    Ractangle(f64,f64),
}

fn calculate_area(shape: shape) ->  f64{
    //pattern matching in enum
    match shape {
        shape::Circal(radius) => std::f64::consts::PI * radius * radius,
        shape::Square(length) => length * length,
        shape::Ractangle(height,width) => height * width,
    }
}

fn read_from_file_if_else(file_content: String) -> Result<String, String>{
    let res = fs::read_to_string(file_content);

    if let Ok(content) =res{
        Ok(content)
    }else{
        return Err("error reading file".to_string());
    }
}