use qtbridge::{QApp, QThreadHandle, QmlObject, qobject, qsignal, qslot,QModelItem};
use crate::protocol::{Command,Song,DataEvent,MpdWorkerEvent,ChangedEvent,State};
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc::Sender;
use tokio::runtime::Builder;
use crate::master::Master;
pub struct Backend{
    is_play: bool,
    current_song: Rc<RefCell<Song>>,  
    runtime: tokio::runtime::Runtime,
    sender_to_worker: Option<Sender<Command>>,
    queue: Vec<Rc<RefCell<Song>>>,
    current_id: u32,
    current_position: f32,

}


// #[qtbridge::qobject(Base = QParserStatus)]
#[qobject]
impl Backend{
    qproperty!("is_play", Member = is_play, Write = set_is_play, Notify = is_play_changed);
    // qproperty!("currentSong", Member = current_song, Write = set_current_song, Notify = current_song_changed);
    // qproperty!("currentTitle", Read = get_current_title, Notify = current_song_changed);
    // qproperty!("currentAlbum", Read = get_current_album, Notify = current_song_changed);
    // qproperty!("currentSong", Member = current_song);
    qproperty!("currentSong", Member = current_song, Notify = current_song_changed);
    qproperty!("currentId", Member = current_id, Notify = current_song_changed);
    qproperty!("currentPosition", Member = current_position, Write = set_current_song, Notify = current_position_changed);
    qproperty!("queue", Member = queue, Notify = queue_changed);
    #[qsignal]
    fn queue_changed(&mut self);
    #[qsignal]
    fn current_position_changed(&mut self);
    // qproperty!("currentTitle", Read = get_title, Notify = song_changed);
    // qproperty!("currentAlbum", Read = get_album, Notify = song_changed);
    // qproperty!("currentSong", Member = current_song, Notify = song_changed);
// qproperty!("currentSong", Member = current_song,Read = get_current_song, Notify = current_song_changed);

// fn get_current_song(&self) -> Rc<RefCell<SongS>> {
//         Rc::clone(&self.current_song)
//     }
//     #[qsignal]
//     fn current_song_changed(&mut self);

    #[qsignal]
    fn current_song_changed(&mut self);
    // #[qsignal]
    // fn id_in_queue_changed(&mut self);

    #[qslot]
    fn start(&self){
        let handle = self.get_thread_handle();
        // let handle_current_song = self.current_song.borrow().get_thread_handle();
        self.runtime.spawn(async move{
            let mut master = Master::new(handle);
            master.run().await;
        });
        
    }

    #[qslot]
    fn seek_current_time(&self, time: f32){
        if let Some(sender) = &self.sender_to_worker{
            let tx = sender.clone();
            tokio::spawn(async move{
                let _ = tx.send(Command::Seek(time)).await;
            });
        }
        
    }

    #[qslot]
    fn new_xx(&self, mut pos: u32,song_seleted: Vec<u32> ){
        let len = self.queue.len() as u32;
        if pos == len && pos > 0{
            pos-=1;
        }
        if let Some(tx) = &self.sender_to_worker {
            let sender = tx.clone();
            tokio::spawn(async move{
                for id in song_seleted{
                    let _ = sender.send(Command::MoveSong { song_id: id, pos }).await;
                }
            });
        }
        // println!("in tai {} {:?}",pos,song_seleted);
    }

    #[qslot]
    fn play_song_at_id(&self, id: u32){
        if let Some(tx) = &self.sender_to_worker {
            let sender = tx.clone();
            tokio::spawn(async move{
                let _ = sender.send(Command::PlayID(id)).await;
            });
        }
    }
    pub fn set_current_song_by_pos(&mut self, pos: u32){
        self.current_song = self.queue[pos as usize].clone();
        self.current_id = pos;

        self.current_song_changed();
    }

    pub fn set_queue(& mut self, queue: Vec<Song>){
        self.queue = queue.into_iter().map(|song| Rc::new(RefCell::new(song))).collect();
        self.queue_changed();
    }

    fn get_title(&self) -> String {
        let test = self.current_song.borrow_mut().title.clone();
        println!("from function {}",test);
        test
    }

    fn get_album(&self) -> String {
        self.current_song.borrow().album.clone()
    }
    #[qsignal]
    fn song_changed(&mut self);
    
   
    pub fn assign(&mut self, sender: Sender<Command>){
        self.sender_to_worker = Some(sender);
        self.ready();
    }

    #[qslot]
    fn ready(&self){
        if let Some(tx) = &self.sender_to_worker {
            let txx = tx.clone();
            tokio::spawn(async move{
                let _ = txx.send(Command::Status).await;
            });
        }
        
    }

    #[qslot]
    fn toggle_play(&mut self){
        if let Some(tx) = &self.sender_to_worker{
            let cmd = if  self.is_play{Command::Pause} else{Command::Play};
            let txx = tx.clone();
            tokio::spawn(async move{let _ = txx.send(cmd).await;});
            // self.set_pause(!self.is_play);
        }
    }

    #[qslot]
    fn toggle_next(&mut self){
        if let Some(tx) =  &self.sender_to_worker {
            let txx = tx.clone();
            tokio::spawn(async move{
                let _ = txx.send(Command::Next).await;
            });
            
        }
    }

    #[qslot]
    fn toggle_prev(&mut self){
        if let Some(tx) =  &self.sender_to_worker{
            let txx = tx.clone();
            tokio::spawn(async move{
                let _ = txx.send(Command::Previous).await;
            });
            
        }
    }

    // pub fn set_pause(&mut self,is_play: bool, current_position: f32){
    //     self.is_play = is_play;
    //     self.current_position = current_position;
    //     self.is_play_changed();
    // }

    fn set_current_song(&mut self, current_position: f32){
        self.current_position = current_position;
        self.current_position_changed();
    }
    fn set_is_play(&mut self,is_play: bool){
        self.is_play = is_play;
        self.is_play_changed();
    }
    pub fn update_status(&mut self, is_play: bool, current_position: f32) {
        self.set_current_song(current_position);
        self.set_is_play(is_play);
    }

    #[qsignal]
    fn is_play_changed(&mut self);
}

impl Backend {
    pub fn set_sender(&mut self, sender: Sender<Command>){
        self.sender_to_worker = Some(sender);
    }
}
// impl QParserStatus for Backend {
//     fn component_complete(&mut self) {
//         // Inject the shared networking handle into every child.
//         // for resource in &self.resources {
//         //     resource.borrow_mut().set_service(Rc::clone(&self.service));
//         // }
//         // self.login.borrow_mut().set_service(Rc::clone(&self.service));
//     }
// }
impl  Default for Backend {
    fn default() -> Self {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();
        Self { 
            is_play: false,
            current_song: Rc::new(RefCell::new(Song::default())),
            runtime,
            sender_to_worker: None,
            queue: Default::default(),
            current_id: 0,
            current_position: 0.0,
        }
    }
}