#![no_main]
use libfuzzer_sys::fuzz_target;
use plotters::data::Quartiles;

fuzz_target!(|input: (Vec<u8>, Vec<u16>, Vec<u32>, Vec<f32>)| {
    let (a, b, c, d) = input;
    if a.is_empty() || b.is_empty() || c.is_empty() || d.is_empty() {
        return;
    }
    if d.iter().any(|f| f.is_nan()) {
        return;
    }

    let quartiles = Quartiles::new(&a);
    quartiles.values();
    quartiles.median();

    let quartiles = Quartiles::new(&b);
    quartiles.values();
    quartiles.median();

    let quartiles = Quartiles::new(&c);
    quartiles.values();
    quartiles.median();

    let quartiles = Quartiles::new(&d);
    quartiles.values();
    quartiles.median();
});