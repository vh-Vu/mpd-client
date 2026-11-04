use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver,Sender};
use tokio::sync::mpsc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{self,Duration};

use crate::protocol::{ChangedEvent, Command, DataEvent, MpdWorkerEvent, Song, parse_status};






pub struct MpdWorker {}

impl MpdWorker{
    pub async fn run(address: &str, sender: Sender<MpdWorkerEvent>,dtx: Sender<DataEvent>){
        let command = TcpStream::connect(address).await;
        let idle = TcpStream::connect(address).await;
        if command.is_err() || idle.is_err(){
            let _ = sender.send(MpdWorkerEvent::Disconnected).await;
            return ;
        }
        let tx = sender.clone();
        let watcher = tokio::spawn(async move{ watcher(idle.unwrap(), tx).await});
        let (send, rx) = mpsc::channel::<Command>(32);
        let worker = tokio::spawn(async move{ worker(command.unwrap(), rx,dtx).await});


        let _ = sender.send(MpdWorkerEvent::Connected(send)).await;
                                        println!("send?");

        tokio::select! {
            _res = worker => {},
            _res = watcher => {},
        }
        let _ = sender.send(MpdWorkerEvent::Disconnected).await;
    }
}



async fn watcher(stream: TcpStream,sender_to_master: Sender<MpdWorkerEvent>){

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
                            "player" => MpdWorkerEvent::Changed(ChangedEvent::Player),
                            _ => MpdWorkerEvent::Changed(ChangedEvent::Unknown),
                        };
                        let _  = sender_to_master.send(mess).await;
                    }
                    else{
                        break 'idle;
                    }
                }
            }
        }
    }
}

async fn worker(stream: TcpStream, mut receiver_command: Receiver<Command>,dtx: Sender<DataEvent>){
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
                if writer.write_all(cmd.as_bytes()).await.is_err() || writer.flush().await.is_err() {
                    break 'cmd_loop;
                }
                // cmd;
                // let mut line = String::new();
                let mut l = Vec::new(); 
                loop {
                    line.clear();
                    match buf_reader.read_line(&mut line).await {
                        Ok(0) | Err(_) => break 'cmd_loop,
                        Ok(_) => {
                            let trimmed: String = line.trim().to_string();
                            // println!("{}",trimmed);
                            //TODO
                            if trimmed == "OK" || trimmed.starts_with("ACK") {
                                break;
                            }
                            l.push(trimmed);
                        }
                    }
                }
                match cmd {
                    Command::CurrentSong => {
                        let mut song = Song::default();
                        for li in l{
                            let p = li.find(':').unwrap();
                            let att = &li[0..p];
                            match att{
                                "file" => song.file = li[p+1..].trim().to_string(),
                                "Title" => song.title = li[p+1..].trim().to_string(),
                                _ => {}
                            }
                            // if att == "file"{
                            //     song.file = att
                            // }
                        }
                        // se
                        let _ = dtx.send(DataEvent::CurrentSong(song)).await;

                    },
                    Command::Status => {
                        match parse_status(l){
                            Ok(status) => {let _ = dtx.send(DataEvent::Status(status)).await;},
                            Err(_) => {},
                        }
                        
                        
                    }
                    _ => {

                    }
                }
            },
            _ = ping_interval.tick() => {
                if writer.write_all(b"ping\n").await.is_err() || writer.flush().await.is_err() {
                    break 'cmd_loop;
                }
                // let mut line = String::new();
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


