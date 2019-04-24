### Name : ILHAM ARROSYID
### NIM  : 1313617018

# OS-RustExercism


In this task i solved 5 exercise in medium.
1. Triangle
2. Perfect-Numbers
3. Pythagorean-Triplet
4. Hamming
5. Scrabble-Score

This is one problem that i have solved.

## Scrabble Score

### Problem

Scrabble Score is like scrabble game, this is counting the score value of the letter. This case
The value of the letter is...

| Letter | Value |
|:-------|:-------|
|A, E, I, O, U, L, N, R, S, T| 1|
|D, G|2|
|B, C, M, P|3|
|F, H, V, W, Y|4|
|K|5|
|J, X|8|
|Q, Z|10|

for Example : <br>
*"cabbage"* should be scored as worth 14 points:
* 3 points for C
*	1 point for A, twice
*	3 points for B, twice
*	2 points for G
*	1 point for E

And to total: <br>
3 + 2x1 + 2x3 + 2 + 1 <br>
= 3 + 2 + 6 + 3 <br> 
= 5 + 9 <br> 
= 14

### Solution

How i think to solve this problem :
1.	see the test, if the letter any lowercase or uppercase, make it same type. <br>
    i change type a word to `to_lowercase()` for being in the same type
    ```rust
    pub fn score(word: &str) -> u64 {
    //unimplemented!("Score {} in Scrabble.", word);
    let kata = word.to_lowercase();
    ```
2. make a looping with for each, to checking condition in char
    ```rust
     for i in kata.chars(){
    ```
3.	Make a condition letter value
    ```rust
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
    ```
3.	Sum the value of word and return it.
    ```rust
     let mut hasil = 0;
     // in here is looping
     // Condition
     
     hasil
    ```

### Conclusion
In this exercise, i learn to using `chars()` and change word to lowercase/uppercase  `to_lowercase()` `to_uppercase`in String rust to read the character and return the sum of the value word in the integer type `u64`
