use md5::md5::{  md5};
fn main() {
    // let ascii_text = "Yes, the above function supports both single-byte (ASCII) and multi-byte Unicode characters because it uses Rust’s chars() iterator, which correctly identifies each Unicode character as a single logical unit (or ) regardless of the number of bytes it requires in UTF-8.";        
    // let unicode_text = "Héllo 😊";
    // let _output_ascii = convert_str_to_vec(&ascii_text);
    // let _output_unicode = convert_str_to_vec(&unicode_text);
    let num: u64 = 0x123456789ABCDEF0;
    // let split_manual = split_u64_to_u8_array(num);
    // let split_opt = split_u64_to_u8_array2(num);
    let output = md5("Hello");
    // println!("manual {:?}",split_manual);
    // println!("optimized {:?}",split_opt);
    println!("output: {:?}", output)



    // println!("{:?}",output_ascii);
    // println!("{:?}",output_unicode);

}
