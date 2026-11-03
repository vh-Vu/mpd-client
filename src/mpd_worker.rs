use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver,Sender};
use tokio::sync::mpsc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{self,Duration};


#[derive(Debug)]
pub enum MpdWorkerEvent{
    Connected(Sender<Command>),
    Disconnected,
}

enum InternalProtocol {
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

pub enum Command{
    Play,
    Next,
    Pause,
    Previous,
}
pub struct MpdWorker {}

impl MpdWorker{
    pub async fn run(address: &str, sender: Sender<MpdWorkerEvent>){
        let command = TcpStream::connect(address).await;
        let idle = TcpStream::connect(address).await;
        if command.is_err() || idle.is_err(){
            let _ = sender.send(MpdWorkerEvent::Disconnected).await;
            return ;
        }
        let (tx,rx) = mpsc::channel::<InternalProtocol>(32);
        let watcher = tokio::spawn(async move{ watcher(idle.unwrap(),tx).await});
        let (send, rece) = mpsc::channel::<Command>(32);
        let worker = tokio::spawn(async move{ woker(command.unwrap(),rx,rece).await});


        let _ = sender.send(MpdWorkerEvent::Connected(send)).await;
        let _ = tokio::join!(watcher,worker);
        let _ = sender.send(MpdWorkerEvent::Disconnected).await;
    }
}



async fn watcher(stream: TcpStream,sender_internal: Sender<InternalProtocol>){

    let (reader, mut writer) = tokio::io::split(stream);
    let mut buf_reader = BufReader::new(reader);
    let mut greeting = String::new();

    if buf_reader.read_line(&mut greeting).await.is_err() { return }

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
                    }else if let Some(rep) = response.strip_prefix("changed: "){
                        // TODO
                        let mess = match rep {
                            "player" => InternalProtocol::Player,
                            _ => InternalProtocol::Unknown,
                        };
                        let _  = sender_internal.send(mess).await;
                    }
                    else{
                        break 'idle;
                    }
                }
            }
        }
    }
}

async fn woker(stream: TcpStream, mut receiver_internal: Receiver<InternalProtocol>, mut receiver_command: Receiver<Command>){
    let (reader, mut writer) = tokio::io::split(stream);
    let mut buf_reader = BufReader::new(reader);
    let mut greeting = String::new();

    if buf_reader.read_line(&mut greeting).await.is_err() { return }

    let mut line = String::new();

    let mut ping_interval = time::interval(Duration::from_secs(10));
    ping_interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
    
    'cmd_loop: loop{

        tokio::select! {
            // TODO with resonpse UI command
            Some(cmd) = receiver_command.recv() =>{
                let cmd: &str = cmd.into();
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
            Some(mess) = receiver_internal.recv() => {
                let req = match mess{
                    InternalProtocol::Player => "currentsong\n",
                    _ => "",
                };
                if writer.write_all(req.as_bytes()).await.is_err() || writer.flush().await.is_err() {
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

}


impl From<Command> for &str{
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Play => "play\n",
            Command::Pause => "pause\n",
            Command::Next => "next\n",
            Command::Previous => "previous\n",
        }
    }
}