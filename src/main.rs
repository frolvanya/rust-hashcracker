use clap::{App, Arg};
use itertools::Itertools;
use sha2::Digest;

fn password_generator() -> impl Iterator<Item = String> {
    let charset: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

    (1..=20)
        .flat_map(move |len| {
            charset
                .chars()
                .combinations_with_replacement(len)
                .map(move |combos| (combos, len))
        })
        .flat_map(|(combos, len)| combos.into_iter().permutations(len))
        .dedup()
        .map(|chars| chars.into_iter().collect())
}

async fn hash_brute_forcing(entered_hash: &str) {
    let mut tasks = Vec::new();
    let mut hashed_amount = 0;
    let start_time = std::time::Instant::now();

    for password in password_generator() {
        hashed_amount += 1;

        tasks.push(async move {
            let mut hasher = sha2::Sha256::new();
            hasher.update(password.as_bytes());

            if entered_hash == format!("{:x}", hasher.finalize()) {
                println!("{}", "-".repeat(50));

                println!(
                    "[{}] Correct Password: '{}'",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    password,
                );

                println!(
                    "[{}] Hashed Amount: {}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    hashed_amount.clone(),
                );

                println!(
                    "[{}] Hashes Per Second: {:.3}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    hashed_amount as f64 / start_time.elapsed().as_secs_f64(),
                );
                std::process::exit(1);
            }
        });

        if tasks.len() == 1000000 {
            println!(
                "[{}] Executing {} tasks",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                &tasks.len()
            );

            futures::future::join_all(tasks).await;
            tasks = Vec::new();
        }
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("Hash Brute Force")
        .version("1.0")
        .author("Frolov Ivan <frolvanya@gmail.com>")
        .about("Cracking Hashes")
        .arg(
            Arg::with_name("sha256")
                .long("sha256")
                .value_name("HASH")
                .help("Sets a sha256 hash")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sha512")
                .long("sha512")
                .value_name("HASH")
                .help("Sets a sha512 hash")
                .takes_value(true),
        )
        .get_matches();

    let sha256_hash: &str = match matches.value_of("sha256") {
        None => "None",
        hash => hash.unwrap(),
    };

    let sha512_hash: &str = match matches.value_of("sha512") {
        None => "None",
        hash => hash.unwrap(),
    };

    if sha256_hash == "None" && sha512_hash == "None" {
        println!(
            "[{}] Hash Was Not Entered",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
        );
        std::process::exit(1);
    }

    println!(
        "[{}] Starting Hash Brute Force",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    );

    if sha256_hash != "None" {
        hash_brute_forcing(sha256_hash).await;
    } else if sha512_hash != "None" {
        hash_brute_forcing(sha512_hash).await;
    }
}
