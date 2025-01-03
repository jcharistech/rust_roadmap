
fn main() {
    println!("Cryptography");

    // Substitution Cipher: replacing each member of the plaintext with another member which can be of the same set. 
    // Rot13
    let cipher_text = rot13_encrypt("a");
    println!("{}",cipher_text);
    let reverse_cipher_text = reverse_encrypt("hello");
    println!("{}",reverse_cipher_text);
    let gematria_1 = gematric_shift_encrypt("abc");
    println!("{}",gematria_1);
    
}

// Symmetric substitution cipher:  can be used to both encrypt and decrypt
fn rot13_encrypt(text:&str) -> String{
    let encrypted_text = text.to_uppercase();
    encrypted_text
    .chars()
    .map(|c| match c {
        'A'..='M' => ((c as u8) + 13) as char,
        'N'..='Z' => ((c as u8) - 13) as char,
        _ => c,

    }).collect()
}

// Caesar Shift (rotX, rot16)
fn caesar_encrypt(text: &str, shift: u8) -> String{
    let mut encrypted_text = String::new();
    for char in text.chars(){
        if char.is_ascii_alphabetic(){
            let base = if char.is_ascii_uppercase() {'A'} else {'a'};
            let shifted = ((char as u8 - base as u8 + shift ) % 26) + base as u8;
            encrypted_text.push(shifted as char);
        } else {
            encrypted_text.push(char)
        }
    }
    encrypted_text

}

fn reverse_encrypt(text: &str)  ->String {
    text.chars().rev().collect()
}


fn gematric_shift_encrypt(text: &str) -> String{
    // Convert Alphabets to Numbers and join all numbers separated by white space
    // since we have double digits and we want to differentiate them
    let mut encrypted_text = String::new();
    for char in text.chars(){
        if char.is_ascii_alphabetic(){
            let base = if char.is_ascii_uppercase() {'A' as u32 } else {'a' as u32};
            let number = (char as u32 -base + 1) as u8;
            encrypted_text.push_str(&number.to_string());
            encrypted_text.push(' ');
        }
    }
    encrypted_text.trim().to_string()
}


fn simple_gematria(text: &str) -> u32 {
    // convert alphabets to numbers and get the sum of all of them
    let mut gematria = 0;
    for char in text.to_lowercase().chars(){
        if char.is_alphabetic(){
            let value = (char as u32 - 'a' as u32 + 1) as u32;
            gematria += value;
        }
    }
    gematria
}


mod tests {
    use super::*;

    #[test]
    fn test_rot13(){
        assert_eq!(rot13_encrypt("hello"),"URYYB")
    }

    
    #[test]
    fn test_rot13_decrypt(){
        assert_eq!(rot13_encrypt("URYYB"),"HELLO")
    }


    #[test]
    fn test_caesar_encrypt(){
        assert_eq!(caesar_encrypt("hello",16),"xubbe");
        assert_eq!(caesar_encrypt("hello",13),"uryyb")
    }
    #[test]
    fn test_caesar_encrypt_as_13(){
        assert_eq!(caesar_encrypt("hello",13),"uryyb")
    }

    #[test]
    fn test_gematria_shift_encrypt(){
        assert_eq!(gematric_shift_encrypt("rust",),"18 21 19 20")
    }

    #[test]
    fn test_simple_gematria(){
        assert_eq!(simple_gematria("rust",),78);
        assert_eq!(simple_gematria("rust",),78)
    }
}

