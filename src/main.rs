use std::io;
use webbrowser;

fn main() {
    println!("Download tools 0.1 - 0x477564 @GudCheat @HerosPvP (14-8-2020)");

    loop {
        println!("Scrivi il numero corrispondere del tool da scaricare:\n1 = Paladin\n2 = Blsquad\n3 = Avenge");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse();

        match input {
            Ok(result) => {
                match result {
                    1 => open_link("http://dl.pld.ac"),
                    2 => open_link("https://mega.nz/file/lVcUXAzK#3YBs1hS2vRGUsxTYIyU5PfQL7zC1flKSPm5zxYXO39E"),
                    3 => open_link("http://dl.avenge.ac"),
                    _ => continue
                }
            }
            Err(err) => {
                println!("Si e' verificato un errore di parsing (non hai inserito un numero valido): {}", err);
                continue
            }
        }
    }

}

fn open_link(link: &str) {
    println!("Mi sto collegando a: {}", link);
    if webbrowser::open(link).is_ok() {
        println!("Collegamento a '{}' riuscito, controlla il browser.", link);
    } else {
        println!("Non sono riuscito a connettermi a: {}", link);
    }
}
