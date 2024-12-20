use std::net::SocketAddr;
use std::path::{PathBuf};
use std::sync::Arc;
use tracing::info;
use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use log::warn;
use tower_http::services::ServeDir;

#[derive(Debug)]
struct HttpServeState{
    path: PathBuf,
}
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()>{

    let state = HttpServeState{path: path.clone()};

    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    let router = Router::new()
        // .route("/*path",get(file_handler))
        .nest_service("/tower",dir_service)
            .with_state(Arc::new(state));

    let addr = SocketAddr::from(([0,0,0,0],port));
    info!("Serving {:?} on port [{}]",addr,port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener,router).await?;

    Ok(())
}

async fn file_handler(State(state):State<Arc<HttpServeState>>, Path(path):Path<String>,) -> (StatusCode,String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}",p);
    if !p.exists(){
        return (StatusCode::NOT_FOUND,format!("File {} not found",p.display()))
    }

    match tokio::fs::read_to_string(p).await{
        Ok(content) => {
            info!("Read {} bytes",content.len());
            (StatusCode::OK,content)
        }
        Err(e) => {
            warn!("Error reading file: {:?}",e);
            (StatusCode::INTERNAL_SERVER_ERROR,e.to_string())
        }
    }
}