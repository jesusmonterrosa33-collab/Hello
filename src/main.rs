// #[allow(unused_variables)]
// fn main() {
//     let x: i32 = 5;

//     {
//         let x = 12;
//         assert_eq!(x,12);
//     }
//     assert_eq!(x,5);

//     let x: i32 = 42;
//     println!("{}",x);
// }
// #[allow(unused_variables)]

// fn main(){
//     let mut x: i32;
//     x = 7;

//     // shadowing and re-binding
//     x += 3;

//     let y: i32 = 4;

//     // shadowing
//     let y: &str  = "I can also bound to text!";

//     println!("Success!");
    
// }

// Fix the error below with least amount of modifications

// fn main() {
//     let ( mut x, y) = ( 1 , 2);

//     x += 2;

//     assert_eq!(x,3);
//     assert_eq!(y,2);

// }



// fn main() {
//     let (x, y);

//     (x,..) = (3, 4);
//     [..,y] = [1, 2];

//     assert_eq!([x,y],[3,2]);

//     println!("Success!");
// }


// fn main(){

//     let x: i32 = 5;
//     let mut y: i32 = 5;

//     y = x;

//     let z: i32 = 10; // Type of z

//     println!("Success!");

// }


// fn main(){
//     let v: u16 = 36_u8 as u16;

//     println!("Success!");

// }

// Modify assert_eq! to make it work

// fn main(){
//     let x : u32 = 5;
//     assert_eq!("u32".to_string(), type_of(&x));
//     println!("Success!");
// }
// // Get the type of given variable, return a string represrnation of the type
// fn type_of<T>(_:&T) -> String{
//     format!("{}", std::any::type_name::<T>())
// }

// Fill the blank to make it work

// fn main(){
//     assert_eq!(i8::MAX,127);
//     assert_eq!(u8::MAX,255);

//     println!("Success!");
// }

// fn main(){
//     let v1 = 251_u16 + 8;
//     let v2 = i16 :: checked_add(251, 8).unwrap();

//     println!("{},{}",v1,v2);
// }

// fn main(){
//     assert!(0.1_f32+0.2_f32==0.3_f32);

//     println!("Success!");
// }

// fn main(){
//     let mut sum: i32 = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z'{
//         println!("{}",c as u8) ;
//     }
// }

// use std::ops::{Range, RangeInclusive};
// fn main(){
//     assert_eq!((1..5), Range{start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));

//     println!("Success!");
// }

// fn main(){
//     // integer additon
//     assert!(1_u32 + 2_u32 == 3);

//     //Integer subtraction
//     assert!(1_i32 - 2_i32 == -1);
//     assert!(1_i32 - 2_i32 == -1);

//     assert!(3*50 == 150);

//     assert!(9.6_f32 / 3.2_f32 == 3.0_f32) ;

//     assert!(24%5 == 4);

//     // short-circuiting boolean logic

//     assert!( true && false == !true);
//     assert!(true || false == true);
//     assert!(!true == false);

//     //Bitwise operations

//     println!("0011 AND 0101 is a {:04b}",0b0011u32 & 0b0101u32);
//     println!("0011 OR 0101 is a {:04b}",0b0011u32 | 0b0101u32);
//     println!("0011 XOR 0101 is a {:04b}",0b0011u32 ^ 0b0101u32);
//     println!("1<<5 is {}",1u32<<5 );
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

// char,bool and unit

// use std::mem::size_of_val;

// fn main(){
//     let c1: char = 'a';
//     assert_eq!(size_of_val(&c1),4);

//     println!("Success!");
// }\


// fn main(){
//     let _f: bool = false;

//     let t: bool = false;

//     if !t{
//         println!("Success!");

//     }
// }

// fn main(){
//     let f: bool = true;
//     let t: bool = true && !false;
//     assert_eq!(t,f);

//     println!("Successs!");
// }


// make it work don't modify 'implicitly_ret_unit'
// #[allow(unused_variables)]

// fn main(){
//     let _v: () = ();

//     let v: (i32,i32) = (2,3);
//     assert_eq!(_v, implicitly_ret_unit());

//     println!("Success!");


// }

// fn implicitly_ret_unit(){
//     println!("I Will return a ()");
// }

// // Don't use this one
// #[allow(dead_code)]
// fn explicitly_ret_unit() -> (){
//     println!("I will return a ()");
// }


