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
#![no_std]

#[macro_use]
extern crate alloc;
use alloc::string::String;


use crate::job::JOBS;
use crate::animal::ANIMALS;
use crate::adjective::ADJECTIVES;
use crate::names::NAMES;
use crate::prefix::JOBS_PREFIX;
use crate::prefix::ANIMAL_PREFIX;

use rand::thread_rng;
use rand::seq::SliceRandom;

mod adjective;
mod animal;
mod names;
mod job;
mod prefix;

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
        // println!("Total Combinations {:?}", combinations_animals * ANIMAL_PREFIX.len());
        format!("{}{}-{}-{}", adj1, prefix, animal, name)
    }else{
        // It's a job!
        let prefix = get_job_prefix(&mut rng);
        // println!("Total Combinations {:?}", combinations_jobs * JOBS_PREFIX.len());
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

        // println!("Total Combinations {:?}", combinations_animals * ANIMAL_PREFIX.len());
        format!("{}-the-{}-and-{}{}-{}", name, adj1, adj2, prefix, animal)
    }else{
        // It's a job!
        let prefix = get_job_prefix(&mut rng);
        // println!("Total Combinations {:?}", combinations_jobs * JOBS_PREFIX.len());
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

        // println!("Total Combinations {:?}", combinations_animals * ANIMAL_PREFIX.len());
        format!("{}-{}-the-{}-and-{}{}-{}", name, name2, adj1, adj2, prefix, animal)
    }else{
        // It's a job!
        let prefix = get_job_prefix(&mut rng);
        // println!("Total Combinations {:?}", combinations_jobs * JOBS_PREFIX.len());
        format!("{}-{}-the-{}-and-{}{}-{}", name, name2, adj1, adj2, prefix, job)
    }

}

const fn max(a: usize, b: usize) -> usize { [a, b][(a < b) as usize] }

// returns maximum byte lengh of the given array
const fn get_max_len(items: &[&str]) -> usize {
    let mut i = 0;
    let mut largest = 0;
    while i < items.len() {
        let len = items[i].len();
        if len > largest { largest = len };
        i += 1;
    }
    largest
}

#[inline]
/// Returns the theoretical maximum byte length of the string returned by `get_very_long_id`
pub const fn get_very_long_id_max_len() -> usize {
    max(get_max_len(&ANIMAL_PREFIX), get_max_len(&JOBS_PREFIX)) +
    get_max_len(&NAMES) + 
    get_max_len(&NAMES) + 
    get_max_len(&ADJECTIVES) + 
    get_max_len(&ADJECTIVES) + 
    get_max_len(&ANIMALS) + 
    get_max_len(&JOBS)
}

#[inline]
/// Returns the theoretical maximum byte length of the string returned by `get_long_id`
pub const fn get_long_id_max_len() -> usize {
    max(get_max_len(&ANIMAL_PREFIX), get_max_len(&JOBS_PREFIX)) +
    get_max_len(&NAMES) + 
    get_max_len(&ADJECTIVES) + 
    get_max_len(&ADJECTIVES) + 
    get_max_len(&ANIMALS) + 
    get_max_len(&JOBS) 
}

#[inline]
/// Returns the theoretical maximum byte length of the string returned by `get__id`
pub const fn get_id_max_len() -> usize {
    max(get_max_len(&ANIMAL_PREFIX), get_max_len(&JOBS_PREFIX)) +
    get_max_len(&NAMES) + 
    get_max_len(&ADJECTIVES) + 
    get_max_len(&ANIMALS) + 
    get_max_len(&JOBS)
}

#[inline]
fn get_animal_prefix(rng: &mut rand::rngs::ThreadRng) -> &'static str {
    let prefix = ANIMAL_PREFIX.choose(rng).unwrap();
    prefix
}

#[inline]
fn get_job_prefix(rng: &mut rand::rngs::ThreadRng) -> &'static str {
    let prefix = JOBS_PREFIX.choose(rng).unwrap();
    prefix
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::*;
    #[test]
    fn it_works() {

        // let yo: Vec<String> = (0..1000).map(|_|get_id()).collect();
        // println!("{:?}", yo.join(" "));

        // println!("{}", get_id());
        // println!("{}", get_id());
        // println!("{}", get_id());
        // println!("{}", get_id());
        // println!("{}", get_id());
        // println!("{}", get_id());
        // println!("{}", get_id());
        // println!("{}", get_id());
        // println!("{}", get_id());

        // println!("{}", get_long_id());
        // println!("{}", get_long_id());
        // println!("{}", get_long_id());
        // println!("{}", get_long_id());
        // println!("{}", get_long_id());
        // println!("{}", get_long_id());
        // println!("{}", get_long_id());
        // println!("{}", get_long_id());
        // println!("{}", get_long_id());

        // println!("{}", get_very_long_id());
        // println!("{}", get_very_long_id());
        // println!("{}", get_very_long_id());
        // println!("{}", get_very_long_id());
        // println!("{}", get_very_long_id());
        // println!("{}", get_very_long_id());
        // println!("{}", get_very_long_id());
        // println!("{}", get_very_long_id());
        // println!("{}", get_very_long_id());
        // println!("{}", get_id_max_len());
        // println!("{}", get_long_id_max_len());
        // println!("{}", get_very_long_id_max_len());
    }
}

