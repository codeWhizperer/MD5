use md5::md5::{md5};



fn main() {
    let output = md5("Hello world :)");
    println!("output: {:?}", output)
}
#[cfg(test)]
mod test{
    use super::*;
#[test]
    fn test_digest(){
        let result = "0df1081a4a83352fa3f1d97c3444bf0d";
        let input = "Hello world :)";
        let digest = md5(input);
        assert_eq!(result,digest);
        }
}