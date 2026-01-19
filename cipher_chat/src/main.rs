use clap::{Parser, Subcommand};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

// Valeurs fournies dans l'énoncé pour Diffie-Hellman
const P: u64 = 0xD87FA3E291B4C7F3;
const G: u64 = 2;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    role: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lancer le serveur sur un port
    Server { port: u16 },
    /// Se connecter à un serveur (ip:port)
    Client { addr: String },
}

// Fonction pour l'exponentiation modulaire (square and multiply)
// Indispensable pour calculer les clés DH sans overflow
fn power_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut res = 1u128;
    let mut b = base as u128;
    let m = m as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * b) % m;
        }
        b = (b * b) % m;
        exp /= 2;
    }
    res as u64
}

fn run_chat(mut stream: TcpStream, is_server: bool) -> io::Result<()> {
    // --- Phase 1 : Échange Diffie-Hellman ---

    // On génère notre part du secret
    let my_priv = rand::thread_rng().gen_range(1..P);
    let my_pub = power_mod(G, my_priv, P);

    let mut buffer_pub = [0u8; 8];
    if is_server {
        // Le serveur envoie sa clé publique puis attend celle du client
        stream.write_all(&my_pub.to_be_bytes())?;
        stream.read_exact(&mut buffer_pub)?;
    } else {
        // Le client attend d'abord la clé du serveur
        stream.read_exact(&mut buffer_pub)?;
        stream.write_all(&my_pub.to_be_bytes())?;
    }

    let other_pub = u64::from_be_bytes(buffer_pub);
    let final_secret = power_mod(other_pub, my_priv, P);
    println!("Clé de session établie : {:x}", final_secret);

    // --- Phase 2 : Communication chiffrée ---

    // On utilise le secret comme graine pour le flux XOR
    let mut cipher_rng = ChaCha20Rng::seed_from_u64(final_secret);
    let mut msg_buf = [0u8; 1024];

    loop {
        if is_server {
            // Lecture du message chiffré venant du client
            let size = stream.read(&mut msg_buf)?;
            if size == 0 {
                break;
            }

            // On applique le XOR pour déchiffrer
            let decoded: Vec<u8> = msg_buf[..size]
                .iter()
                .map(|b| b ^ cipher_rng.r#gen::<u8>())
                .collect();
            println!("Client dit : {}", String::from_utf8_lossy(&decoded));
        } else {
            // Saisie utilisateur et envoi chiffré
            let mut text = String::new();
            io::stdin().read_line(&mut text)?;

            let crypted: Vec<u8> = text
                .trim()
                .as_bytes()
                .iter()
                .map(|b| b ^ cipher_rng.r#gen::<u8>())
                .collect();
            stream.write_all(&crypted)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.role {
        Commands::Server { port } => {
            let server = TcpListener::bind(format!("0.0.0.0:{}", port))?;
            println!("Serveur en attente sur le port {}...", port);
            if let Some(conn) = server.incoming().next() {
                run_chat(conn?, true)?;
            }
        }
        Commands::Client { addr } => {
            let conn = TcpStream::connect(addr)?;
            println!("Connecté au chat !");
            run_chat(conn, false)?;
        }
    }
    Ok(())
}
