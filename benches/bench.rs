#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use cool_id_generator::get_very_long_id;
    use cool_id_generator::get_long_id;
    use cool_id_generator::get_id;
    #[bench]
    fn bench_id(b: &mut Bencher) {
        b.iter(|| {
            get_id()
        });
    }
    #[bench]
    fn bench_long_id(b: &mut Bencher) {
        b.iter(|| {
            get_long_id()
        });
    }
    #[bench]
    fn bench_very_long_id(b: &mut Bencher) {
        b.iter(|| {
            get_very_long_id()
        });
    }
}