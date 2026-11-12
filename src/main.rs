mod mpd_worker;
mod protocol;
mod master;
mod backend;
use backend::Backend;
use qtbridge::QApp;
use qtbridge::include_bytes_qml;





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    include_bytes_qml!("../GUI", "qt/qml/mpd_client");
    QApp::new()
        .register::<Backend>()
        .load_qml_from_file("GUI/Main.qml")
        .run();
    Ok(())
}


