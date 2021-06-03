use console::{style, Emoji};
use indicatif::{
    HumanBytes, HumanDuration, MultiProgress, ParallelProgressIterator, ProgressBar,
    ProgressIterator, ProgressStyle,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::min;
use std::thread;
use std::time::{Duration, Instant};

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn main() {
    let started = Instant::now();

    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠁⠂⠄⡀⡀⢀⠠⠐⠈⠈")
        .template("{prefix:.bold} {spinner} {wide_msg}");

    // progress_bar 单一进度条
    let progress_bar = ProgressBar::new(10_000);
    progress_bar.set_style(spinner_style.clone());
    // 0 .. 999
    for i in 0..1000 {
        progress_bar.set_message(&format!("render {}", i));
        thread::sleep(Duration::from_millis(1));
        progress_bar.inc(1);
    }
    progress_bar.finish();

    // 迭代器进度条
    // 1 .. 10_000
    for _ in (1..=10_000).progress() {}

    let v: Vec<_> = (0..10_000).collect();
    let _: Vec<_> = v.par_iter().progress().map(|i| i + 1).collect();

    // 多进度条
    let main_bar = MultiProgress::new();
    for i in 0..4 {
        let count = 1_000;
        let bar = main_bar.add(ProgressBar::new(count));
        bar.set_style(spinner_style.clone());
        bar.set_prefix(&format!(
            "{}",
            style(format!("{}/{} {}", i, 4, SPARKLE)).bold().yellow()
        ));
        let _ = thread::spawn(move || {
            for c in 0..count {
                bar.set_message(&format!("{}/{}", c, count));
                thread::sleep(Duration::from_millis(1));
                bar.inc(1);
            }
            bar.finish();
        });
    }
    main_bar.join().unwrap();

    let mut downloaded = 0;
    let total_size = 100_1000;
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})")
                            .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 2000, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(1));
    }

    pb.finish_with_message("downloaded");

    // 可读时间
    println!("the script take {}", HumanDuration(started.elapsed()));
    // 可读 bytes 大小
    println!("the file is {} large", HumanBytes(10_000));
}
