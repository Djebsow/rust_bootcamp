use clap::Parser;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(
    name = "wordfreq",
    about = "Compte la fréquence des mots dans un texte"
)]
struct Args {
    /// Texte à analyser (si absent, lit depuis l'entrée standard)
    text: Option<String>,

    /// Affiche les N mots les plus fréquents [default: 10]
    #[arg(long, default_value_t = 10)]
    top: usize,

    /// Ignore les mots plus courts que N [default: 1]
    #[arg(long, default_value_t = 1)]
    min_length: usize,

    /// Ne fait pas de distinction entre majuscules et minuscules
    #[arg(long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    // 1. Récupération du texte (Argument ou STDIN)
    let mut content = String::new();
    if let Some(t) = args.text {
        content = t;
    } else {
        // Lit tout le contenu envoyé via un pipe (ex: echo "test" | cargo run)
        io::stdin().read_to_string(&mut content).unwrap_or(0);
    }

    // 2. Traitement du texte et comptage
    let mut counts = HashMap::new();

    // On sépare le texte en mots en ignorant tout ce qui n'est pas alphanumérique
    for word in content
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
    // On enlève les chaînes vides
    {
        let mut processed_word = word.to_string();

        // Gestion de la casse (ignore_case)
        if args.ignore_case {
            processed_word = processed_word.to_lowercase();
        }

        // Filtrage par longueur (min_length)
        if processed_word.len() >= args.min_length {
            *counts.entry(processed_word).or_insert(0) += 1;
        }
    }

    // 3. Transformation en Vecteur pour le tri
    let mut sorted_counts: Vec<_> = counts.into_iter().collect();

    // LOGIQUE DE TRI :
    // On trie d'abord par fréquence (b.1.cmp(&a.1) -> ordre décroissant)
    // Puis par ordre alphabétique en cas d'égalité (a.0.cmp(&b.0))
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // 4. Affichage des résultats (limité par l'argument --top)
    for (word, count) in sorted_counts.into_iter().take(args.top) {
        println!("{}: {}", word, count);
    }
}
