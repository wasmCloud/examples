use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetRequest};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct LeaderboardActor {}

// Enhancement idea: make the max scores a parameter on the create leaderboard operation
const MAX_SCORES: u8 = 10;

#[async_trait]
impl HttpServer for LeaderboardActor {
    /// Process the HTTP API routes
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();

        match (req.method.as_ref(), segments.as_slice()) {
            ("GET", ["leaderboards"]) => get_leaderboards(ctx).await,
            ("POST", ["leaderboards"]) => create_leaderboard(ctx, deser_json(&req.body)?).await,
            ("GET", ["leaderboards", leaderboard_id]) => get_leaderboard(ctx, leaderboard_id).await,
            ("POST", ["leaderboards", leaderboard_id, "scores"]) => {
                post_score(ctx, leaderboard_id, deser_json(&req.body)?).await
            }
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

/// Creates a new leaderboard. Stores a leaderboard.{id} item in the KV store
/// as well as a leaderboards item that maintains a JSON payload of the leaderboard
/// summaries.
async fn create_leaderboard(
    ctx: &Context,
    leaderboard: LeaderboardSummary,
) -> RpcResult<HttpResponse> {
    let lb = Leaderboard {
        id: leaderboard.id.to_string(),
        name: leaderboard.name.to_string(),
        scores: vec![],
    };
    let raw = serde_json::to_string(&lb).map_err(|e| RpcError::Ser(e.to_string()))?;
    let key = format!("leaderboard.{}", leaderboard.id);
    let kv = KeyValueSender::new();
    kv.set(
        ctx,
        &SetRequest {
            key,
            value: raw,
            expires: 0,
        },
    )
    .await?;

    let mut list = get_leaderboard_list(ctx).await;
    list.push(leaderboard);

    kv.set(
        ctx,
        &SetRequest {
            key: "leaderboards".to_string(),
            value: serde_json::to_string(&list).unwrap(),
            expires: 0,
        },
    )
    .await?;

    Ok(HttpResponse::ok(
        serde_json::to_string(&list).unwrap_or_default(),
    ))
}

/// Returns an HTTP 200 with a list of board summaries: ID and name.
async fn get_leaderboards(ctx: &Context) -> RpcResult<HttpResponse> {
    let boards = get_leaderboard_list(ctx).await;
    let raw = serde_json::to_vec(&boards).unwrap_or_default();
    Ok(HttpResponse::ok(raw))
}

/// Utility function used to grab a list of leaderboard summaries
async fn get_leaderboard_list(ctx: &Context) -> Vec<LeaderboardSummary> {
    let kv = KeyValueSender::new();
    let Ok(res) = kv.get(ctx, "leaderboards").await else {
        return vec![]
    };
    if res.exists {
        serde_json::from_str(&res.value).unwrap_or_default()
    } else {
        vec![]
    }
}

/// Retrieves leaderboard details and renders them
async fn get_leaderboard(ctx: &Context, id: &str) -> RpcResult<HttpResponse> {
    let kv = KeyValueSender::new();
    let Ok(res) = kv.get(ctx, &format!("leaderboard.{}", id)).await else {
        return Ok(HttpResponse::not_found())
    };
    if res.exists {
        Ok(HttpResponse::ok(res.value))
    } else {
        Ok(HttpResponse::not_found())
    }
}

/// Posts a score to a leaderboard
async fn post_score(ctx: &Context, id: &str, score: ScoreRecord) -> RpcResult<HttpResponse> {
    let kv = KeyValueSender::new();
    let key = format!("leaderboard.{}", id);
    let Ok(res) = kv.get(ctx, &key).await else {
        return Ok(HttpResponse::not_found())
    };
    if res.exists {
        let mut board: Leaderboard =
            serde_json::from_str(&res.value).map_err(|e| RpcError::Deser(e.to_string()))?;
        board.add_score(score);
        kv.set(
            ctx,
            &SetRequest {
                key,
                value: serde_json::to_string(&board).unwrap(),
                expires: 0,
            },
        )
        .await?;
        Ok(HttpResponse::ok(serde_json::to_string(&board).unwrap()))
    } else {
        Ok(HttpResponse::not_found())
    }
}

/// Utility function to deserialize a JSON blob into a strongly typed value
fn deser_json<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(e.to_string()))
}

/// Represents a single row on a leaderboard
#[derive(Serialize, Deserialize, Default, Clone)]
struct ScoreRecord {
    pub owner_id: String,
    pub owner_name: String,
    pub value: u32,
}

/// Represents a leaderboard summary and its contained scores
#[derive(Serialize, Deserialize, Default, Clone)]
struct Leaderboard {
    pub id: String,
    pub name: String,
    pub scores: Vec<ScoreRecord>,
}

impl Leaderboard {
    fn add_score(&mut self, score: ScoreRecord) {
        self.scores.push(score);
        self.scores.sort_by(|a, b| b.value.cmp(&a.value));
        self.scores.truncate(MAX_SCORES as _);
    }
}

/// Structure used when listing out summary information for leaderboards
#[derive(Serialize, Deserialize, Default, Clone)]
struct LeaderboardSummary {
    pub id: String,
    pub name: String,
}
