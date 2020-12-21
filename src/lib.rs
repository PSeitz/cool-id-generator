/*!
This is a cool-id-generator.

It makes memorable ids.
honest-turbo-tailor-gregory, romantic-robot-chicken-kenneth and happy-ultra-barista-shane would approve.

```
#[macro_use]
use cool_id_generator::{get_id, Size};

fn main() {
    let my_id = get_id(Size::Medium);
    println!("{:?}", my_id);
    let my_long_id = get_id(Size::Long);
    println!("{:?}", my_long_id);
}
```
*/
#![no_std]

#[macro_use]
extern crate alloc;
use alloc::string::String;

pub mod words;
use words::{ADJECTIVES, ANIMALS, ANIMAL_PREFIX, JOBS, JOBS_PREFIX, NAMES};

use rand::seq::SliceRandom;
use rand::thread_rng;

// mod adjective;
// mod animal;
// mod job;
// mod names;
// mod prefix;

#[inline]
pub fn get_id(size: Size) -> String {
    let mut rng = thread_rng();

    let animal_or_job = {
        if rand::random::<bool>() {
            (
                ANIMAL_PREFIX.choose(&mut rng).unwrap(),
                ANIMALS.choose(&mut rng).unwrap(),
            )
        } else {
            (
                JOBS_PREFIX.choose(&mut rng).unwrap(),
                JOBS.choose(&mut rng).unwrap(),
            )
        }
    };

    match size {
        Size::Medium => {
            let adj1 = ADJECTIVES.choose(&mut rng).unwrap();
            let name = NAMES.choose(&mut rng).unwrap();
            format!("{}{}-{}-{}", adj1, animal_or_job.0, animal_or_job.1, name)
        }
        Size::Long => {
            let name = NAMES.choose(&mut rng).unwrap();
            let adj1 = ADJECTIVES.choose(&mut rng).unwrap();
            let adj2 = ADJECTIVES.choose(&mut rng).unwrap();
            format!(
                "{}-the-{}-and-{}{}-{}",
                name, adj1, adj2, animal_or_job.0, animal_or_job.1
            )
        }
        Size::VeryLong => {
            let name = NAMES.choose(&mut rng).unwrap();
            let name2 = NAMES.choose(&mut rng).unwrap();
            let adj1 = ADJECTIVES.choose(&mut rng).unwrap();
            let adj2 = ADJECTIVES.choose(&mut rng).unwrap();
            format!(
                "{}-{}-the-{}-and-{}{}-{}",
                name, name2, adj1, adj2, animal_or_job.0, animal_or_job.1
            )
        }
    }
}

/// Size which can be cast into `usize` to use as the size of the output byte array e.g.
/// ## Example
/// ```
/// use cool_id_generator::Size;
///
/// fn main() {
///     const size: usize = Size::Medium as usize;
///     let byte_array: [u8; size] = [0u8; size];
/// }
/// ```
pub enum Size {
    /// Creates ids in the format of {adjective}-{prefix}-{animal|job}-{name}
    /// e.g. "unpleasant-steampunk-poet-gerald"
    ///
    /// Generates 1 billion combinations
    Medium = get_id_max_len() as isize,
    /// Creates ids in the format of {name}-the-{adjective}-and-{adjective}-prefix-{animal|job}
    /// e.g. "unpleasant-steampunk-poet-gerald"
    ///
    /// Generates 115 billion combinations
    Long = get_long_id_max_len() as isize,
    /// Creates ids in the format of {name1}-{name2}-the-{adjective}-and-{adjective}-prefix-{animal|job}
    /// e.g. "unpleasant-steampunk-poet-gerald"
    ///
    /// Generates 10^15 combinations (or 2088136477473228)
    VeryLong = get_very_long_id_max_len() as isize,
}

const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

// returns maximum byte lengh of the given array
const fn get_max_len(items: &[&str]) -> usize {
    let mut i = 0;
    let mut largest = 0;
    while i < items.len() {
        let len = items[i].len();
        if len > largest {
            largest = len
        };
        i += 1;
    }
    largest
}

const fn get_very_long_id_max_len() -> usize {
    max(get_max_len(&ANIMAL_PREFIX), get_max_len(&JOBS_PREFIX))
        + get_max_len(&NAMES)
        + get_max_len(&NAMES)
        + get_max_len(&ADJECTIVES)
        + get_max_len(&ADJECTIVES)
        + get_max_len(&ANIMALS)
        + get_max_len(&JOBS)
}

const fn get_long_id_max_len() -> usize {
    max(get_max_len(&ANIMAL_PREFIX), get_max_len(&JOBS_PREFIX))
        + get_max_len(&NAMES)
        + get_max_len(&ADJECTIVES)
        + get_max_len(&ADJECTIVES)
        + get_max_len(&ANIMALS)
        + get_max_len(&JOBS)
}

const fn get_id_max_len() -> usize {
    max(get_max_len(&ANIMAL_PREFIX), get_max_len(&JOBS_PREFIX))
        + get_max_len(&NAMES)
        + get_max_len(&ADJECTIVES)
        + get_max_len(&ANIMALS)
        + get_max_len(&JOBS)
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
