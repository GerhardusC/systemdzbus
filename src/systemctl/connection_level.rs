pub enum ConnectionLevel {
    /// Create a Connection to the session/user message bus.
    UserLevel,
    /// Create a Connection to the system-wide message bus.
    SystemLevel,
}
