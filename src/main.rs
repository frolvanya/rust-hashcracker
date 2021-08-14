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

async fn hash_brute_forcing() {
    let mut tasks = Vec::new();
    let mut hashed_amount = 0;
    let start_time = std::time::Instant::now();

    for password in password_generator() {
        hashed_amount += 1;

        tasks.push(async move {
            let mut hasher = sha2::Sha256::new();
            hasher.update(password.as_bytes());

            if "9e69e7e29351ad837503c44a5971edebc9b7e6d8601c89c284b1b59bf37afa80".to_string()
                == format!("{:x}", hasher.finalize())
            {
                println!("{}", "-".repeat(50));

                println!(
                    "[{}] Correct Password: '{}'",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    password,
                );

                println!(
                    "[{}] Request Amount: {}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    hashed_amount.clone(),
                );

                println!(
                    "[{}] Requests Per Second: {:.3}",
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
    println!(
        "[{}] Starting Hash Brute Force",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    );
    hash_brute_forcing().await;
}
