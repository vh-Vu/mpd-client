use anyhow::Result;
use tokio::sync::mpsc::Sender;
#[derive(Default,Debug)]
pub struct Status{
    pub volume: i8,
    pub repeat: bool,
    pub random: bool,
    pub single: Mode,
    pub consume: Mode,
    pub partition: String,
    pub playlist: u32,
    pub playlistlength: i32,
    pub mixrampdb: i32,
    pub state: State,
    pub lastloadedplaylist: String,
    pub song: u32,
    pub songid: u32,
    pub elapsed: f32,
    pub bitrate: u32,
    pub duration: f32,
    pub audio: String,
    pub nextsong: u32,
    pub nextsongid: u32,
}

pub fn parse_status(lines: Vec<String>) -> Result<Status>{
    let mut status = Status::default();
    for line in lines{
        let p = line.find(':').ok_or_else(|| anyhow::anyhow!(""))?;
        let key = &line[..p];
        let value = line[p+1..].trim();
        match key {
            "volume" 
                => status.volume = value.parse::<i8>()?,
            "repeat" 
                => status.repeat = value == "1",
            "random" 
                => status.random = value == "1",
            "single" 
                => status.single = match value {
                                        "0" => Mode::Off,
                                        "1" => Mode::On,
                                        _   => Mode::OneShot
                                    },
            "consume" 
                => status.consume = match value {
                                    "0" => Mode::Off,
                                    "1" => Mode::On,
                                    _   => Mode::OneShot
                                },
            "partition" 
                => status.partition = value.to_string(),
            "playlist" 
                => status.playlist = value.parse::<u32>()?,
            "playlistlength" 
                => status.playlistlength = value.parse::<i32>()?,
            "mixrampdb" 
                => status.mixrampdb = value.parse::<i32>()?,
            "state" 
                => status.state = match value {
                                        "play" => State::Play,
                                        "pause" => State::Pause,
                                        _   => State::Stop,
                                    },
            "lastloadedplaylist" 
                => status.lastloadedplaylist = value.to_string(),
            "song" 
                => status.song = value.parse::<u32>()?,            
            "songid" 
                => status.songid = value.parse::<u32>()?,
            "elapsed" 
                => status.elapsed = value.parse::<f32>()?,
            "bitrate" 
                => status.bitrate = value.parse::<u32>()?,
            "duration" 
                => status.duration = value.parse::<f32>()?,
            "audio" 
                => status.audio = value.to_string(),
            "nextsong" 
                => status.nextsong = value.parse::<u32>()?,
            "nextsongid" 
                => status.nextsongid = value.parse::<u32>()?,
            _ => {},
        }
    }
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_success() {
        let raw_data = "volume: 91\n\
            repeat: 0\n\
            random: 0\n\
            single: 0\n\
            consume: 0\n\
            partition: default\n\
            playlist: 19\n\
            playlistlength: 7\n\
            mixrampdb: 0\n\
            state: pause\n\
            lastloadedplaylist: \n\
            song: 1\n\
            songid: 17\n\
            time: 260:299\n\
            elapsed: 259.598\n\
            bitrate: 1139\n\
            duration: 298.640\n\
            audio: 44100:16:2\n\
            nextsong: 2\n\
            nextsongid: 18";
       
        let v: Vec<String> = raw_data.split('\n').map(|l| l.to_string()).collect();
        let result = parse_status(v);
        assert!(result.is_ok()); 
        let status = result.unwrap();
        assert!(matches!(status.state, State::Pause));
        assert_eq!(status.songid,17);
        assert_eq!(status.elapsed, 259.598);
        assert_eq!(status.volume, 91);
        assert!(matches!(status.single, Mode::Off));

    }

    #[test]
    fn test_parse_song_success() {
        let raw_data = "file: alan/Voice of EARTH/01  天女 ～interlude～.flac\n\
                                Last-Modified: 2026-09-14T08:47:16Z\n\
                                Added: 2026-09-07T21:21:32Z\n\
                                Format: 44100:16:2\n\
                                Artist: alan\n\
                                Title: 天女 ～interlude～\n\
                                Album: Voice of EARTH\n\
                                Date: 2009-03-04\n\
                                Genre: Pop\n\
                                Composer: Kazuhito Kikuchi\n\
                                AlbumArtist: alan\n\
                                Track: \n\
                                Disc: 1\n\
                                Label: avex trax\n\
                                Time: 192\n\
                                duration: 192.000";
            
        let v: Vec<String> = raw_data.split('\n').map(|l| l.to_string()).collect();
        let result = parse_song(v);
        assert!(result.is_ok());
        let song = result.unwrap();
        assert_eq!(song.composer,vec!["Kazuhito Kikuchi"]);
        assert_eq!(song.artist,vec!["alan"]);
        // assert_eq!(song.track,None);
        assert_eq!(song.track,0);
        assert_eq!(song.time,192);
        assert_eq!(song.duration,192.000);
        assert_eq!(song.disc,1);
    }

}

