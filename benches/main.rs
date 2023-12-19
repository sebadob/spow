use criterion::{criterion_group, criterion_main, Criterion};
use spow::pow::Pow;

static TEST_CHALLENGES: [&str; 20] = [
    "1:14:1702763000:kBAxun85H3u8VU7V:Bo47e37phaYgu2fIpI7Pss4otpLglpLBI47KiirhWpI:",
    "1:14:1702763000:VjEd2qqS7AEYOIqr:u7661yas+CB/9mjkNto4EUUciyFkRk7mip7e8K3AVAg:",
    "1:14:1702763000:zb4JO2HpGlsm6P0z:fQmVWe8kkJzjTavLOSZTujp2H6ttUrPhnWD4OH198yM:",
    "1:14:1702763000:eSmq9NzfKOASKoZ3:whRBW5mvnJeEWymrDx7YT4yAtjH8L6FWJeTN+Hcoans:",
    "1:14:1702763000:CNykv8B7yq6ZEWh3:cmF3bMfKhVfSuZGpDt0yhib3buk+JnoAxnqQXCn5vTE:",
    "1:14:1702763000:u6eDmr3XmyFfp5L8:Dtd3Nak1ltyR73MtfwM3fUwm01YkOqorvinlqsWkLNk:",
    "1:14:1702763000:m9MrcXxXPKCXdWLw:u+fSax6PotVrZtO5Q/FqzMtOeOyoPnwicKKLFMoJ0Wc:",
    "1:14:1702763000:io3WLXuEqBqjmWAT:pfeTGlbsBPuZ2qxmH2ye1wkCr/hOi2AO2ps3l9c/YL4:",
    "1:14:1702763000:PvqvkKV9VkJJtwIw:6VzuMau75SOQi0ptyNDcIJjdjIRZdsPvV15IL0wPD0s:",
    "1:14:1702763000:cqO66wCUzIkrktF/:NODrEwkMf4HrJrUu1Yae66oZX8OpkExZjgay5z+qLVo:",
    "1:14:1702763000:yWTsVzq45dQisdMo:TIBIHYMpPVzNIkQ0YGLOuRttDgPQO2kACYaxYV2Nc+4:",
    "1:14:1702763000:cGW2T70x9dfxbt3G:24XPlWhbyco9SXxbEJ4GZz+jMDsrbgTfbG2U1W1YXzc:",
    "1:14:1702763000:X1gU59ZD9YPzkmCh:op2CK/a6EMSXhGqn2XyNkMVZYxNqSzocmNp8NhokrNs:",
    "1:14:1702763000:FkSGlloxXWKRyx1z:OVDNqE0TTqaOx/N6pwNGxob3gfjHoxdDTFXFv+sgam4:",
    "1:14:1702763000:uLA03IzICJdNOkcQ:qXHDAik4uOwwsAtiHj1jIJrdPBJ8ZDT+oKmTQAdevZg:",
    "1:14:1702763000:SQTOBRkEfix90yiA:i29ZmKIgamYqpHKM+uTfo3Ya0ZePV9ZJ9ks0yLRjRQ8:",
    "1:14:1702763000:arwdGyGVcjUvWw3D:u5tXij3lAVZpN9ocfQhO86L5CVINURyANAc6p2RY4YM:",
    "1:14:1702763000:FSjaNuiGpwwgT4Uh:3q9dH9NK0DpKVCivNQulO/300z3nN9Nc6/pQZE1rLog:",
    "1:14:1702763000:xElvmRnE+v4pzFjc:V3Un7zJrncAdJCL/cCiTJcOVwNEsLokcCPuw6JkUVjw:",
    "1:14:1702763000:9j5Mqo4z8oSJRV17:nQHpX9isDusIM/7OUrNi/9D+LVnr4j6llOyEQiujkqw:",
];

fn criterion_benchmark(c: &mut Criterion) {
    Pow::init("MySecureTestSecret1337".to_string());

    for challenge in TEST_CHALLENGES.iter().take(5) {
        c.bench_function("work pow v1", |b| b.iter(|| Pow::work(&challenge).unwrap()));
    }

    let worked = Pow::work(TEST_CHALLENGES.first().unwrap()).unwrap();
    println!("{}", worked);
    Pow::validate(&worked).unwrap();
    c.bench_function("validate pow", |b| {
        b.iter(|| Pow::validate(&worked).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
