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

mod adjective;
mod animal;
mod names;
mod job;

/// Creates ids in the format of {adjective}-{suffix}-{animal|job}-{name}
/// e.g. "unpleasant-steampunk-poet-gerald"
/// Generates 55 million combinations
pub fn get_id() -> String {
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    let mut rng = thread_rng();

    let name = NAMES.choose(&mut rng).unwrap();
    let adj1 = ADJECTIVES.choose(&mut rng).unwrap();
    let _adj2 = ADJECTIVES.choose(&mut rng).unwrap();
    let animal = ANIMALS.choose(&mut rng).unwrap();
    let job = JOBS.choose(&mut rng).unwrap();

    // let combinations_animals = NAMES.len() * ADJECTIVES.len() * ANIMALS.len();
    // let combinations_jobs = NAMES.len() * ADJECTIVES.len() * JOBS.len();

    let is_animal_or_job = rand::random();
    if is_animal_or_job {
        // It's an animal!
        let suffixes = ["-robot","-mecha","-dino","-laser","-turbo","-rocket","-dressed","-space"];
        let suffix = suffixes.choose(&mut rng).unwrap();
        // println!("Total Combinations {:?}", combinations_animals * suffixes.len());
        format!("{}{}-{}-{}", adj1, suffix, animal, name)
    }else{
        // It's a job!
        let suffixes = ["-robot","-laser","-turbo","-space","-steampunk","-jobless","-homeless"];
        let suffix = suffixes.choose(&mut rng).unwrap();
        // println!("Total Combinations {:?}", combinations_jobs * suffixes.len());
        format!("{}{}-{}-{}", adj1, suffix, job, name)
    }

}

/// Creates ids in the format of {name}-the-{adjective}-and-{adjective}-suffix-{animal|job}
/// e.g. "unpleasant-steampunk-poet-gerald"
/// Generates 6 billion combinations
pub fn get_long_id() -> String {
    use rand::thread_rng;
    use rand::seq::SliceRandom;
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
        let suffixes = ["-robot","-mecha","-dino","-laser","-turbo","-rocket","-dressed","-space"];
        let suffix = suffixes.choose(&mut rng).unwrap();

        // println!("Total Combinations {:?}", combinations_animals * suffixes.len());
        format!("{}-the-{}-and-{}{}-{}", name, adj1, adj2, suffix, animal)
    }else{
        // It's a job!
        let suffixes = ["-robot","-laser","-turbo","-space","-steampunk","-jobless","-homeless"];
        let suffix = suffixes.choose(&mut rng).unwrap();
        // println!("Total Combinations {:?}", combinations_jobs * suffixes.len());
        format!("{}-the-{}-and-{}{}-{}", name, adj1, adj2, suffix, job)
    }

}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        println!("{:?}", get_id());
        println!("{:?}", get_id());
        println!("{:?}", get_id());
        println!("{:?}", get_id());
        println!("{:?}", get_id());
        println!("{:?}", get_id());
        println!("{:?}", get_id());
        println!("{:?}", get_id());
        println!("{:?}", get_id());

        println!("{:?}", get_long_id());
        println!("{:?}", get_long_id());
        println!("{:?}", get_long_id());
        println!("{:?}", get_long_id());
        println!("{:?}", get_long_id());
        println!("{:?}", get_long_id());
        println!("{:?}", get_long_id());
        println!("{:?}", get_long_id());
        println!("{:?}", get_long_id());
    }
}