#[derive(Default,Debug)]
pub enum State{
    Play,
    #[default]
    Pause,
    Stop,
}
#[derive(Default,Debug)]

pub enum Mode{
    On,
    #[default]
    Off,
    OneShot,
}

use qtbridge::QModelItem;

// #[derive(Default,Debug,PartialEq, Clone)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Song{
    pub file: String,
    pub title: String,
    pub artist: Vec<String>,
    pub album: String,
    pub date: String,
    pub genre: Vec<String>,
    pub composer: Vec<String>,
    pub album_artist: Vec<String>,
    // pub track: Option<u32>,
    // pub disc: Option<u32>,
    pub track: u32,
    pub disc: u32,
    pub label: String,
    pub time: u32,
    pub duration: f32,
    pub id: u32,
    pub pos: u32,
    pub key: String,
    //
    pub year: u16,
    pub bit_deep: u8,
    pub sample_rate: u32, 
}

pub fn parse_song(lines: Vec<String>) -> Result<Song>{
    let mut song = Song::default();
    for line in lines{
        let p = line.find(':').ok_or_else(|| anyhow::anyhow!(""))?;
        let key = &line[..p].to_lowercase();
        let value = line[p+1..].trim();
        match key.as_str() {
            "file" 
                => song.file = value.to_string(),
            "title"
                => song.title = value.to_string(),
            "artist"
                => song.artist.push(value.to_string()),
            "album"
                => song.album = value.to_string(),
            "date" 
                => song.date = value.to_string(),
            "genre" 
                => song.genre.push(value.to_string()),
            "composer"
                => song.composer.push(value.to_string()),
            "albumartist"
                => song.album_artist.push(value.to_string()),
            "track"
                // => song.track = value.parse::<u32>().map_or(None, |v| Some(v)),
                => song.track = value.parse::<u32>().map_or(0, |v| v),
            "disc"
                // => song.disc = value.parse::<u32>().map_or(None, |v| Some(v)),
                => song.disc = value.parse::<u32>().map_or(0, |v| v),

            "label"
                => song.label = value.to_string(),
            "time"
                => song.time = value.parse::<u32>()?,
            "duration"
                => song.duration = value.parse::<f32>()?,
            _ => {},
        }
    }
    Ok(song)
}


#[derive(Debug,Clone)]
pub enum Command{
    Play,
    Next,
    Pause,
    Previous,
    CurrentSong,
    Status,
    GetQueue,
}

impl Command{
    pub fn as_bytes(&self) -> &[u8]{
        match self {
            Command::Play => b"play\n",
            Command::Pause => b"pause\n",
            Command::Next => b"next\n",
            Command::Previous => b"previous\n",
            Command::CurrentSong => b"currentsong\n",
            Command::Status => b"status\n",
            Command::GetQueue => b"playlistinfo\n",
        }
    }
}


pub enum DataEvent {
    Status(Status),
    CurrentSong(Song),
    Queue(Vec<Song>),
}

#[derive(Debug)]
pub enum MpdWorkerEvent{
    Connected(Sender<Command>),
    Disconnected,
    Changed(ChangedEvent),
}


#[derive(Debug)]
pub enum ChangedEvent {
    Database,
    Update,
    Playlist,
    StoredPlaylist,
    Player,
    Mixer,
    Output,
    Options,
    Partition,
    Sticker,
    Subscription,
    Message,
    Neihbor,
    Mount,
    Unknown,
}

#[derive(Default, Debug,  PartialEq, Clone,QModelItem)]
pub struct SongS{
    pub file: String,
    pub title: String,
    pub artist: Vec<String>,
    pub artist_str: String,
    pub album: String,
    // pub alo: Vec<String>,
    // pub album: Vec<String>,
    // pub date: String,
    // pub genre: Vec<String>,
    // pub composer: Vec<String>,
    // pub album_artist: Vec<String>,
    pub track: u32,
    pub cover: String,
    // // pub disc: Option<u32>,
    // pub track: u32,
    // pub disc: u32,
    // pub label: String,
    // pub time: u32,
    pub duration: u32,
}

