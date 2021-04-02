use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum EngineCommand {
    Stop,
    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,
    Load(Option<PathBuf>),
}

impl FromStr for EngineCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EngineCommand::*;

        let cmd = match s.trim() {
            "move north" => MoveNorth,
            "move east" => MoveEast,
            "move south" => MoveSouth,
            "move west" => MoveWest,
            "load" => Load(None),
            s if s.starts_with("load ") => Load(Some(s[5..].trim().parse().map_err(|_| ())?)),
            _ => return Err(()),
        };

        Ok(cmd)
    }
}
