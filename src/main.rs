// This main function is only used for determining some median values for a given difficulty
#[cfg(feature = "server")]
fn main() -> Result<(), spow::pow::PowError> {
    use spow::pow::Pow;
    use std::cmp::{max, min};
    use std::env;
    use std::time::Instant;

    let mut args = env::args();
    args.next();

    let difficulty = args
        .next()
        .expect("provide the difficulty as arg")
        .parse::<u8>()
        .expect("the difficulty cannot be parsed to u8 value");

    let it = args
        .next()
        .expect("provide the number of iterations as arg")
        .parse::<u64>()
        .expect("the number of iterations cannot be parsed to u64 value");

    println!(
        "Finding rough values for performance comparison with {} iterations",
        it
    );

    Pow::init_random()?;

    let mut elapsed_total = 0;
    let mut elapsed_min = u64::MAX;
    let mut elapsed_max = 0;

    for i in 0..it {
        let p = Pow::with_difficulty(difficulty, 60)?;
        let challenge = p.to_string();

        let start = Instant::now();
        let res = Pow::work(&challenge)?;

        let millis = start.elapsed().as_millis() as u64;
        elapsed_total += millis;
        elapsed_min = min(elapsed_min, millis);
        elapsed_max = max(elapsed_max, millis);

        println!("{}: {}", i, res);
    }

    let median = elapsed_total / it;
    println!(
        r#"
After {} iterations with difficulty {}:

median: {} ms
min:    {} ms
max:    {} ms
"#,
        it, difficulty, median, elapsed_min, elapsed_max
    );

    Ok(())
}

#[cfg(not(feature = "server"))]
fn main() {
    panic!("Enabled the `server` feature to compile binary");
}
