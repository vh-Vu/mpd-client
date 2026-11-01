use socket2::TcpKeepalive;
use tokio::net::tcp::WriteHalf;
// use tokio::net::unix::pipe::Receiver;
use std::env;
use std::os::unix::thread;
use std::sync::Arc;
use std::sync::mpsc::RecvTimeoutError::Disconnected;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio::time::{self,Duration};
use tokio::sync::mpsc::{Receiver,Sender};

/// Sự kiện gửi từ Task Idle về Main UI
#[derive(Debug)]
pub enum MpdEvent {
    ConnectionState(State),
    Command(String),
}


#[derive(Debug)]
enum State{
    Connected,
    Disconnected,
}

#[derive(Debug)]

enum Event{
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
}

impl State{
    pub fn into(self) -> MpdEvent{
        MpdEvent::ConnectionState(self)
    }
}

#[derive(Debug)]
pub struct MpdCommand {
    pub command: String,
    pub responder: mpsc::Sender<Result<Vec<String>, String>>,
}




const DEFAULT_PORT: u16 = 6600;
const LOCAL_HOST: &str = "127.0.0.1";

use qtbridge::{QApp, qobject};
// #[derive(Default)]
pub struct Backend{
    is_play: bool
}
use std::sync::OnceLock;
static MPD_SENDER: OnceLock<mpsc::Sender<MpdEvent>> = OnceLock::new();
// #[qobject(Singleton)]
#[qobject]
impl Backend{
    #[qslot]
    fn say_hello(&self){
        println!("hello");
    }
    #[qslot]
    fn toggle_play(&mut self){
        if let Some(tx) = MPD_SENDER.get() {
            let txx = tx.clone();
            let is_play = self.is_play;
                                // let _ = txx.send(MpdEvent::Command("play\n".to_string()));

            tokio::spawn(async move{
                if is_play {
                let _ = txx.send(MpdEvent::Command(format!("pause\n"))).await;
                }else{
                    println!("here {}",is_play);
                                                    let _ = txx.send(MpdEvent::Command("play\n".to_string())).await;

                }
            });
            
            self.is_play = !self.is_play
        }
    }

}

impl  Default for Backend {
    fn default() -> Self {
        Self { is_play: false }
    }
}

use std::io::{self, Write};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// fn main() {
    let (tx,rx) = mpsc::channel::<MpdEvent>(32);
    // let txx = tx.clone();
    MPD_SENDER.set(tx.clone()).map_err(|_| "Failed to set sender")?;
    let mut lifetime_manager = LifetimeCycle::new(tx,rx);

    tokio::spawn(async move{
        lifetime_manager.run().await;
    });
    QApp::new()
        .register::<Backend>()
        .load_qml(include_bytes!("../GUI/Main.qml"))
        .run();
    // 
    // let txx = tx.clone();
    // tokio::spawn( async move{
    //     test(txx).await
    // });
    // let mut lifetime_manager = LifetimeCycle::new(tx,rx);
    // lifetime_manager.run().await;

    Ok(())
}
    

async fn test(sender: Sender<MpdEvent>) {
    loop{
        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let _ = sender.send(MpdEvent::Command(input)).await;
    }
   
}

struct LifetimeCycle{
    mpd_address: String,
    sender: Sender<MpdEvent>,
    receiver: Receiver<MpdEvent>,
}

