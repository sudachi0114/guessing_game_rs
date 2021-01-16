use std::io;

fn main() {
    println!("Guess the number!");

	println!("Please input your guess.");

	let mut guess = String::new();

	io::stdin().read_line(&mut guess)
		.expect("Failed to read line");  // 入力の読み込みに失敗

	println!("You guessed: {}", guess);
}


