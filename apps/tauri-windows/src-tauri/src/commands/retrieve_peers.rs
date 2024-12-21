

type PeerMap = Arc<TokioMutex<HashMap<String, PeerInfo>>>;

#[tauri::command]
pub fn generate_qr_code(data: Option<String>, state: State<PeerMap>) -> Result<String, String> {


}
