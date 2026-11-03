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

pub fn parse_status(lines: Vec<String>) -> Status{
    let mut status = Status::default();
    for line in lines{
        let p = line.find(':').unwrap();
        let key = &line[..p];
        let value = line[p+1..].trim();
        match key {
            "volume" 
                => status.volume = value.parse::<i8>().unwrap(),
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
                => status.playlist = value.parse::<u32>().unwrap(),
            "playlistlength" 
                => status.playlistlength = value.parse::<i32>().unwrap(),
            "mixrampdb" 
                => status.mixrampdb = value.parse::<i32>().unwrap(),
            "state" 
                => status.state = match value {
                                        "play" => State::Play,
                                        "pause" => State::Pause,
                                        _   => State::Stop,
                                    },
            "lastloadedplaylist" 
                => status.lastloadedplaylist = value.to_string(),
            "song" 
                => status.song = value.parse::<u32>().unwrap(),            
            "songid" 
                => status.songid = value.parse::<u32>().unwrap(),
            "elapsed" 
                => status.elapsed = value.parse::<f32>().unwrap(),
            "bitrate" 
                => status.bitrate = value.parse::<u32>().unwrap(),
            "duration" 
                => status.duration = value.parse::<f32>().unwrap(),
            "audio" 
                => status.audio = value.to_string(),
            "nextsong" 
                => status.nextsong = value.parse::<u32>().unwrap(),
            "nextsongid" 
                => status.nextsongid = value.parse::<u32>().unwrap(),
            _ => {},
        }
    }
    status
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
        let status = parse_status(v);
        assert!(matches!(status.state, State::Pause));
        assert_eq!(status.songid,17);
        assert_eq!(status.elapsed, 259.598);
        assert_eq!(status.volume, 91);
        assert!(matches!(status.single, Mode::Off));

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


#[derive(Default,Debug)]
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



#[derive(Debug,Clone)]
pub enum Command{
    Play,
    Next,
    Pause,
    Previous,
    CurrentSong,
    Status,
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
        }
    }
}


pub enum DataEvent {
    Status(Status),
    CurrentSong(Song),
}

#[derive(Debug)]
pub enum MpdWorkerEvent{
    Connected(Sender<Command>),
    Disconnected,
}