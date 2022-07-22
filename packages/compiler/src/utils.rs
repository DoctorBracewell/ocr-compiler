const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn denary_to_alphabet(n: u32) -> String {
    let mut num = n.clone();

    let values = if num == 0 {
        vec![0]
    } else {
        let mut vec: Vec<usize> = Vec::new();

        while num > 0 {
            vec.push((num % 26) as usize);
            num = num / 26;
        }

        vec
    };

    // Index alphabet string using number array and reverse it to find the correct string
    values
        .into_iter()
        .map(|v| ALPHABET.as_bytes()[v] as char)
        .rev()
        .collect::<String>()
}

#[cfg(test)]
mod denary_to_alphabet_tests {
    use super::*;

    #[test]
    fn single_digit() {
        let result = denary_to_alphabet(0);
        assert_eq!(result, "a");
    }

    #[test]
    fn double_digit() {
        let result = denary_to_alphabet(26);
        assert_eq!(result, "ba");
    }

    #[test]
    fn high_number() {
        let result = denary_to_alphabet(232423);
        assert_eq!(result, "nfvj");
    }
}
