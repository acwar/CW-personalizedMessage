fn main() {
    println!("Hello, world!");
}

fn greet(name : &str, owner: &str)-> String {
    let x:String  = "Hello ".to_string();
    if name == owner {
        return x + "boss";
    }
    return x+"guest";

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_greet(){
        assert_eq!(greet("Daniel", "Daniel"), "Hello boss");
        assert_eq!(greet("Greg", "Daniel"), "Hello guest");


    }
}
