macro_rules! empty {
    ($_struct:ident, name: $name:literal, usage: $usage:literal ) => {
        #[derive(Clone)]
        pub(crate) struct $_struct;

        impl nu_protocol::engine::Command for $_struct {
            fn name(&self) -> &str {
                $name
            }

            fn signature(&self) -> nu_protocol::Signature {
                Signature::build(self.name())
            }

            fn usage(&self) -> &str {
                $usage
            }

            fn run(
                &self,
                _engine_state: &nu_protocol::engine::EngineState,
                _stack: &mut nu_protocol::engine::Stack,
                call: &nu_protocol::ast::Call,
                _input: nu_protocol::PipelineData,
            ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
                Ok(nu_protocol::Value::Nothing { span: call.head }.into_pipeline_data())
            }
        }
    };
}

macro_rules! proxy {
    ($_struct:ident, name: $name:literal, usage: $usage:literal, arg: $arg:expr ) => {
        #[derive(Clone)]
        pub(crate) struct $_struct;

        impl nu_protocol::engine::Command for $_struct {
            fn name(&self) -> &str {
                $name
            }

            fn signature(&self) -> nu_protocol::Signature {
                Signature::build(self.name())
            }

            fn usage(&self) -> &str {
                $usage
            }

            fn run(
                &self,
                _engine_state: &nu_protocol::engine::EngineState,
                _stack: &mut nu_protocol::engine::Stack,
                call: &nu_protocol::ast::Call,
                _input: nu_protocol::PipelineData,
            ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
                let val = zf_ffi::zf_call($arg);
                // QUESTION: 1. use json or nu_json or nuon?
                // QUESTION: 2. diff json result vs non json result?
                match nu_command::try_convert_str_to_value(&val, call.head) {
                    Ok(val) => Ok(val.into_pipeline_data()),
                    Err(_) => Ok(nu_protocol::Value::String {
                        val,
                        span: call.head,
                    }
                    .into_pipeline_data()),
                }
            }
        }
    };
}

pub(crate) use empty;
pub(crate) use proxy;
