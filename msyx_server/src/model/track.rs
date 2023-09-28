use std::{default, vec};

use crate::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Track {
    pub path: PathBuf,
    // pub tags: HashMap<String, TrackTag>,
}

/*impl Track {
    pub fn new<T>(path: T) -> Result<Track>
    where
        T: Into<PathBuf>,
    {
        let path = path.into();
        let mut track = Track {
            path: path.clone(),
            ..Track::default()
        };
        let probed = Self::probe(&path)?;
        let mut reader = probed.format;
        loop {
            if let Some(meta) = reader.metadata().current() {
                for t in meta.tags() {
                    let track_tag = t.to_owned();
                    let key = track_tag.key.to_string().clone();
                    let value = track_tag.value.to_string().clone();
                    match track.tags.get_mut(&key) {
                        Some(s) => {
                            s.update(value);
                        }
                        None => {
                            track.tags.insert(key, TrackTag::Tag(value));
                        }
                    };
                }
            }
            if reader.metadata().is_latest() {
                break;
            }
        }
        Ok(track)
    }

    fn probe<T: Into<PathBuf>>(path: T) -> Result<ProbeResult> {
        let path = path.into();
        let src = std::fs::File::open(&path).expect("failed to open media");
        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        let hint = Hint::new();
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        // Probe the media source.
        let probe = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;
        Ok(probe)
    }
}*/
