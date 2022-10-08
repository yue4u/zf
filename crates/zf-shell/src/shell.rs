use anyhow::Result;
use nu_command::*;
use nu_engine::eval_block;
use nu_parser::parse;
use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    CliError, PipelineData, Span,
};

use crate::commands::{Game, GameStart, Hi};

macro_rules! eval {
    ($line:ident with $( $command:expr ),* $(,)? ) => {
        let mut engine_state = EngineState::new();
        let mut stack = Stack::new();
        let mut working_set = StateWorkingSet::new(&engine_state);

        $( working_set.add_decl(Box::new($command)); )*

        let delta = working_set.render();
        engine_state.merge_delta(delta).unwrap();

        eval_impl(
            &mut engine_state, //
            &mut stack,
            $line,
        )
    };
}

pub fn eval(line: String) -> Result<String> {
    eval! {
        line with
        Hi,
        // game
        Game,
        GameStart,

        // default_context below

        // // If there are commands that have the same name as default declarations,
        // // they have to be registered before the main declarations. This helps to make
        // // them only accessible if the correct input value category is used with the
        // // declaration
        // #[cfg(feature = "dataframe")]
        // add_dataframe_decls(&mut working_set);

        // // Database-related
        // // Adds all related commands to query databases
        // #[cfg(feature = "database")]
        // add_database_decls(&mut working_set);

        // // Core
        // bind_command! {
            Alias,
            Ast,
            Commandline,
            Debug,
            Def,
            DefEnv,
            Describe,
            Do,
            Echo,
            ErrorMake,
            ExportAlias,
            ExportCommand,
            ExportDef,
            ExportDefEnv,
            ExportExtern,
            ExportUse,
            Extern,
            For,
            Help,
            Hide,
            HideEnv,
            If,
            Ignore,
            Overlay,
            OverlayUse,
            OverlayList,
            OverlayNew,
            OverlayHide,
            Let,
            Metadata,
            Module,
            Use,
            // Version,
        // };

        // // Charts
        // bind_command! {
        //     Histogram
        // }

        // // Filters
        // bind_command! {
        //     All,
        //     Any,
        //     Append,
        //     Collect,
        //     Columns,
        //     Compact,
        //     Default,
        //     Drop,
        //     DropColumn,
        //     DropNth,
        //     Each,
        //     EachWhile,
        //     Empty,
        //     Every,
        //     Find,
        //     First,
        //     Flatten,
        //     Get,
        //     Group,
        //     GroupBy,
        //     Headers,
        //     Insert,
        //     SplitBy,
        //     Take,
        //     Merge,
        //     Move,
        //     TakeWhile,
        //     TakeUntil,
        //     Last,
        //     Length,
        //     Lines,
        //     ParEach,
        //     Prepend,
        //     Range,
        //     Reduce,
        //     Reject,
        //     Rename,
        //     Reverse,
        //     Roll,
        //     RollDown,
        //     RollUp,
        //     RollLeft,
        //     RollRight,
        //     Rotate,
        //     Select,
        //     Shuffle,
        //     Skip,
        //     SkipUntil,
        //     SkipWhile,
        //     Sort,
        //     SortBy,
        //     SplitList,
        //     Transpose,
        //     Uniq,
        //     Upsert,
        //     Update,
        //     UpdateCells,
        //     Where,
        //     Window,
        //     Wrap,
        //     Zip,
        // };

        // // Misc
        // bind_command! {
        //     History,
        //     Tutor,
        //     HistorySession,
        // };

        // // Path
        // bind_command! {
        //     Path,
        //     PathBasename,
        //     PathDirname,
        //     PathExists,
        //     PathExpand,
        //     PathJoin,
        //     PathParse,
        //     PathRelativeTo,
        //     PathSplit,
        //     PathType,
        // };

        // // System
        // bind_command! {
        //     Benchmark,
        //     Complete,
        //     Exec,
        //     External,
        //     NuCheck,
        //     Sys,
        // };

        // #[cfg(any(
        //     target_os = "android",
        //     target_os = "linux",
        //     target_os = "macos",
        //     target_os = "windows"
        // ))]
        // bind_command! { Ps };

        // #[cfg(feature = "which-support")]
        // bind_command! { Which };

        // // Strings
        // bind_command! {
        //     BuildString,
        //     Char,
        //     Decode,
        //     Encode,
        //     DecodeBase64,
        //     EncodeBase64,
        //     DetectColumns,
        //     Format,
        //     FileSize,
        //     Parse,
        //     Size,
        //     Split,
        //     SplitChars,
        //     SplitColumn,
        //     SplitRow,
        //     SplitWords,
        //     Str,
        //     StrCamelCase,
        //     StrCapitalize,
        //     StrCollect,
        //     StrContains,
        //     StrDistance,
        //     StrDowncase,
        //     StrEndswith,
        //     StrJoin,
        //     StrReplace,
        //     StrIndexOf,
        //     StrKebabCase,
        //     StrLength,
        //     StrLpad,
        //     StrPascalCase,
        //     StrReverse,
        //     StrRpad,
        //     StrScreamingSnakeCase,
        //     StrSnakeCase,
        //     StrStartsWith,
        //     StrSubstring,
        //     StrTrim,
        //     StrTitleCase,
        //     StrUpcase
        // };

        // // Bits
        // bind_command! {
        //     Bits,
        //     BitsAnd,
        //     BitsNot,
        //     BitsOr,
        //     BitsXor,
        //     BitsRotateLeft,
        //     BitsRotateRight,
        //     BitsShiftLeft,
        //     BitsShiftRight,
        // }

        // // Bytes
        // bind_command! {
        //     Bytes,
        //     BytesLen,
        //     BytesStartsWith,
        //     BytesEndsWith,
        //     BytesReverse,
        //     BytesReplace,
        //     BytesAdd,
        //     BytesAt,
        //     BytesIndexOf,
        //     BytesCollect,
        //     BytesRemove,
        //     BytesBuild,
        // }

        // // FileSystem
        // bind_command! {
        //     Cd,
        //     Cp,
        //     Ls,
        //     Mkdir,
        //     Mv,
        //     Open,
        //     Rm,
        //     Save,
        //     Touch,
        //     Glob,
        //     Watch,
        // };

        // // Platform
        // bind_command! {
        //     Ansi,
        //     AnsiGradient,
        //     AnsiStrip,
        //     Clear,
        //     Du,
        //     KeybindingsDefault,
        //     Input,
        //     KeybindingsListen,
        //     Keybindings,
        //     Kill,
        //     KeybindingsList,
        //     Sleep,
        //     TermSize,
        // };

        // // Date
        // bind_command! {
            Date,
            DateFormat,
            DateHumanize,
            DateListTimezones,
            DateNow,
            DateToRecord,
            DateToTable,
            DateToTimezone,
        // };

        // // Shells
        // bind_command! {
        //     Enter,
        //     Exit,
        //     GotoShell,
        //     NextShell,
        //     PrevShell,
        //     Shells,
        // };

        // // Formats
        // bind_command! {
        //     From,
        //     FromCsv,
        //     FromEml,
        //     FromIcs,
        //     FromIni,
        //     FromJson,
        //     FromNuon,
        //     FromOds,
        //     FromSsv,
        //     FromToml,
        //     FromTsv,
        //     FromUrl,
        //     FromVcf,
        //     FromXlsx,
        //     FromXml,
        //     FromYaml,
        //     FromYml,
        //     To,
        //     ToCsv,
        //     ToHtml,
        //     ToJson,
        //     ToMd,
        //     ToNuon,
        //     ToText,
        //     ToToml,
        //     ToTsv,
        //     ToCsv,
        //     Touch,
        //     Use,
        //     Upsert,
        //     Where,
        //     ToUrl,
        //     ToXml,
        //     ToYaml,
        // };

        // // Viewers
        // bind_command! {
        //     Griddle,
        //     Table,
        // };

        // // Conversions
        // bind_command! {
        //     Fmt,
        //     Into,
        //     IntoBool,
        //     IntoBinary,
        //     IntoDatetime,
        //     IntoDecimal,
        //     IntoDuration,
        //     IntoFilesize,
        //     IntoInt,
        //     IntoString,
        // };

        // // Env
        // bind_command! {
        //     Env,
        //     ExportEnv,
        //     LetEnv,
        //     LoadEnv,
        //     SourceEnv,
        //     WithEnv,
        //     ConfigNu,
        //     ConfigEnv,
        //     ConfigMeta,
        //     ConfigReset,
        // };

        // // Math
        // bind_command! {
            Math,
            MathAbs,
            MathAvg,
            MathCeil,
            MathEval,
            MathFloor,
            MathMax,
            MathMedian,
            MathMin,
            MathMode,
            MathProduct,
            MathRound,
            MathSqrt,
            MathStddev,
            MathSum,
            MathVariance,
        // };

        // // Network
        // bind_command! {
        //     Fetch,
        //     Post,
        //     Url,
        //     UrlHost,
        //     UrlPath,
        //     UrlQuery,
        //     UrlScheme,
        //     Port,
        // }

        // // Random
        // bind_command! {
            Random,
            RandomBool,
            RandomChars,
            RandomDecimal,
            RandomDice,
            RandomInteger,
            RandomUuid,
        // };

        // // Generators
        // bind_command! {
        //     Cal,
        //     Seq,
        //     SeqDate,
        //     SeqChar,
        // };

        // // Hash
        // bind_command! {
        //     Hash,
        //     HashMd5::default(),
        //     HashSha256::default(),
        // };

        // // Experimental
        // bind_command! {
        //     ViewSource,
        //     IsAdmin,
        // };

        // // Deprecated
        // bind_command! {
        //     HashBase64,
        //     Source,
        //     StrDatetimeDeprecated,
        //     StrDecimalDeprecated,
        //     StrIntDeprecated,
        //     StrFindReplaceDeprecated,
        // };

        // #[cfg(feature = "plugin")]
        // bind_command!(Register);

    }
}

