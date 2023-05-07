#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn debugPrint(){
    println!("{:?} months in year.",12);
    println!("{1:?} {0:?} is the {actor:?} name.","Slater","Christan",actor="actor\'s");

     // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}