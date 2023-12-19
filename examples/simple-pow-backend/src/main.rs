use spow::pow::{Pow, PowError};

fn main() -> Result<(), PowError> {
    // First, we need to initialize our signing secret.
    // If you need to be able to validate PoW's from multiple backend's,
    // use `init()` and provide the same secret for each instance.
    Pow::init_random()?;

    // Create a new PoW which will be valid for 60 seconds
    let pow = Pow::new(60)?;

    // Create a puzzle challenge from this pow.
    // You can either call `build_challenge()` or `.to_string()`.
    let challenge = pow.to_string();
    println!("The challenge we would need to send to a client:\n{}", challenge);

    // At this point, we would send this challenge to a client, which then
    // needs to solve the puzzle and send the result back. For the sake of
    // this example, we will simply solve it ourselves at this point.
    // How it works in the UI can be seen in the other example in this repo.
    let work = Pow::work(&challenge)?;
    println!("\nThe result the client would send back:\n{}", work);

    // The client would send back the result with another request, which
    // we then need to validate.
    Pow::validate(&work)?;
    println!("\nThe PoW has been validated successfully");

    // ... that's it. If the `validate()` does not throw an error, the result
    // was correct and valid.

    Ok(())
}
