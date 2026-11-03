pub struct Status{
    volume: i8,
    repeat: bool,
    random: bool,
    single: Mode,
    comsume: Mode,
    partition: String,
    playlist: u32,
    playlistlength: i32,
    mixrampdb: i32,
    state: State,
    lastloadedplaylist: String,
    song: u32,
    songid: u32,
    elapsed: f32,
    bitrate: u32,
    duration: f32,
    audio: String,
    nextsong: u32,
    nextsongid: u32,
}


pub enum State{
    Play,
    Pause,
    Stop,
}

pub enum Mode{
    On,
    Off,
    OneShot,
}




pub struct Song{
    pub file: String,
    pub title: String,
    pub artist: Vec<String>,
    pub album: String,
    pub date: String,
    pub genre: Vec<String>,
    pub composer: Vec<String>,
    pub album_artist: Vec<String>,
    pub track: Option<u32>,
    pub disc: Option<u32>,
    pub label: String,
    pub time: u32,
    pub duration: f64,
    pub pos: u32,
    pub id: u32, 
}


impl Default for Song{
    fn default() -> Self {
        Self {  
            file: String::new(), 
            title: String::new(), 
            artist: Vec::new(), 
            album: String::new(), 
            date: String::new(), 
            genre: Vec::new(), 
            composer: Vec::new(), 
            album_artist: Vec::new(), 
            track: None, 
            disc: None, 
            label: String::new(), 
            time: 0, 
            duration: 0.0, 
            pos: 0, 
            id: 0
        }
    }
}