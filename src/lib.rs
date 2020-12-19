/*!
This is a cool-id-generator.

It makes memorable ids.
honest-turbo-tailor-gregory, romantic-robot-chicken-kenneth and happy-ultra-barista-shane would approve.

```rust
#[macro_use]
use cool_id_generator::{get_id, get_long_id};

fn main() {
    let my_id = get_id();
    println!("{:?}", my_id);
    let my_long_id = get_long_id();
    println!("{:?}", my_long_id);
}
```
*/
use crate::job::JOBS;
use crate::animal::ANIMALS;
use crate::adjective::ADJECTIVES;
use crate::names::NAMES;

use rand::thread_rng;
use rand::seq::SliceRandom;

mod adjective;
mod animal;
mod names;
mod job;

/// Creates ids in the format of {adjective}-{prefix}-{animal|job}-{name}
/// e.g. "unpleasant-steampunk-poet-gerald"
///
/// Generates 1 billion combinations
#[inline]
pub fn get_id() -> String {
    let mut rng = thread_rng();

    let name = NAMES.choose(&mut rng).unwrap();
    let adj1 = ADJECTIVES.choose(&mut rng).unwrap();
    let animal = ANIMALS.choose(&mut rng).unwrap();
    let job = JOBS.choose(&mut rng).unwrap();

    // let combinations_animals = NAMES.len() * ADJECTIVES.len() * ANIMALS.len();
    // let combinations_jobs = NAMES.len() * ADJECTIVES.len() * JOBS.len();

    let is_animal_or_job = rand::random();
    if is_animal_or_job {
        // It's an animal!
        let prefix = get_animal_prefix(&mut rng);
        // println!("Total Combinations {:?}", combinations_animals * prefixes.len());
        format!("{}{}-{}-{}", adj1, prefix, animal, name)
    }else{
        // It's a job!
        let prefix = get_job_prefix(&mut rng);
        // println!("Total Combinations {:?}", combinations_jobs * prefixes.len());
        format!("{}{}-{}-{}", adj1, prefix, job, name)
    }

}

/// Creates ids in the format of {name}-the-{adjective}-and-{adjective}-prefix-{animal|job}
/// e.g. "unpleasant-steampunk-poet-gerald"
///
/// Generates 115 billion combinations
#[inline]
pub fn get_long_id() -> String {
    let mut rng = thread_rng();

    let name = NAMES.choose(&mut rng).unwrap();
    let adj1 = ADJECTIVES.choose(&mut rng).unwrap();
    let adj2 = ADJECTIVES.choose(&mut rng).unwrap();
    let animal = ANIMALS.choose(&mut rng).unwrap();
    let job = JOBS.choose(&mut rng).unwrap();

    // let combinations_animals = NAMES.len() * ADJECTIVES.len() * ADJECTIVES.len() * ANIMALS.len();
    // let combinations_jobs = NAMES.len() * ADJECTIVES.len() * ADJECTIVES.len() * JOBS.len();

    let is_animal_or_job = rand::random();
    if is_animal_or_job {
        // It's an animal!
        let prefix = get_animal_prefix(&mut rng);

        // println!("Total Combinations {:?}", combinations_animals * prefixes.len());
        format!("{}-the-{}-and-{}{}-{}", name, adj1, adj2, prefix, animal)
    }else{
        // It's a job!
        let prefix = get_job_prefix(&mut rng);
        // println!("Total Combinations {:?}", combinations_jobs * prefixes.len());
        format!("{}-the-{}-and-{}{}-{}", name, adj1, adj2, prefix, job)
    }

}

/// Creates ids in the format of {name1}-{name2}-the-{adjective}-and-{adjective}-prefix-{animal|job}
/// e.g. "unpleasant-steampunk-poet-gerald"
///
/// Generates 10^15 combinations (or 2088136477473228)
#[inline]
pub fn get_very_long_id() -> String {
    let mut rng = thread_rng();

    let name = NAMES.choose(&mut rng).unwrap();
    let name2 = NAMES.choose(&mut rng).unwrap();
    let adj1 = ADJECTIVES.choose(&mut rng).unwrap();
    let adj2 = ADJECTIVES.choose(&mut rng).unwrap();
    let animal = ANIMALS.choose(&mut rng).unwrap();
    let job = JOBS.choose(&mut rng).unwrap();

    // let _combinations_animals = NAMES.len() * NAMES.len() * ADJECTIVES.len() * ADJECTIVES.len() * ANIMALS.len();
    // let _combinations_jobs = NAMES.len() * NAMES.len() * ADJECTIVES.len() * ADJECTIVES.len() * JOBS.len();

    let is_animal_or_job = rand::random();
    if is_animal_or_job {
        // It's an animal!
        let prefix = get_animal_prefix(&mut rng);

        // println!("Total Combinations {:?}", combinations_animals * prefixes.len());
        format!("{}-{}-the-{}-and-{}{}-{}", name, name2, adj1, adj2, prefix, animal)
    }else{
        // It's a job!
        let prefix = get_job_prefix(&mut rng);
        // println!("Total Combinations {:?}", combinations_jobs * prefixes.len());
        format!("{}-{}-the-{}-and-{}{}-{}", name, name2, adj1, adj2, prefix, job)
    }

}

#[inline]
fn get_animal_prefix(rng: &mut rand::rngs::ThreadRng) -> &'static str {
    let prefixes = ["-robot","-mecha","-dino","-laser","-turbo","-rocket","-dressed","-space"];
    let prefix = prefixes.choose(rng).unwrap();
    prefix
}

#[inline]
fn get_job_prefix(rng: &mut rand::rngs::ThreadRng) -> &'static str {
    let prefixes = ["-robot","-laser","-turbo","-space","-steampunk","-jobless","-homeless"];
    let prefix = prefixes.choose(rng).unwrap();
    prefix
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {

        // let yo: Vec<String> = (0..1000).map(|_|get_id()).collect();
        // println!("{:?}", yo.join(" "));
        println!("{}", get_id());
        println!("{}", get_id());
        println!("{}", get_id());
        println!("{}", get_id());
        println!("{}", get_id());
        println!("{}", get_id());
        println!("{}", get_id());
        println!("{}", get_id());
        println!("{}", get_id());

        println!("{}", get_long_id());
        println!("{}", get_long_id());
        println!("{}", get_long_id());
        println!("{}", get_long_id());
        println!("{}", get_long_id());
        println!("{}", get_long_id());
        println!("{}", get_long_id());
        println!("{}", get_long_id());
        println!("{}", get_long_id());

        println!("{}", get_very_long_id());
        println!("{}", get_very_long_id());
        println!("{}", get_very_long_id());
        println!("{}", get_very_long_id());
        println!("{}", get_very_long_id());
        println!("{}", get_very_long_id());
        println!("{}", get_very_long_id());
        println!("{}", get_very_long_id());
        println!("{}", get_very_long_id());
    }
}

