#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use cool_id_generator::{get_id, Size};
    use test::Bencher;
    #[bench]
    fn bench_id(b: &mut Bencher) {
        b.iter(|| get_id(Size::Medium));
    }
    #[bench]
    fn bench_long_id(b: &mut Bencher) {
        b.iter(|| get_id(Size::Long));
    }
    #[bench]
    fn bench_very_long_id(b: &mut Bencher) {
        b.iter(|| get_id(Size::VeryLong));
    }
}
