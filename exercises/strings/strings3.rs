// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.




fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut str  = String::from(input);

    let len = str.len();
    let mut start = 0 ;
    let mut end = 0;
    let mut count = 0;
    for i in input.chars(){
        if i != ' '{
            break;
        }
        start += 1;
    }
    for i in input.chars(){
        if i != ' '{
            end = count
        }
        count += 1;
    }
    return input[start..end+1].to_string();
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let str  = String::from(input);
    return str + " world!";
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let lens = input.chars().count();
    let str = String::new();
    let mut cnt = 0;
    for i in 0..lens - 3{
        if &input[i..i+4] == "cars"{
            cnt = i ;
            break
        }
    }
    return  str + &input[0 ..cnt] + "balloons" + &input[cnt+4 ..];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
