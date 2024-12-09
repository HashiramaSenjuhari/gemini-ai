use crate::{
    Config, ConfigBuilder, ConfigNotPresent, ConfigPresent, EnvVariablePresent, GeminiContentGen,
    GeminiContentGenBuilder, InstructionNotPresent, InstructionPresent, Kind, MaxLenNotPresent,
    MaxLenPresent, Memory, MemoryNot, ModelPresent, PropertiesNotPresent, PropertiesPresent,
    TextNotPresent, TextPresent, TokenLen,
};

impl<'properties>
    GeminiContentGenBuilder<
        'properties,
        EnvVariablePresent,
        ModelPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        MemoryNot,
    >
{
    pub fn kind(
        mut self,
        response: Kind<'properties>,
    ) -> GeminiContentGenBuilder<
        'properties,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        MaxLenNotPresent,
        TextNotPresent,
        InstructionNotPresent,
        PropertiesPresent,
        MemoryNot,
    > {
        self.config.r#type = response;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
            memory: self.memory,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
}

impl<'instruction>
    GeminiContentGenBuilder<
        'instruction,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        MaxLenNotPresent,
        TextNotPresent,
        InstructionNotPresent,
        PropertiesPresent,
        MemoryNot,
    >
{
    pub fn instruction(
        mut self,
        instruction: &'instruction str,
    ) -> GeminiContentGenBuilder<
        'instruction,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionPresent,
        PropertiesPresent,
        MemoryNot,
    > {
        self.instruction = instruction;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
            memory: self.memory,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
}

impl<'text>
    GeminiContentGenBuilder<
        'text,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionPresent,
        PropertiesPresent,
        MemoryNot,
    >
{
    pub fn text(
        mut self,
        text: &'text str,
    ) -> GeminiContentGenBuilder<
        'text,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextPresent,
        MaxLenNotPresent,
        InstructionPresent,
        PropertiesPresent,
        MemoryNot,
    > {
        self.text = text;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
            memory: self.memory,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
}

impl<'max_len>
    GeminiContentGenBuilder<
        'max_len,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextPresent,
        MaxLenNotPresent,
        InstructionPresent,
        PropertiesPresent,
        MemoryNot,
    >
{
    pub fn max_token(
        mut self,
        max: TokenLen,
    ) -> GeminiContentGenBuilder<
        'max_len,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextPresent,
        MaxLenPresent,
        InstructionPresent,
        PropertiesPresent,
        MemoryNot,
    > {
        match max {
            TokenLen::Custome(values) => {
                self.max_len = values;
            }
            TokenLen::Default => self.max_len = 8192,
        }
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
            memory: self.memory,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
}

impl<'build>
    GeminiContentGenBuilder<
        'build,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextPresent,
        MaxLenPresent,
        InstructionPresent,
        PropertiesPresent,
        MemoryNot,
    >
{
    pub fn build(self) -> GeminiContentGen<'build> {
        GeminiContentGen {
            model: &self.model,
            env_variable: &self.env_variable,
            max_len: self.max_len,
            text: self.text,
            instruction: &self.instruction,
            config: Config {
                response: self.config.r#type,
            },
            memory: self.memory,
        }
    }
}
