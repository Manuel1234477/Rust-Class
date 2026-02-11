fn main() {
    println!("Hello, world!");

    let a = vec![1, 2, 3];
    let cloned = a.clone();
    let double_cloned = cloned.clone();

    println!("{:?}", double_cloned);

    let s1 = String::from("ferris");
    let s2 = &s1;
    println!("s1 = {}", s1);
    
    let mut greeting = String::from("hello");
    print_length( &greeting);
    append_world( &mut greeting);
    println!("{}", greeting )
}

fn print_length(s: &String){
    println!("length = {}", s.len());
}

fn append_world(s: &mut String){
    s.push_str(" world!");
    
}