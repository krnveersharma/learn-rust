mod evenodd;
fn main() {
    let ans: bool=evenodd::even_odd(2);
    if ans==false {
        print!("Number is odd\n");
    }
    else{
        print!("number is even\n")
    }
}
