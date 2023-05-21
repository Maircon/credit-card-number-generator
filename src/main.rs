use rand::Rng;

enum Provider {
	Visa(String, u8),
	MasterCard(String, u8),
	AmericanExpress(String, u8)
}

impl Provider {
	fn get_name(&self) -> String {
		match	&self {
			Provider::Visa(name, _) => name.to_string(),
			Provider::MasterCard(name, _) => name.to_string(),
			Provider::AmericanExpress(name, _) => name.to_string()
		}
	}

	fn get_first_number(&self) -> u8 {
		match	&self {
			Provider::Visa(_, code) => *code,
			Provider::MasterCard(_, code) => *code,
			Provider::AmericanExpress(_, code) => *code
		}
	}
}

fn generate_provider() -> Provider {
	match rand::thread_rng().gen_range(1..=3) {
		1 => Provider::Visa(String::from("VISA"), 4),
		2 => Provider::MasterCard(String::from("MasterCard"), 5),
		3 => Provider::AmericanExpress(String::from("American Express"), 3),
		_ => unreachable!()
	}
}

fn generate_four_random_numbers(fnumber: Option<u8>) -> String {
	let first_number: u8 = if fnumber.is_some() { fnumber.unwrap() } else { rand::thread_rng().gen_range(0..=9) };
	let second_number: u8 = rand::thread_rng().gen_range(0..=9);
	let third_number: u8 = rand::thread_rng().gen_range(0..=9);
	let forth_number: u8 = rand::thread_rng().gen_range(0..=9);
	return first_number.to_string() + &second_number.to_string() + &third_number.to_string() + &forth_number.to_string();
}

fn generate_credit_card(provider: &Provider) -> String {
	let first_number = provider.get_first_number();
	let credit_card = format!("{} {} {} {}",
		generate_four_random_numbers(Some(first_number)),
		generate_four_random_numbers(None),
		generate_four_random_numbers(None),
		generate_four_random_numbers(None)
	);
	return credit_card;
}

fn main() {
	let provider = generate_provider();
	let card_number = generate_credit_card(&provider);
	println!("provider: {} and code {}", provider.get_name(), provider.get_first_number());
	println!("credit card number {}", card_number);
}
