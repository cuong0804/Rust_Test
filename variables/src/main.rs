fn main() {
    let a :i32 =5 ;
    let b :i32 =6;
    let c:i32;
    c= a+b; 
    println!("{:?}", c); 
    //Rust provides type safety via static typing.
    //EXample
    let an_integer =1u32;
    let a_boolean = true;
    let unit=();
    let copied_integer = an_integer;
    println!("An integer =: {:?}",copied_integer);
    println!("An boolean  =: {:?}",a_boolean);
    println!("Unit value:{:?} ", unit);
    // const (hang) khong the thay doi trong qua trinh chay
    const PI :f64 =3.14159;
    println!("{}", PI);
    // static variable 
    
    // de thay doi static variable co the su dung unsafe

    // co the su dung cau truc du lieu nhu Mutex or Atomic de thay doi variable static


    // chuyen thanh in hoa 
    let s : &str ="hello";
    let s : String =s.to_uppercase();
    println!("{}",s);


}
