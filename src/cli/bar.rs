use indicatif::{ProgressBar, ProgressStyle};

pub struct Bar {
    pd: ProgressBar,
}

impl Bar {
    pub fn new(len: u64) -> ProgressBar {
        let pd = ProgressBar::new(len);
        pd.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{bar:40.cyan/blue}] {pos:>7} {len:7} [{elapsed_precise}]",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        return pd;
    }
    pub fn inc(&mut self, len: u64) {
        self.pd.inc(len);
    }
}
