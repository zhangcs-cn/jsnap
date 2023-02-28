use std::fmt::Write;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

pub struct Bar {
    pd: ProgressBar,
}

impl Bar {
    pub fn new(msg: String, len: u64) -> ProgressBar {
        let pd = ProgressBar::new(len);
        pd.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        pd.set_message(msg);
        return pd;
    }
}
