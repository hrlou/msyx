use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}};

use crate::prelude::*;

pub struct Scanner {
    pwd: PathBuf,
    handle: Option<thread::JoinHandle<()>>,
    alive: Arc<AtomicBool>,
    queue: Arc<Mutex<Vec<PathBuf>>>
}

impl Scanner {
    /// Create a Scanner with a given directory to scan
    pub fn new<T>(p: T) -> Self
        where PathBuf: From<T>
    {   
        let p = p.into();
        Self {
            pwd: p,
            handle: None,
            alive: Arc::new(AtomicBool::new(false)),
            queue: Arc::new(Mutex::new(Vec::new())),
         }
    }

    pub fn scan(&mut self, path: Option<PathBuf>) -> Result<()> {
        let path = path.unwrap_or(self.pwd.clone());
        for path in fs::read_dir(&path)? {
            let path = path?.path();
            match path.is_dir() {
                true => self.scan(Some(path.clone()))?,
                false => {
                    let mut lock = self.queue.lock().unwrap();
                },
            }
        };
        Ok(())
    }

    /// Start the scanner
    /// Go through all directories recursively and find audio files
    pub fn start<F>(&mut self, f: F)
        where F: 'static + Send + FnMut() -> (),
    {
        self.alive.store(true, Ordering::SeqCst);
        let alive = self.alive.clone();
    
        self.handle = Some(thread::spawn(move || {
            let mut f = f;
            f();
        }));
        // todo!()
    }

    /// Stop the scanner
    pub fn stop(&mut self) {

    }
}