// use std::mem::size_of_val;
// fn main(){
//     let unit: () =();
//     assert!(size_of_val(&unit)== 0);
//     println!("Success!");
// }


// Satements and Expressions

// fn main(){
//     let x: u32 = 5u32;

//     let y: u32 = {
//         let x_squared: u32 = x * x;
//         let x_cube: u32 = x_squared * x;
        
//         x_cube + x_squared + x

//     };

//     let z: u32  = {
//         2 * x
//     };

//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }


// ONE WAY

// fn main(){
//     let v: i32 = {
//         let mut x: i32 = 1;
//         x += 2;
//         x
//     };

//     assert_eq!(v,3);

//     println!("Success!");
// }


// THE OTHER WAY --REMMEBER ()-> is a unit and a unit is nothing 

// fn main(){
//     let v: () = {
//         let mut _x: i32 = 1;
//         _x += 2;// we are assigning the value to x not to v so v -> (unit) which in nothing or empty tuple
        
//     };

//     assert_eq!(v,());

//     println!("Success!");
// }

// fn main(){
//     let v:i32 = {
//         let x: i32 = 3;
//         x
//     };

//     assert!( v == 3);

//     println!("Success!");
// 
//#[allow(unused_variables)]
//fn main(){
  //let s = sum(1,2);
    //assert_eq!(s,3);

  //  println!("Success!");
//}

//fn sum(x: i32, y: i32)-> i32{
//    x + y
//}

//fn main(){

   // let s1 = String::from("Hello");
  //  let s2 = s1.clone();
//
//    println!("s1 is equal to:  {}, s2 is equal to: {}",s1, s2); 


//
//
//}
//fn main(){

// use as many approaches as you can to make it work
//

    //let x:String = String::from("hello, world");
    //let y:String = x.clone();

  //  println!("{},{}",x,y);

// }
//fn main(){

    //let s1: String = String::from("Hello World!");
    //let s2: String = take_ownership(s1);

  //  println!("{}", s2);


//}
// only modify the code below

//fn take_ownership(s:String)-> String {
    //println!("{}",s);
  //  s
//}

//fn main(){
    //let s: String = give_ownership();
  //  println!("{}", s);
//}

//fn give_ownership() -> String {

   // let s: String  = String::from("Hello, World!");

    // Convert String to vec
    
    //let _s = s.as_bytes();
  //  s

//}

//fn main (){
    //let s: String = String::from("Hello, World!");

    //print_str(s.clone());

  //  println!("{}", s);
//}


//fn print_str(s: String) {

//    println!("{}",s);
//
//}

//fn main(){
    //let x:(i32, i32, (), &str) = (1, 2, () , "Hello");
    //let y:(i32, i32, (), &str) = x;
    
  //  println!("{:?}, {:?}", x , y);
//}


//fn main(){
   // let s: String = String::from("Hello");
// Modify this line only
  //  let mut  s1: String  = s;

    //s1.push_str("World!!");

    //println!("Success!");
//}

//fn main(){
  //  let x: Box<i32> = Box::new(5);

//    let mut y:  Box<i32> = Box::new(1);

  //  *y = 4;

    //assert_eq!(*x, 5);

   // println!("Success");

//}
//fn main(){
//    #[derive(Debug)] 
//    struct Person{ 
//        name: String, 
//        age: Box<u8>),
//    }; 
//    let person: Person = Person {
//        name: String::from("Alice"), 
//        age: Box::new(20),
//    }; 
//    //name is moved out of person but age is referenced
//    let Person{name, ref age} = person;
//
//    println!("The persons age is {}", age);
//    println!("The persons name is {name}");
//
//    println!("The person's age from person struct is {}", person.age);
//
//
//}

//fn main(){
//    let t: (String, String) = (String::from("Hello"), String::from("World!!"));
//
//    let _s: String = t.0;
//
//    //Modify this line only don't use  '_s'
//    
//    println!("{:?}", t.1);
//}



//fn main(){
//    let t: (String, String) = (String::from("Hello"), String::from("World!!"));
//
//
//    let (s1,s2) = t.clone(); 
//
//
//
//    println!("{:?},{:?},{:?}", s1 , s2, t);
//}


