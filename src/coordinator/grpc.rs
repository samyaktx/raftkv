//! Coordinator gRPC service (internal)

use crate::proto::{AppendRequest, AppendResponse, HeartbeatRequest, HeartbeatResponse, JoinRequest, JoinResponse, SnapshotRequest, SnapshotResponse, VoteRequest, VoteResponse, coordinator_internal_server::CoordinatorInternal};

use tonic::{Request, Response, Status};

pub struct CoordGrpcService {}

impl CoordGrpcService {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl CoordinatorInternal for CoordGrpcService {
    async fn request_vote(&self, _req: Request<VoteRequest>) -> Result<Response<VoteResponse>, Status> {
        // Todo: Implement Raft RequestVote RPC
        Err(Status::unimplemented("RequestVote not implemented"))
    }

    async fn append_entries(&self, _req: Request<AppendRequest>) -> Result<Response<AppendResponse>, Status> {
        // Todo: Implement Raft AppendEntries RPC
        Err(Status::unimplemented("AppendEntries not implemeted"))
    }

    async fn install_snapshot(&self, _req: Request<SnapshotRequest>) -> Result<Response<SnapshotResponse>, Status> {
        Err(Status::unimplemented("InstallSnapshot not implemented"))
    }

    async fn join(&self, _req: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        // Todo: Handle volume registration
        Ok(Response::new(JoinResponse {
            ok: true,
            cluster_id: "cluster-1".to_string(),
        }))
    }

    async fn heartbeat(&self, _req: Request<HeartbeatRequest>) -> Result<Response<HeartbeatResponse>, Status> {
        // Todo: Update volume state
        Ok(Response::new(HeartbeatResponse { 
            ok: true, 
            commands: vec![] 
        }))
    }
}