use indicatif::{ProgressBar, ProgressStyle};


pub(crate) fn test(len: u64) {
    let pd = ProgressBar::new(len);
    pd.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos:>7} {len:7} [{elapsed_precise}]")
            .unwrap()
            .progress_chars("#>-")
    );

    for i in 0..100 {
        pd.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}