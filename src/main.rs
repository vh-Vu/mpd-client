use tokio::net::tcp::WriteHalf;
// use tokio::net::unix::pipe::Receiver;
use std::os::unix::thread;
use std::sync::Arc;
use std::sync::mpsc::RecvTimeoutError::Disconnected;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio::time::{self,Duration};
use tokio::sync::mpsc::{Receiver,Sender};
mod mpd_worker;
use mpd_worker::{MpdWorker};
mod protocol;
use protocol::{Command,Song,DataEvent,MpdWorkerEvent,ChangedEvent,State};
use std::cell::RefCell;
use std::rc::Rc;
use qtbridge::QParserStatus;
mod master;
use master::Master;
mod backend;
use backend::Backend;

use tokio::runtime::Builder;





use qtbridge::{QApp, QThreadHandle, QmlObject, qobject, qsignal, qslot,QModelItem};



use crate::protocol::{Status};
use qtbridge::include_bytes_qml;

// use qtbridge_runtime::qapp;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    include_bytes_qml!("../GUI", "qt/qml/mpd_client");
    QApp::new()
        .register::<Backend>()
        .load_qml_from_file("GUI/Main.qml")
        .run();
    Ok(())
}


