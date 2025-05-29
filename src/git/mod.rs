//! Git 操作抽象层
//! 
//! 提供对 Git 操作的高级抽象，封装 git2 库的复杂性。

pub mod repository;
pub mod branch;
pub mod commit;
pub mod remote;
pub mod stash;

pub use repository::Repository;
pub use branch::BranchManager;
pub use commit::CommitManager;
pub use remote::RemoteManager;
pub use stash::StashManager; 