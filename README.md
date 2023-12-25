# spow - Proof of Work for Server and Client + WASM

## Why?

Everyone loves captcha's right? Well, not that much. They are absolutely annoying and often
even humans cannot solve them, because something is just on the edge or the server on the
other side simply thinks your stupid and gives you another one.  
They provide an absolutely awful UX and the only reason they exist is to prevent spammers.

Proof of Work's (PoW) solve kind of the same task, but without any real impact on the UX, but
differently. Instead of making the user solve a puzzle, they make your computer solve it
for you. The user has nothing else to do than wait a small amount of time until the puzzle
has been solved automatically.

The idea behind it is that it is just way too costly for any bots and spammers to do this 
for a single action. If you are just a normal person using an App or a web page, you don't 
care and will not even notice this small amount of work your computer has to do now
and then, but if you are a spammer and are doing this on millions of Apps at the same time,
it will simply not be possible.

There are really sophisticated solutions to this out there, like
[Cloudflare Turnstile](https://www.cloudflare.com/products/turnstile/) which I really like.
This tiny crate however makes it possible to get the same UX while being able to easily
self-host it and just integrate it into your API without any external dependencies.

## How?

In the backend, it could not be much simpler:

1. At application startup, you need to call either `Pow::init()` or `Pow::init_random()` once.
This will initialize the secret which is used to sign the challenges, so you know if a challenge
actually came from your backend, or if someone else has just generated one himself.
2. If you want to create a new challenge for a client at some point, you then only need to
create a new PoW with either `Pow::new()` or `Pow::with_difficulty()`. This will create
a random, unique PoW, which can be converted into a challenge for the client with either
`.build_challenge()` or `.to_string()`.
3. Send the challenge string to the client
4. The client needs to do some work to solve that puzzle. To do that, you can use a wasm
or JS-only version. These are in the `frontend/` folder and can be used directly in any 
frontend. If you do not have a really good reason not to, you should always use the wasm
version, because it is up to ~20 times faster than JS only. This means you can use higher
difficulties with no impact on the UX.
5. When you get back a solved puzzle from a client, just `Pow::validate()` it. This will ...
    - validate the version tag
    - validate the difficulty (must be 10 - 99)
    - validate the expiry of the PoW (set during `Pow::new()`)
    - validate that the challenge actually came from the same backend and has not been changed
    - validate the puzzle result itself using the given difficulty

... that's it.

Under the hood, `spow` uses a modified, extended version of the `Hashcat` PoW algorithm,
which is being used for instance for the Bitcoin blockchain as well.

## Difficulty

The difficulty is an int between 10 and 99. The default is `20`.  
With each step, the time to solve the puzzle will increase exponentially, while the time to
validate it in the backend will always stay the same and is independent of the difficulty.

How high you want to set this greatly depends on your use case, where your app will be running
(powerful desktop machines, laptops, or an app on some smartphone, ...) and how much you want
to simply make the user wait.

The way these puzzles work is, you can get very lucky and solve it quickly, or have bad luck
and need way longer than the median value. All of this depends on the random values of the
PoW during creation. To find a good value, you should test a lot of different PoW's preferably
on your target system to find a good median time.  
This project has a bin target which does exactly that, but actually in the backend. You can
run it with for instance `cargo run --release 20 100` and it will solve 100 randomly generated
puzzles with difficulty 20, and at the end print out the median time. Please keep in mind though
that the backend code is much more performant than the one inside the browser. In first tests,
the wasm version in the browser needs roughly 2.5 - 3 times the amount of time, that is needed
in the backend in optimized, pure rust code. The JS version is ~20 times slower than wasm.

The `examples` folder contains a tiny frontend demo using Svelte. It uses a hardcoded
challenge though, and it only shows how to solve a puzzle at this time. It does not provide
a good indicator for a median time needed to solve puzzles with these difficulties.
