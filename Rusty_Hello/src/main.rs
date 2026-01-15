use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rusty_hello", about = "CLI arguments et ownership")]
struct Args {
    /// Nom pour saluer 
    #[arg(default_value = "World")]
    name: String,

    /// Pour convertir en majuscule
    #[arg(short, long)]
    upper: bool,

    /// Répeter un nombre de fois 
    #[arg(short, long, default_value_t = 1)]
    repeat: u32,
}

fn main() {
    let args = Args::parse();

    // Gestion de la mise en majuscules (Ownership) 
    let mut display_name = args.name; // c'est une string pour déplacer la donnée vers la variable display_name. On la rend mut pour possible modification
    
    if args.upper {
        display_name = display_name.to_uppercase();
    }

    // Boucle pour la répétition 
    for _ in 0..args.repeat {
        println!("Hello, {}!", display_name);
    }
}
