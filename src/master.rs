use crate::protocol::{Command,Song,DataEvent,MpdWorkerEvent,ChangedEvent,State,Status};
use crate::Backend;
use tokio::sync::mpsc::{Receiver,Sender};
use std::{default, env};
use qtbridge::QThreadHandle;
use tokio::sync::mpsc;
use crate::mpd_worker::{MpdWorker};

const DEFAULT_PORT: u16 = 6600;
const LOCAL_HOST: &str = "127.0.0.1";
pub struct Master{
    mpd_address: String,
    sender_worker: Option<Sender<Command>>,
    handle: QThreadHandle<Backend>,
    status: Status,
}

impl Master{
    pub fn new(handle: QThreadHandle<Backend>,
    ) -> Self{
        let host = env::var("MPD_HOST").unwrap_or_else(|_| LOCAL_HOST.to_string());
        let port = env::var("MPD_PORT").ok()
                .and_then(|val| val.parse::<u16>().ok())
                .unwrap_or(DEFAULT_PORT);
        let mpd_address = format!("{}:{}", host, port);
        Self {  
            mpd_address,
            sender_worker: None,
            handle,
            status: Status::default()
        }
    }

    pub async fn run(&mut self){
        loop{
            let (tx,mut rx) = mpsc::channel::<MpdWorkerEvent>(32);
            let (dtx, mut drx) = mpsc::channel::<DataEvent>(32);
            let address = self.mpd_address.clone();
            tokio::spawn(async move{
                MpdWorker::run(&address, tx,dtx).await;
            });
            'select : loop {
                tokio::select! {
                    Some(mess) = rx.recv() => {
                        match mess{
                            MpdWorkerEvent::Connected(sender_worker) => {

                                let _  = sender_worker.send(Command::GetQueue).await;
                                let _  = sender_worker.send(Command::Status).await;
                                self.sender_worker = Some(sender_worker.clone());

                                // let _  = sender_worker.send(Command::CurrentSong).await;
                                self.handle.invoke_mut(move |backend| backend.assign(sender_worker));
                                // handle.clone();
                            },
                            MpdWorkerEvent::Disconnected => {   
                                break 'select;
                            },
                            MpdWorkerEvent::Changed(event) => {
                                match event {
                                    ChangedEvent::Player => self.call_status(),
                                    ChangedEvent::Playlist => {
                                        if let Some(sender) = &self.sender_worker{
                                            let _ = sender.send(Command::GetQueue).await;
                                            let _  = sender.send(Command::Status).await;
                                            // let tx = sender.clone();
                                            // tokio::spawn(async move{
                                            //     let _ = tx.send(Command::CurrentSong).await;
                                            // });
                                        }
                                    },
                                    _ => {},
                                }
                            }
                        }
                    },
                    Some(data) = drx.recv() => {
                        match data {
                            DataEvent::Status(status) => {
                                self.similar(status);
                            }
                            DataEvent::CurrentSong(song) => {
                                // let file_path = song.file.clone();
                                // self.song(song);
                            },
                            DataEvent::Queue(queue) => {
                                self.queue(queue);
                                // self.
                            }
                            // _ => {},
                        }
                    }
                    // asdsd,
                };
            }
            
            
            
        }; 
    }
    fn similar(&mut self, status: Status){
        println!("{} {}",status.songid ,self.status.songid);
        if status.songid != self.status.songid{
            // self.call_current_song();
              self.handle.invoke_mut(move |backend| backend.set_current_song_by_pos(status.song));
        }
        
        self.status = status;
        self.state();
    }

    fn call_current_song(&mut self){
        if let Some(sender) = &self.sender_worker{
            let tx = sender.clone();
            tokio::spawn(async move{
                let _ = tx.send(Command::CurrentSong).await;
            });
        }
    }
    fn call_status(&mut self){
        if let Some(sender) = &self.sender_worker{
            let tx = sender.clone();
            tokio::spawn(async move{
                let _ = tx.send(Command::Status).await;
            });
        }
    }
    fn queue(&mut self,queue: Vec<Song>){
        self.handle.invoke_mut(move|backend| backend.set_queue(queue));
    }

    // fn song(&mut self, song: Song){
    //     let songtitle: SongS = (&song).into();
    //     // println!("{:?}",songtitle.file);
    //     // println!("{:?}",songtitle.album);
    //     // println!("{:?}",songtitle.title);
    //     self.handle_song.invoke_mut(move|current_song| current_song.update_data(songtitle) );
    //             // self.handle.invoke_mut(move|backend| backend.testx(songtitle));

    //     // self.handle_song.invoke_mut(move|current_song| current_song.set_title(song.title) );
    //     // let song_name = song.title.clone();
    //     // self.handle.invoke_mut(move|backend| backend.set_current_song(Song::default()));
    // }

    fn state(&mut self){
        let is_play = match self.status.state {
            State::Play => true,
            _ => false, 
        };
        self.handle.invoke_mut(move|backend| backend.set_pause(is_play));

    }
   
}
