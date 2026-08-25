// This main function is only used for determining time taken for a given difficulty
#[cfg(feature = "server")]
fn main() -> Result<(), spow::pow::PowError> {
    use spow::pow::Pow;
    use std::env;
    use std::time::Instant;

    let mut args = env::args();
    args.next();

    let difficulty = args
        .next()
        .expect("provide the difficulty as arg")
        .parse::<u8>()
        .expect("the difficulty cannot be parsed to u8 value");
    if !(10..100).contains(&difficulty) {
        panic!("The difficulty (first arg) must be 10 <= difficulty <= 99");
    }

    let it = args
        .next()
        .expect("provide the number of iterations as arg")
        .parse::<u64>()
        .expect("the number of iterations cannot be parsed to u64 value");
    if it == 0 {
        panic!("The iterations (second arg) must not be 0");
    }
    if it < 100 {
        eprintln!("\nMake sure to have a high enough iterations (second arg) for good results.\n");
    }

    println!("Running calculations with difficulty {difficulty} and {it} iterations\n");

    Pow::init_random()?;

    let mut durations = Vec::with_capacity(it as usize);

    for i in 0..it {
        let p = Pow::with_difficulty(difficulty, 60)?;
        let challenge = p.to_string();

        let start = Instant::now();
        let res = Pow::work(&challenge)?;

        let millis = start.elapsed().as_millis() as u64;
        durations.push(millis);

        println!("{}: {}", i, res);
    }

    durations.sort_unstable();

    let get_percentile = |p: f64| -> u64 {
        if durations.is_empty() {
            return 0;
        }
        let idx = ((durations.len() as f64 - 1.0) * p).round() as usize;
        durations[idx]
    };

    let p50 = get_percentile(0.50);
    let p90 = get_percentile(0.90);
    let p95 = get_percentile(0.95);
    let p99 = get_percentile(0.99);
    let p99_9 = get_percentile(0.999);

    println!(
        r#"
    After {it} iterations with difficulty {difficulty}:

    p50:    {p50} ms
    p90:    {p90} ms
    p95:    {p95} ms
    p99:    {p99} ms
    p99.9:  {p99_9} ms
    "#
    );

    Ok(())
}

#[cfg(not(feature = "server"))]
fn main() {
    panic!("Enabled the `server` feature to compile binary");
}
