#[derive(Clone)]
pub enum EngineCommand {
    Stop,
    Command(String),
}
