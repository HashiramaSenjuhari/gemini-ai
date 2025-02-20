use crate::{
    Config, ConfigBuilder, ConfigNotPresent, ConfigPresent, EnvVariablePresent, Gemini,
    GeminiContentGen, InstructionNotPresent, InstructionPresent, Kind, MaxLenNotPresent,
    MaxLenPresent, Memory, MemoryNot, ModelPresent, PropertiesNotPresent, PropertiesPresent,
    TextNotPresent, TextPresent, TokenLen,
};

impl<'properties>
    Gemini<
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
    ) -> Gemini<
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
        Gemini {
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
    Gemini<
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
    ) -> Gemini<
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
        Gemini {
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
    Gemini<
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
    ) -> Gemini<
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
        Gemini {
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
    Gemini<
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
    ) -> Gemini<
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
        Gemini {
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
    Gemini<
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
