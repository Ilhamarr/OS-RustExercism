/// Compute the Scrabble score for a word.
// Saya kerjakan problem ini sendiri
pub fn score(word: &str) -> u64 {
    //unimplemented!("Score {} in Scrabble.", word);
    let kata = word.to_lowercase();
    let mut hasil = 0;
    for i in kata.chars(){
        if i == 'a' || i == 'e' || i == 'i' || i == 'o' ||i == 'u' || i == 'l' ||i == 'n' || i == 'r' ||i == 's' || i == 't' {
            hasil += 1
        }
        if i == 'd' || i == 'g' {
            hasil += 2
        }
        if i == 'b'|| i == 'c' || i == 'm' || i == 'p' {
            hasil += 3
        }
        if i == 'f' || i == 'h' || i == 'v' || i == 'w' || i == 'y' {
            hasil += 4
        }
        if i == 'k' {
            hasil += 5
        }
        if i == 'j' || i == 'x'{
            hasil += 8
        }
        if i == 'q' || i == 'z'{
            hasil += 10
        }
        else {
            hasil += 0
        }
    }
    hasil
    
}