//fn main(){
//
//    let mut s = String::from("HELLO");
//
//    
//
//    change(&mut s);
//    
//    println!("{}",s);
//    
//}
//
//fn change(some_string: &mut String){
//    some_string.push_str(",WORLD!!")
//}
//#[allow(unused_variables)]
//fn main(){
//    let mut s: String = String::from("HELLO!!");
//
//    {
//        let s1 = &mut s;
//    }
//
//    let s2 = &mut s;
//
//    println!("Success!");
////
//fn main(){
//    
//    let x: i32 = 5;
//
//    let p: &i32 = &x;
//
//    println!("The memory address of x is {:p}",p);
//}
//
//
//fn main(){
//
//    let x: i32 = 5;
//    let y: &i32 = &x;
//
//    assert_eq!(5, *y);
//
//    println!("Success");
//}
//
//fn main(){
//
//    let mut s: String = String::from("Hello, ");
//
//    borrow_object(&s);
//
//    println!("Success!");
//
//}
//
//fn borrow_object(s: &String){
//}
//
//
//fn main(){
//
//    let mut s: String = String::from("Hello");
//
//    push_str(&mut s);
//
//    println!("Success!");
//
//}
//
//fn push_str(s: &mut String){
//
//    s.push_str("World!")
// }

//fn main(){
//
//    let mut s: String = String::from("Hello");
//
//    let p: &mut  String = &mut s;
//
//
//    p.push_str("World!");
//
//    println!("Success!");
//}

//fn main(){
//
//    let x: i32 = 5;
//
//    let p: &i32  = &x; 
//
//
//    println!("the memory address of x is {:p}",p);
//}

//fn main(){
//    
//    let x: i32 = 5; 
//
//    let y: &i32 = &x;
//
//    assert_eq!(5,*y);
//
//    println!("Success!");
//}
//
//#[allow(unused_variables)]
//
//
//fn main(){
//    
//    let mut s: String = String::from("Hello");
//
//    borrow_object(&s);
//
//    println!("Success!");
//
//}
//
//fn borrow_object(s: &String){
//    
//}

//fn main(){
//    
//    let mut s: String = String::from("Hello");
//
//    push_str(&mut s);
//
//    println!("Success!");
//}
//
//fn push_str(s: &mut String) {
//    s.push_str("World");
//}

//fn main(){
//    
//    let mut s: String = String::from("Hello");
//
//    let p: &mut String = &mut s;
//
//    p.push_str("World");
//
//    println!("Success!");
//}


//fn main(){
//    
//    let c: char = '&';
//
//    let r1: &char = &c;
//
//    let ref r2: char = c;
//
//
//
//    assert_eq!(*r1, *r2);
//    
//
//    assert_eq!(get_addr(r1), get_addr(r2));
//
//    println!("Success!");
//}
//
//fn get_addr(r: &char) -> String{
//    format!("{:p}", r)
//}
//
//fn main(){
//    
//    let mut s: String = String::from("Hello");
//    
//    let r1 = &s;
//    let r2 = &s;
//
//    println!("{},{}",r1,r2);
//
//    println!("Success!");
//}

//fn main(){
//    
//    let mut s: String = String::from("Hello");
//
//    borrow_object(&mut s);
//
//    println!("Success!");
//}
//
//fn borrow_object(s: &mut String){
//
//}

//fn main(){
//    
//    let mut s: String = String::from("Hello");
//
//    borrow_object(&s);
//
//    s.push_str("World");
//
//    println!("Success!");
//
//    println!("{}",s);
//}
//#[allow(unused_variables)]
//fn borrow_object(s: &String){
//    
//   
//}

//fn main(){
//    
//    let mut s: String = String::from("Hello");
//
//    let r1: &mut String = &mut s;
//    r1.push_str("World");
//
//    let r2: &mut String = &mut s;
//    r2.push_str("!");
//
//// the pointer r1 would be out of Scope if r2 was used in the print statement but If we used r1
//// again in the print statement we would have an error at compile time.
//
//    println!("{}",r2);
//}

//fn main(){
//    
//    let mut s: String = String::from("Hello");
//
//    //{
//        let r1: &mut String  = &mut s;
//    //}
//
//    let r2: &mut String = &mut s;
//
//    println!("{},{}",r1,r2);
//}


//fn main(){
//    
//    let s: &str = "Hello World!";
//
//    println!("Success!");
//
//}