#[qtbridge::qobject]
impl SongS {
    qproperty!("title", Member = title,  Notify = title_changed);
    qproperty!("file", Member = file, Notify = file_changed);
    qproperty!("album", Member = album, Notify = album_changed);
    qproperty!("artist", Member = artist, Notify = album_changed);
    qproperty!("track", Member = track, Notify = track_changed);
    qproperty!("cover", Member = cover, Notify = cover_changed);
    qproperty!("duration", Member = duration, Notify = duration_changed);
    qproperty!("artist_str", Member = artist_str, Notify = duration_changed);
    // qproperty!("album", Member = album);
    // qproperty!("alo")

    #[qsignal]
    fn title_changed(&mut self);
    #[qsignal]
    fn file_changed(&mut self);
    #[qsignal]
    fn album_changed(&mut self);    
    #[qsignal]
    fn artist_changed(&mut self);
    #[qsignal]
    fn track_changed(&mut self);    
    #[qsignal]
    fn cover_changed(&mut self);
        #[qsignal]
    fn duration_changed(&mut self);
    // #[qsignal]
    // fn album_changed(&mut self);

    // Các hàm helper để Rust dễ dàng can thiệp và cập nhật dữ liệu từ Master worker
    pub fn update_data(&mut self, song: SongS) {
                // println!("{:?}",song);

        self.album = song.album;
        self.title = song.title;
        self.file = song.file;
        self.artist = song.artist;
        self.track = song.track;
        self.cover = song.cover;
        self.duration = song.duration;
        
        self.title_changed();
        self.file_changed();
        self.album_changed();
        self.artist_changed();
        self.cover_changed();
        self.duration_changed();
    }

    pub fn set_title(&mut self, song: String){
        self.title = song;
        self.title_changed();
    } 
    fn get_handle_invoke(&self) -> QThreadHandle<SongS>{
        self.get_thread_handle()
    }

}

use qtbridge::{QApp, QThreadHandle, QmlObject, qobject, qsignal, qslot,};
impl From<&Song> for SongS{
    fn from(song: &Song) -> Self {
        // let 
        let index = song.file.rfind('/').unwrap();
        let cover = format!("file://mnt/980/Music/{}cover.jpg",&song.file[..=index]);
        SongS { file: song.file.clone(), title: song.title.clone(), artist: song.artist.clone(), album: song.album.clone() , 
            track:  song.track,
            cover,
            duration: song.time,
            artist_str: song.album_artist.join(", "),
            // if let Some(v) = song.track{v} else{0}
        }
    }
}


use std::cell::RefCell;
use std::rc::Rc;

const INVALID_TRACK: u32 = u32::MAX; 
const INVALID_DISC: u32 = u32::MAX; 
const INVALID_ID: u32 = u32::MAX; 
const INVALID_TIME: u32 = u32::MAX; 
const INVALID_DURATION: f32 = -1.0; 
const INVALID_POS: u32 = u32::MAX; 
const INVALID_YEAR: u16 = u16::MAX;
const INVALID_BITDEAP: u8 = u8::MAX;
const INVALID_SAMPLERATE: u32 = u32::MAX;
pub fn parse_queue(data: Vec<String>) -> Vec<Song>{
    let mut queue = Vec::new();
    let mut song = Song::default();
    for line in data{
        if let Some(p) = line.find(':'){
            let key = &line[..p].to_lowercase();
            let value = line[p+1..].trim();
            match key.as_str() {
                "file" 
                    => song.file = value.to_string(),
                "format"
                    => {
                        let split_elements: Vec<&str> = value.split(':').collect();
                        song.sample_rate = split_elements[0].parse::<u32>().unwrap_or(INVALID_SAMPLERATE);
                        song.bit_deep = split_elements[1].parse::<u8>().unwrap_or(INVALID_BITDEAP);
                    }
                "title"
                    => song.title = value.to_string(),
                "artist"
                    => song.artist.push(value.to_string()),
                "album"
                    => song.album = value.to_string(),
                "date" 
                    => song.date = value.to_string(),
                "genre" 
                    => song.genre.push(value.to_string()),
                "composer"
                    => song.composer.push(value.to_string()),
                "albumartist"
                    => song.album_artist.push(value.to_string()),
                "track"
                    => song.track = value.parse::<u32>().unwrap_or(INVALID_TRACK),
                "disc"
                    => song.disc = value.parse::<u32>().unwrap_or(INVALID_DISC),

                "label"
                    => song.label = value.to_string(),
                "time"
                    => song.time = value.parse::<u32>().unwrap_or(INVALID_TIME),
                "duration"
                    => song.duration = value.parse::<f32>().unwrap_or(INVALID_DURATION),
                
                "pos"
                    => song.pos = value.parse::<u32>().unwrap_or(INVALID_POS),
                "id" 
                    => {
                        song.id = value.parse::<u32>().unwrap_or(INVALID_ID);
                        song.year = if song.date.len() >= 4 {song.date[0..4].parse::<u16>().unwrap_or(INVALID_YEAR)} else {INVALID_YEAR};
                        song.key = format!("{}{}{}",song.album,song.date,song.artist.join(""));
                        let completed_song = std::mem::take(&mut song);
                        queue.push(completed_song);
                    },
                _ => {},
            }
        }
        
    }
    queue
}


