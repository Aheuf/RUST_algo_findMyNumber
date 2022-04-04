use rand::Rng;

fn main() {
    let mystery = rand::thread_rng().gen_range(1..=100);
    println!("mystery = {} ;)", mystery);
    let player_name = input("quel est votre prénom ? ");
    let mut essai = 0;
    println!("bienvenu dans chiffre mystère {}", player_name);
    println!("je pense à un nombre entre 1 et 100, serais-tu deviner lequel ?");
    while essai != 6 {
        let val = int(&input("quelle est votre proposition ? "));
        essai += 1;

        match val {
            val if val == mystery => {
                println!("bien joué {} !!!!! vous avez trouvé en {} coups", player_name, essai);
                break;
            },
            val if (val < mystery && val != -1) => println!("c'est plus !"),
            val if val > mystery => println!("c'est moins !"),
            _ => {
                println!("saisie incorrecte !");
                essai-=1
            }
        }
    }
}

fn input(message: &str) -> String {
    use std::io;
    use std::io::Write;
    print!("{}",message);
    io::stdout().flush().unwrap(); //vidage du buffer d'affichage
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap(); // attente de la saisie
    return buffer.trim_end().to_string();
}

fn int(s:&String) -> i32 {
    match s.parse::<i32>(){
        Ok(x) => x,
        Err(_) => -1 //-1 si ParseError
    }
}