trait CheckOutcome<T, E>
where
    E: miette::Diagnostic + Send + Sync + 'static,
{
    fn check_outcome(self, engine_state: &EngineState) -> anyhow::Result<T>;
}

impl<T, E> CheckOutcome<T, E> for Result<T, E>
where
    E: miette::Diagnostic + Send + Sync + 'static,
{
    fn check_outcome(self, engine_state: &EngineState) -> anyhow::Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(err) => {
                let working_set = StateWorkingSet::new(engine_state);
                Err(anyhow::Error::msg(format!(
                    "Error: {:?}",
                    CliError(&err, &working_set)
                )))
            }
        }
    }
}

pub fn eval_impl(
    engine_state: &mut EngineState,
    stack: &mut Stack,
    source_lines: String,
) -> Result<String> {
    let mut last_output = String::new();

    for (i, line) in source_lines.lines().enumerate() {
        let mut working_set = StateWorkingSet::new(&engine_state);
        let (block, err) = parse(
            &mut working_set,
            Some(&format!("line{}", i)),
            line.as_bytes(),
            false,
            &[],
        );

        if let Some(err) = err {
            Err(err).check_outcome(engine_state)?
        }

        let delta = working_set.render();

        engine_state
            .merge_delta(delta)
            .check_outcome(engine_state)?;

        let input = PipelineData::new(Span::test_data());
        let config = engine_state.get_config();

        last_output = eval_block(
            &engine_state, //
            stack,
            &block,
            input,
            false,
            false,
        )
        .check_outcome(engine_state)?
        .collect_string("", config)
        .check_outcome(engine_state)?;
    }
    Ok(last_output)
}