impl LifetimeCycle {
    pub fn new(sender: Sender<MpdEvent>, receiver: Receiver<MpdEvent>) -> Self{
        let host = env::var("MPD_HOST").unwrap_or_else(|_| LOCAL_HOST.to_string());
        let port = env::var("MPD_PORT")
            .ok()
            .and_then(|val| val.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT);

        let mpd_address = format!("{}:{}", host, port);
        Self{
            mpd_address,
            sender,
            receiver
        }
    }
    pub async fn run(&mut self) {
        loop{
            let sender = self.sender.clone();
            let address = self.mpd_address.clone();
            let (tx_command,mut rx_command) = mpsc::channel::<String>(32);
            tokio::spawn(async move{
                worker(address,sender,rx_command).await
            });
            let sender = self.sender.clone();
            let address = self.mpd_address.clone();
            tokio::spawn(async move{
                watcher(address,sender).await
            });
            while let Some(mes) = self.receiver.recv().await{
                match mes{
                    MpdEvent::ConnectionState(state) => {
                        match state {
                            State::Disconnected => {
                                //TODO
                                break;
                            },
                            State::Connected => {//sending message to upper
                                //TODO
                                println!("ready");
                            },
                        }
                    }
                    MpdEvent::Command(cmd) => {
                        println!("ha");
                        let _ = tx_command.send(cmd).await;
                    }
                }
            }
            sleep(Duration::from_secs(5)).await;
            println!("reconecting");
        }
    } 
}


async fn worker(addr: String, tx_events: mpsc::Sender<MpdEvent>, mut rx_command: mpsc::Receiver<String>) {

    match TcpStream::connect(addr).await {
        Ok(stream) => {
            let _ = tx_events.send(State::Connected.into()).await;

            let (reader, mut writer) = tokio::io::split(stream);
            let mut buf_reader = BufReader::new(reader);
            let mut greeting = String::new();
            if buf_reader.read_line(&mut greeting).await.is_err() {
                let _  = tx_events.send(State::Disconnected.into()).await;
                return;
            }
            let mut ping_interval = time::interval(Duration::from_secs(10));
            ping_interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
            'cmd_loop: loop{
                tokio::select! {
                    Some(cmd) = rx_command.recv() => {
                        if writer.write_all(cmd.as_bytes()).await.is_err() || writer.flush().await.is_err() {
                            break 'cmd_loop;
                        }
                        let mut line = String::new();
                        loop {
                            line.clear();
                            match buf_reader.read_line(&mut line).await {
                                Ok(0) | Err(_) => break 'cmd_loop,
                                Ok(_) => {
                                    let trimmed = line.trim();
                                    println!("{}",trimmed);
                                    //TODO
                                    if trimmed == "OK" || trimmed.starts_with("ACK") {
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    _ = ping_interval.tick() => {
                        if writer.write_all(b"ping\n").await.is_err() || writer.flush().await.is_err() {
                            break 'cmd_loop;
                        }
                        let mut line = String::new();
                        loop {
                            line.clear();
                            match buf_reader.read_line(&mut line).await {
                                Ok(0) | Err(_) => break 'cmd_loop,
                                Ok(_) => {
                                    let trimmed = line.trim();
                                    if trimmed == "OK" || trimmed.starts_with("ACK") {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }

        },
        Err(_) => {},
    }
    println!("woker sap");
    let _  = tx_events.send(State::Disconnected.into()).await;

}



async fn watcher(addr: String, tx_events: mpsc::Sender<MpdEvent>) {
    match TcpStream::connect(addr).await {
        Ok(stream) => {
            let _ = tx_events.send(State::Connected.into()).await;

            let (reader, mut writer) = tokio::io::split(stream);
            let mut buf_reader = BufReader::new(reader);
            let mut greeting = String::new();
            if buf_reader.read_line(&mut greeting).await.is_err() {
                let _  = tx_events.send(State::Disconnected.into()).await;
                return;
            }
            let mut line = String::new();
            'idle: loop{
                    if writer.write_all(b"idle\n").await.is_err() || writer.flush().await.is_err() {
                        break 'idle;
                    }
                    loop{
                        line.clear();
                        match buf_reader.read_line(&mut line).await {
                            Ok(0) => break,
                            Err(_) => break 'idle,
                            Ok(_) => {
                                let response = line.trim();
                                if response == "OK" || response == "ACK"{
                                    break;
                                }else if let Some(sub) = response.strip_prefix("changed: "){
                                    // TODO
                                     println!("{}",sub);

                                }
                                else{
                                    break 'idle;
                                }
                            }
                        }
                    }
                    
                  
            }

        },
        Err(_) => {},
    }
    println!("wacher sap");
    let _  = tx_events.send(State::Disconnected.into()).await;

}


