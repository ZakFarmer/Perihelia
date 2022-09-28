#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum OnlineMenuState {
    ConnectToIp,
    CreateServer,
    Profile,
    ServerList,
}
