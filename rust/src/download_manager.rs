use std::sync::mpsc;
use std::thread;
use std::path::Path;

pub struct DownloadManager {
    workers: usize,
    sender: mpsc::Sender<DownloadTask>,
    receiver: mpsc::Receiver<DownloadTask>,
}

impl DownloadManager {
    pub fn new(workers: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        DownloadManager {
            workers,
            sender,
            receiver,
        }
    }

    pub fn start(&self) {
        for _ in 0..self.workers {
            let receiver = self.receiver.clone();
            thread::spawn(move || {
                while let Ok(task) = receiver.recv() {
                    task.download();
                }
            });
        }
    }

    pub fn add_download(&self, package: PackageMetadata, dest: &Path) {
        let task = DownloadTask {
            package,
            dest: dest.to_path_buf(),
        };
        self.sender.send(task).unwrap();
    }
}

struct DownloadTask {
    package: PackageMetadata,
    dest: std::path::PathBuf,
}

impl DownloadTask {
    fn download(self) {
        // Find appropriate adapter for this package
        // ...
        // Download using selected adapter
        // ...
    }
}