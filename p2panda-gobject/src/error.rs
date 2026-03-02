#[derive(Debug, Copy, Clone, glib::Enum, glib::ErrorDomain)]
#[error_domain(name = "p2panda-error")]
#[repr(C)]
#[enum_type(name = "P2pandaError")]
#[non_exhaustive]
pub enum Error {
    Failed = 0,
    SpawnNode = 1,
    SpawnTopic = 2,
    NotSpawned = 3,
    Decoding = 4,
    Replay = 5,
    HasNoPersistent = 6,
    HasNoEphemeral = 7,
    Publish = 8,
    Signature = 9,
}
