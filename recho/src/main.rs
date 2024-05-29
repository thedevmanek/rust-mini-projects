use std::env;

fn echo_input(args: Vec<String>) -> String {
    if args.len()==1{
        return String::new();
    }
    let mut output=String::new();
    for i in 1.. args.len(){
        let input:&String=&args[i];
        output.push_str(input);
        if i!=args.len()-1{
            output.push_str(" ")
        }
    }
    return output;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", echo_input(args))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_input_with_none() {
        // Test with None input
        let mut input:Vec<String> = Vec::new();
        input.push("0".parse().unwrap());
        let result = echo_input(input);
        assert_eq!(result, String::new());
    }

    #[test]
    fn test_echo_input_with_some() {
        let mut input:Vec<&str> = "Hello, world!".split(" ").collect();
        let mut input_str:Vec<String>=input.iter().map(|&s| s.to_string()).collect();
        input_str.insert(0,"none".to_string());
        println!("{:?}",input_str);
        let result = echo_input(input_str);
        assert_eq!(result, String::from("Hello, world!"));
    }
}
