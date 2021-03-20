use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum EngineCommand {
    Stop,
    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,
}

impl FromStr for EngineCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EngineCommand::*;

        let cmd = match s {
            "move north" => MoveNorth,
            "move east" => MoveEast,
            "move south" => MoveSouth,
            "move west" => MoveWest,
            _ => return Err(()),
        };

        Ok(cmd)
    }
}
