use downloader::Downloader;
use std::thread;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

// Define a custom progress reporter:
struct SimpleReporterPrivate {
    last_update: std::time::Instant,
    max_progress: Option<u64>,
    message: String,
}
struct SimpleReporter {
    private: std::sync::Mutex<Option<SimpleReporterPrivate>>,
}

impl SimpleReporter {
    fn create() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            private: std::sync::Mutex::new(None),
        })
    }
}

impl downloader::progress::Reporter for SimpleReporter {
    fn setup(&self, max_progress: Option<u64>, message: &str) {
        let private = SimpleReporterPrivate {
            last_update: std::time::Instant::now(),
            max_progress,
            message: message.to_owned(),
        };

        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            let max_bytes = match p.max_progress {
                Some(bytes) => format!("{:?}", bytes),
                None => "{unknown}".to_owned(),
            };
            let bytes = max_bytes.parse::<u64>().unwrap();
            let mut downloaded = 0;

            let pb = ProgressBar::new(bytes);
            pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

            while downloaded < bytes {
                let new = min(downloaded + 223211, bytes);
                downloaded = new;
                pb.set_position(new);
                thread::sleep(Duration::from_millis(12));
            }
        }
    }

    fn set_message(&self, _message: &str) {
        //println!("test file: Message changed to: {}", message);
    }

    fn done(&self) {
        let mut guard = self.private.lock().unwrap();
        *guard = None;
        //println!("test file: [DONE]");
    }
}

pub fn download(link: &str) -> String{
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("/oneclickdriverinstalltemp"))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl = downloader::Download::new(link);

    let dl = dl.progress(SimpleReporter::create());

    let result = downloader.download(&[dl]).unwrap();

    let mut downloaded = "error".to_string();
    for r in result {
        match r {
            Err(e) => {
                println!("Error: {}", e.to_string()); 
            },
            Ok(s) => {
                println!("Success: {}", &s); 
                downloaded = "success".to_string();
            },
        };
    }
    return downloaded;
}
