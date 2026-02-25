//! # minikv
//!
//! A production-grade distributed key-value store with:
//! - Raft consensus for coordinator high availability
//! - Write-ahead log (WAL) for durability
//! - Automatic compaction and rebalancing
//! - gRPC for internal coordination, HTTP for public API
//! - Bloom filters and index snapshots for performance
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │         Coordinator Cluster             │
//! │  (Raft consensus for metadata)          │
//! │   - Leader: handles writes              │
//! │   - Followers: replicate state          │
//! └─────────────────┬───────────────────────┘
//!                   │ gRPC
//!       ┌───────────┴────┬──────────────┐
//!       │                │              │
//! ┌─────▼──────┐   ┌─────▼──────┐   ┌──▼───────────┐
//! │ Volume 1   │   │ Volume 2   │   │ Volume 3     │
//! │ (Shard A)  │   │ (Shard B)  │   │ (Shard C)    │
//! │  + WAL     │   │  + WAL     │   │  + WAL       │
//! └────────────┘   └────────────┘   └──────────────┘
//! ```
//!
//! ## Usage
//!
//! ### Start a coordinator
//! ```bash
//! raft-kv-coord serve \
//!   --id coord-1 \
//!   --bind 0.0.0.0:5000 \
//!   --db ./coord-data \
//!   --peers coord-2:5001,coord-3:5002
//! ```
//!
//! ### Start a volume server
//! ```bash
//! raft-kv-volume serve \
//!   --id vol-1 \
//!   --bind 0.0.0.0:6000 \
//!   --data ./vol-data \
//!   --coordinator http://localhost:5000
//! ```
//!
//! ### Use the CLI
//! ```bash
//! # Put a blob
//! raft-kv put my-key --file ./data.bin --coordinator http://localhost:5000
//!
//! # Get a blob
//! raft-kv get my-key --output ./out.bin
//!
//! # Delete
//! raft-kv delete my-key
//!
//! # Ops commands
//! raft-kv verify --deep
//! raft-kv repair --replicas 3
//! raft-kv compact --shard 0
//! ```

pub mod common;
pub mod coordinator;
pub mod volume;
pub mod ops;

// Re-export commonly used types
pub use common::{Config, Error, Result};
pub use coordinator::Coordinator;
pub use volume::VolumeServer;

// Generated protobuf code
pub mod proto {
    tonic::include_proto!("minikv");
}

/// Current version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Build info
pub const BUILD_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_NAME"),
    ")"
);