//
//fn main(){
//    
//    let s: &str = "Hello World";
//
//    greetings(s);
//}
//
//fn greetings(s: &str){
//    
//    println!("{}",s);
//}

//fn main(){
//    
//    let mut s: String = String::from("");
//
//    s.push_str("Hello World");
//
//    s.push('!');
//
//    assert_eq!(s, "Hello World!");
//    
//
//    println!("{}",s);
//
//
//    println!("Success!");
//}

//fn main(){
//    
//    let mut s: String = String::from("Hello");
//
//    s.push(',');
//
//    s.push_str(" World");
//
//    s += "!";
//
//    println!("{}", s);
//}

//fn main(){
//
//    let mut s: String = String::from("I like dogs");
//
//
//    let s1 = s.replace("dogs","cats");
//
//    assert_eq!(s1, "I like cats");
//
//    println!("Success!");
//
//}

//fn main(){
//    
//    let s1: String = String::from("Hello ");
//
//    let s2: String  = String::from("World!");
//
//    let s3: String = s1 + &s2; //String -> &str and &String == &str
//
//    assert_eq!(s3,"Hello World!");
//
//    println!("{}",s3);
//
//}

//fn main(){
//    
//    let s: &str = "hello world!";
//    greetings(String::from(s));  // to convert form string slice -> &str to String I could use the
//                                 // .to_string(), mehtod or using the string ->
//                                 // String::from(variable)
//
//}
//
//fn greetings(s: String){
//
//    println!("{}",s);
//}

//fn main(){
//    
//    let arr: [i32; 5] = [1,2,3,4,5];
//
//    assert_eq!(arr.len() , 5);
//
//    println!("Success!");
//}

//#[allow(unused_variables)]
//fn main(){
//    
//    let arr0: [i32; 3] = [1,2,3];
//
//    let arr:[char; 3]= ['a','b','c'];
//
//
//    assert!(std::mem::size_of_val(&arr)== 12);
//    
//    println!("Success!");
//}


//fn main(){
//
//    let list: [i32;100] = [1;100];
//
//    assert!(list[6] == 1);
//    assert!(list.len() == 100);
//
//    println!("Success!");
//}


//fn main(){
//
//    let _arr: [i32; 3] = [1,2,3];
//
//    println!("Success!");
//
//}

//fn main(){
//
//    let a = [1,2,3,4,5];
//
//    let slice = &a[0..3];
//
//    assert_eq!(slice, &[1,2,3]);
//
//    println!("Success!");
//
//}
//#[allow(unused_variables)]
//fn main(){
//
//    let arr: [i32; 3] = [1 ,2 ,3];
//
//    let s1: &[i32] = &arr[0..2];
//
//    let s2: &str = "Hello World";
//
//    println!("Success!");
//
//}

//fn main(){
//
//    let _t0: (u8, i16) = (0, -1);
//
//    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//
//    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "Hello", String::from("World") );
//
//    println!("Success!");
//
//}
//

//fn main(){
//
//    let t: (&str, &str, &str) = ("I", "am", "Sunface");
//
//
//    assert_eq!(t.2, "Sunface");
//
//    println!("Success!");
//
//}

//fn main(){
//
//    let too_long_tuple: &str = "A tuple can only hold 12 elements";
//
//    println!("{}", too_long_tuple);
//
//}


//fn main(){
//
//    let tuple: (i32, f64, &str) = (1, 6.3, "Hello");
//
//    let (x, z, y) = tuple;
//
//    assert_eq!(x, 1);
//    assert_eq!(y, "Hello");
//    assert_eq!(z, 6.3);
//
//    println!("Success!");
//}

//fn main(){
//
//    let (x, y, z);
//    
//    (y, z, x) = (1, 2, 3);
//
//    assert_eq!(x, 3);
//    assert_eq!(y, 1);
//    assert_eq!(z, 2);
//
//    println!("Success!");
//
//}

//fn main(){
//
//    let (x, y ) = sum_multiply((3,4));
//
//    assert_eq!(x,7);
//    assert_eq!(y,12);
//
//    println!("Success!");
//
//}
//
//fn sum_multiply(nums: (i32, i32))->(i32,i32){
//
//    (nums.0 + nums.1 , nums.0*nums.1)
//
//}

:






