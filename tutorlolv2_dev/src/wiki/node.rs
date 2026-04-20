use crate::MayFail;
use serde::{Deserialize, Serialize};
use std::{
    process::Stdio,
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{ChildStdin, ChildStdout, Command},
    sync::{mpsc, oneshot},
};

pub struct NodePool {
    workers: Vec<NodeWorkerHandle>,
    next_worker: AtomicUsize,
    next_id: AtomicU64,
}

impl NodePool {
    pub async fn new(size: usize) -> MayFail<Self> {
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let mut child = Command::new("node")
                .arg("node_worker.mjs")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()?;

            let stdin = child.stdin.take().ok_or("missing stdin")?;
            let stdout = child.stdout.take().ok_or("missing stdout")?;

            let (tx, rx) = mpsc::channel::<WorkerRequest>(64);
            tokio::spawn(worker_loop(stdin, stdout, rx));

            workers.push(NodeWorkerHandle { tx });
        }

        Ok(Self {
            workers,
            next_worker: AtomicUsize::new(0),
            next_id: AtomicU64::new(1),
        })
    }

    pub async fn fetch(&self, url: String, path: String) -> SyncMayFail<NodeJobResult> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let index = self.next_worker.fetch_add(1, Ordering::Relaxed) % self.workers.len();

        let (tx, rx) = oneshot::channel();

        self.workers[index]
            .tx
            .send(WorkerRequest {
                job: NodeJob { id, url, path },
                tx,
            })
            .await?;

        rx.await?
    }
}

#[derive(Debug, Serialize)]
struct NodeJob {
    id: u64,
    url: String,
    path: String,
}

#[derive(Debug, Deserialize)]
pub struct NodeJobResult {
    pub id: Option<u64>,
    pub ok: bool,
    pub cached: Option<bool>,
    pub status: Option<u16>,
    pub final_url: Option<String>,
    pub error: Option<String>,
}

pub type SyncMayFail<T, E = Box<dyn core::error::Error + Send + Sync>> = MayFail<T, E>;

struct WorkerRequest {
    job: NodeJob,
    tx: oneshot::Sender<SyncMayFail<NodeJobResult>>,
}

struct NodeWorkerHandle {
    tx: mpsc::Sender<WorkerRequest>,
}

async fn worker_loop(
    mut stdin: ChildStdin,
    stdout: ChildStdout,
    mut rx: mpsc::Receiver<WorkerRequest>,
) {
    let mut reader = BufReader::new(stdout).lines();

    while let Some(req) = rx.recv().await {
        let line = match serde_json::to_string(&req.job) {
            Ok(s) => s,
            Err(e) => {
                let _ = req.tx.send(Err(e.into()));
                continue;
            }
        };

        if let Err(e) = stdin.write_all(line.as_bytes()).await {
            let _ = req.tx.send(Err(e.into()));
            continue;
        }

        if let Err(e) = stdin.write_all(b"\n").await {
            let _ = req.tx.send(Err(e.into()));
            continue;
        }

        if let Err(e) = stdin.flush().await {
            let _ = req.tx.send(Err(e.into()));
            continue;
        }

        match reader.next_line().await {
            Ok(Some(resp_line)) => {
                let parsed: SyncMayFail<NodeJobResult> =
                    serde_json::from_str(&resp_line).map_err(Into::into);
                let _ = req.tx.send(parsed);
            }
            Ok(_) => {
                let _ = req.tx.send(Err("[error] node worker stdout closed".into()));
                break;
            }
            Err(e) => {
                let _ = req.tx.send(Err(e.into()));
            }
        }
    }
}
