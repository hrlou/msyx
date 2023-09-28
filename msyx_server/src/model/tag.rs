use crate::prelude::*;

use ::phf::{phf_map, Map};

#[derive(Debug, Clone, PartialEq)]
/// The Tag key
/// Using what's standard for MusicBrainz
pub enum TagKey {
    Unknown(String),
    Title,
    Artist,
    Album,
    TrackNumber,
    Date,
    AcoustID,
    AlbumArtist,
    AlbumArtistSortOrder,
    ArtistSortOrder,
    ArtistWebsite,
    Artists,
    Asin,
    CatalogueNumber,
    DiscNumber,
    Genre,
    Media,
    MusicBrainzArtistId,
    MusicBrainzRecordingId,
    MusicBrainzReleaseArtistId,
    MusicBrainzReleaseGroupId,
    MusicBrainzReleaseId,
    MusicBrainzTrackId,
    OriginalReleaseDate,
    OriginalYear,
    RecordLabel,
    ReleaseCountry,
    ReleaseStatus,
    ReleaseType,
    Script,
    TotalDiscs,
    TotalTracks,
}

static TAGKEYS: phf::Map<&'static str, TagKey> = phf_map! {
    "title" => TagKey::Title,
    "artist" => TagKey::Artist,
    "album" => TagKey::Album,
    "tracknumber" => TagKey::TrackNumber,
    "date" => TagKey::Date,
    "acoustid_id" => TagKey::AcoustID,
    "albumartist" => TagKey::AlbumArtist,
    "albumartistsort" => TagKey::AlbumArtistSortOrder,
    "artistsort" => TagKey::ArtistSortOrder,
    "website" => TagKey::ArtistWebsite,
    "artists" => TagKey::Artists,
    "asin" => TagKey::Asin,
    "catalognumber" => TagKey::CatalogueNumber,
    "discnumber" => TagKey::DiscNumber,
    "genre" => TagKey::Genre,
    "media" => TagKey::Media,
    "musicbrainz_artistid" => TagKey::MusicBrainzArtistId,
    "musicbrainz_recordingid" => TagKey::MusicBrainzRecordingId,
    "musicbrainz_albumartistid" => TagKey::MusicBrainzReleaseArtistId,
    "musicbrainz_releasegroupid" => TagKey::MusicBrainzReleaseGroupId,
    "musicbrainz_albumid" => TagKey::MusicBrainzReleaseId,
    "musicbrainz_trackid" => TagKey::MusicBrainzTrackId,
    "originaldate" => TagKey::OriginalReleaseDate,
    "originalyear" => TagKey::OriginalYear,
    "label" => TagKey::RecordLabel,
    "releasecountry" => TagKey::ReleaseCountry,
    "releasestatus" => TagKey::ReleaseStatus,
    "releasetype" => TagKey::ReleaseType,
    "script" => TagKey::Script,
    "totaldiscs" => TagKey::TotalDiscs,
    "totaltracks" => TagKey::TotalTracks,
};


impl ToString for TagKey {
    fn to_string(&self) -> String {
        match self {
            TagKey::Unknown(s) => {
                s.to_owned()
            },
            _ => {
                let entry = TAGKEYS
                    .entries()
                    .find(|(k, v)| {
                        v == &self
                    });
                let str = entry.expect("Couldn't convert `TagKey` to `String`");
                let (str, _) = str;
                str.to_string()
            },
        }
    }
}

impl From<String> for TagKey {
    fn from(v: String) -> Self {
        match TAGKEYS.get(&v) {
            Some(key) => key.to_owned(),
            None => TagKey::Unknown(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagValue {
    Literal(String),
    Number(i32),
    List(Vec<TagValue>),
}

impl Into<TagValue> for String {
    fn into(self) -> TagValue {
        TagValue::Literal(self)
    }
}

impl Into<TagValue> for i32 {
    fn into(self) -> TagValue {
        TagValue::Number(self)
    }
}

impl<T> Into<TagValue> for Vec<T>
where
    T: Into<TagValue> + Clone
{
    fn into(self) -> TagValue {
        let v = self
            .iter()
            .map(|v| {
                let v: TagValue = v.to_owned().into();
                v
            })
            .collect();
        TagValue::List(v)
    }
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}


#[derive(Debug, Clone)]
pub struct Tag {
    pub key: TagKey,
    pub value: TagValue,
}

impl Tag {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<TagKey>,
        V: Into<TagValue>,
    {
        Tag { key: key.into(), value: value.into() }   
    }

    pub fn append<V>(&mut self, v: V)
    where
        V: Into<TagValue> + Clone,
    {
        let original_value = self.value.clone();
        match original_value {
            TagValue::List(_) => {},
            s => self.value = TagValue::List(vec![s]),
        };
        if let TagValue::List(values) = &mut self.value {
            let v: TagValue = v.into();
            match v {
                TagValue::List(v) => {
                    let mut v = v.clone();
                    values.append(&mut v);
                },
                value => values.push(value),
            }
        }
    }
}
