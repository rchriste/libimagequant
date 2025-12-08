#![feature(test)]

extern crate test;
use core::mem::MaybeUninit;
use test::Bencher;
use std::env;
use std::path::PathBuf;

use imagequant::*;

fn load_bench_image() -> lodepng::Bitmap<lodepng::RGBA> {
    let raw = env::var("BENCH_IMAGE").expect("Set BENCH_IMAGE to a PNG path for benchmarks");
    let resolved: PathBuf = if let Some(stripped) = raw.strip_prefix("~/") {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(stripped)
    } else if raw == "~" {
        PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".to_string()))
    } else {
        PathBuf::from(raw)
    };
    lodepng::decode32_file(resolved)
        .unwrap_or_else(|e| panic!("Failed to load BENCH_IMAGE PNG: {e}"))
}

#[bench]
fn histogram(b: &mut Bencher) {
    let img = load_bench_image();
    let liq = Attributes::new();
    b.iter(move || {
        let mut img = liq.new_image(&*img.buffer, img.width, img.height, 0.).unwrap();
        let mut hist = Histogram::new(&liq);
        hist.add_image(&liq, &mut img).unwrap();
    });
}

#[bench]
fn remap_ord(b: &mut Bencher) {
    let img = load_bench_image();
    let mut buf = vec![MaybeUninit::uninit(); img.width * img.height];
    let mut liq = Attributes::new();
    liq.set_speed(10).unwrap();
    let mut img = liq.new_image(img.buffer, img.width, img.height, 0.).unwrap();
    liq.set_max_colors(256).unwrap();
    let mut res = liq.quantize(&mut img).unwrap();
    res.set_dithering_level(0.).unwrap();
    b.iter(move || {
        res.remap_into(&mut img, &mut buf).unwrap();
        res.remap_into(&mut img, &mut buf).unwrap();
    });
}

#[bench]
fn kmeans(b: &mut Bencher) {
    b.iter(_unstable_internal_kmeans_bench());
}

#[bench]
fn remap_floyd(b: &mut Bencher) {
    let img = load_bench_image();
    let mut buf = vec![MaybeUninit::uninit(); img.width * img.height];
    let mut liq = Attributes::new();
    liq.set_speed(10).unwrap();
    let mut img = liq.new_image(img.buffer, img.width, img.height, 0.).unwrap();
    let mut res = liq.quantize(&mut img).unwrap();
    res.set_dithering_level(1.).unwrap();
    b.iter(move || {
        res.remap_into(&mut img, &mut buf).unwrap();
        res.remap_into(&mut img, &mut buf).unwrap();
    });
}

#[bench]
fn quantize_s8(b: &mut Bencher) {
    let img = load_bench_image();
    let mut liq = Attributes::new();
    liq.set_speed(8).unwrap();
    b.iter(move || {
        let mut img = liq.new_image(&*img.buffer, img.width, img.height, 0.).unwrap();
        liq.quantize(&mut img).unwrap();
    });
}

#[bench]
fn quantize_s1(b: &mut Bencher) {
    let img = load_bench_image();
    let mut liq = Attributes::new();
    liq.set_speed(1).unwrap();
    b.iter(move || {
        let mut img = liq.new_image(&*img.buffer, img.width, img.height, 0.).unwrap();
        liq.quantize(&mut img).unwrap();
    });
}
