use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_+";

pub fn process_genpass(length: u8, upper: bool, lower: bool, number: bool, symbol: bool) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut char_pool = Vec::new();

    if upper {
        char_pool.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("Failed to choose a character from UPPER"));
    }

    if lower{
        char_pool.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("Failed to choose a character from LOWER"));
    }

    if number{
        char_pool.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("Failed to choose a character from NUMBER"));
    }

    if symbol{
        char_pool.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("Failed to choose a character from SYMBOL"));
    }

    for _ in 0.. (length - password.len() as u8){
        let c = char_pool.choose(&mut rng)
            .expect("Failed to choose a character from char_pool");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;

    print!("{}",password);

    Ok(())

}