// impl Execute for Command {
//     fn exec(&self) -> ExecuteResult {
//         Ok(match self {
//             Command::Help => HELP.to_owned(),
//             Command::Mission(m) => m.exec()?,
//             _ => format!("{:?}", self),
//         })
//     }
// }

// impl Execute for MissionCommand {
//     fn exec(&self) -> ExecuteResult {
//         let ret = match self {
//             MissionCommand::Summary => Mission::dummy().summary(),
//             MissionCommand::Tartget => Mission::dummy().targets().join("\n"),
//             MissionCommand::Position => format!("{:?}", Mission::dummy().positions()),
//         };
//         Ok(ret)
//     }
// }