// impl Song{
//     pub fn clear(&mut self){
//         self.artist: ,
//     pub album: String,
//     pub date: String,
//     pub genre: Vec<String>,
//     pub composer: Vec<String>,
//     pub album_artist: Vec<String>,
//     }
// }
// #[qtbridge::qobject]
// impl Song{

// }
// #[derive(Default, Debug,  PartialEq)]
// pub struct QueueSong{
//     pub pos: u32,
//     pub id: u32,
//     pub album: String,
//     pub artis_album: Vec<String>,
//     pub year: u16,
//     pub song: Rc<RefCell<SongS>>,
//     pub album_cover: String,
// }
// #[qtbridge::qobject]
// impl QueueSong{
//     pub fn new(
//     pos: u32,
//     id: u32,
//     song: SongS,
//     // album_cover: String,
//     ) -> Self{
//         let album= song.album.clone();
//         // let artis_album = song.album_artist.clone(); 
//         let artis_album = vec![];
//         let year = 1199;

//         QueueSong { pos, id, album, artis_album, year, 
//             song: Rc::new(RefCell::new(song)), 
//             album_cover: "file://mnt/980/Music/alan/Swear/cover.jpg".to_string(),
//         }
//     }
//     qproperty!("pos", Member = pos,  Notify = pos_changed);
//     qproperty!("id", Member = id, Notify = id_changed);
//     qproperty!("album", Member = album, Notify = album_changed);
//     qproperty!("albumArtist", Member = artis_album, Notify = album_artis_changed);
//     qproperty!("year", Member = year, Notify = year_changed);
//     qproperty!("albumCover", Member = album_cover, Notify = album_cover_changed);
//     qproperty!("song", Member = song, Notify = song_changed);
//     // qproperty!("album", Member = album);
//     // qproperty!("alo")

//     #[qsignal]
//     fn pos_changed(&mut self);
//     #[qsignal]
//     fn id_changed(&mut self);
//     #[qsignal]
//     fn album_changed(&mut self);    
//     #[qsignal]
//     fn album_artis_changed(&mut self);
//     #[qsignal]
//     fn year_changed(&mut self);  
//     #[qsignal]
//     fn album_cover_changed(&mut self);        
//     #[qsignal]
//     fn song_changed(&mut self);    
//     pub fn update_data(&mut self, song: QueueSong) {

//         self.album = song.album;

//     }
// }

// pub fn parse_queue(mut lines: Vec<String>) -> Vec<QueueSong>{
//     let queue_len = lines.len()/18;
//     let mut queue = Vec::with_capacity(queue_len);
//     for _ in 0..queue_len{
//         let mut raw_song: Vec<String> = lines.drain(..18).collect();
//         let id_line : String = raw_song.pop().unwrap();
//         let pos_line = raw_song.pop().unwrap();
//         let song = parse_song(raw_song).unwrap();
//         let p = id_line.find(':').unwrap();
//         let value = id_line[p+1..].trim();
//         let id = value.parse::<u32>().unwrap();
//         let p = pos_line.find(':').unwrap();
//         let value = pos_line[p+1..].trim();
//         let pos = value.parse::<u32>().unwrap();
//         queue.push(QueueSong::new(pos, id, (&song).into()));
//     }
//     queue
// }