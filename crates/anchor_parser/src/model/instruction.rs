//! Instruction model for Anchor program instructions
//!
//! In Anchor, instructions are functions within a module marked with the #[program] attribute.
//! They define the entry points and behavior of a Solana program.

/// Represents an instruction in an Anchor program
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Instruction {
    /// Name of the instruction
    pub name: String,

    /// Visibility of the instruction ("", "pub", "pub(crate)", etc.)
    pub visibility: String,

    /// Parameters to the instruction
    pub parameters: Vec<Parameter>,

    /// Return type (if any)
    pub return_type: Option<String>,

    /// Type of the context parameter (e.g., "Initialize")
    pub context_type: Option<String>,
}

/// Represents a parameter to an instruction
#[derive(Debug, Clone, Serialize)]
pub struct Parameter {
    /// Name of the parameter
    pub name: String,

    /// Type of the parameter
    pub ty: String,

    /// Whether this is a Context parameter
    pub is_context: bool,
}

impl Instruction {
    /// Create a new instruction with the given name and visibility
    pub fn new(name: impl Into<String>, visibility: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility.into(),
            parameters: Vec::new(),
            return_type: None,
            context_type: None,
        }
    }

    /// Add a parameter to the instruction
    pub fn add_parameter(&mut self, parameter: Parameter) {
        self.parameters.push(parameter);
    }

    /// Set the return type of the instruction
    pub fn set_return_type(&mut self, ty: impl Into<String>) {
        self.return_type = Some(ty.into());
    }

    /// Set the context type of the instruction
    pub fn set_context_type(&mut self, ty: impl Into<String>) {
        self.context_type = Some(ty.into());
    }

    /// Find a parameter by name
    pub fn find_parameter(&self, name: &str) -> Option<&Parameter> {
        self.parameters.iter().find(|p| p.name == name)
    }

    /// Check if this instruction has a context parameter
    pub fn has_context(&self) -> bool {
        self.parameters.iter().any(|p| p.is_context)
    }

    /// Builder method: with parameters
    pub fn with_parameters(mut self, parameters: Vec<Parameter>) -> Self {
        self.parameters = parameters;
        self
    }

    /// Builder method: with a single parameter
    pub fn with_parameter(mut self, parameter: Parameter) -> Self {
        self.add_parameter(parameter);
        self
    }

    /// Builder method: with return type
    pub fn with_return_type(mut self, ty: impl Into<String>) -> Self {
        self.return_type = Some(ty.into());
        self
    }

    /// Builder method: with context type
    pub fn with_context_type(mut self, ty: impl Into<String>) -> Self {
        self.context_type = Some(ty.into());
        self
    }
}

impl Parameter {
    /// Create a new parameter
    pub fn new(name: impl Into<String>, ty: impl Into<String>, is_context: bool) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            is_context,
        }
    }

    /// Create a new context parameter
    pub fn new_context(name: impl Into<String>, context_type: impl Into<String>) -> Self {
        let context_type = context_type.into();
        Self {
            name: name.into(),
            ty: format!("Context<{}>", context_type),
            is_context: true,
        }
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Self {
            name: String::new(),
            ty: String::new(),
            is_context: false,
        }
    }
}

#[cfg(all(test, feature = "unit_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_new() {
        let instruction = Instruction::new("initialize", "pub");
        assert_eq!(instruction.name, "initialize");
        assert_eq!(instruction.visibility, "pub");
        assert!(instruction.parameters.is_empty());
        assert!(instruction.return_type.is_none());
        assert!(instruction.context_type.is_none());
    }

    #[test]
    fn test_instruction_default() {
        let instruction = Instruction::default();
        assert_eq!(instruction.name, "");
        assert_eq!(instruction.visibility, "");
        assert!(instruction.parameters.is_empty());
        assert!(instruction.return_type.is_none());
        assert!(instruction.context_type.is_none());
    }

    #[test]
    fn test_instruction_add_parameter() {
        let mut instruction = Instruction::new("initialize", "pub");
        let param = Parameter::new("amount", "u64", false);
        instruction.add_parameter(param);

        assert_eq!(instruction.parameters.len(), 1);
        assert_eq!(instruction.parameters[0].name, "amount");
        assert_eq!(instruction.parameters[0].ty, "u64");
        assert!(!instruction.parameters[0].is_context);
    }

    #[test]
    fn test_instruction_set_return_type() {
        let mut instruction = Instruction::new("initialize", "pub");
        instruction.set_return_type("Result<()>");

        assert!(instruction.return_type.is_some());
        assert_eq!(instruction.return_type.unwrap(), "Result<()>");
    }

    #[test]
    fn test_instruction_set_context_type() {
        let mut instruction = Instruction::new("initialize", "pub");
        instruction.set_context_type("Initialize");

        assert!(instruction.context_type.is_some());
        assert_eq!(instruction.context_type.unwrap(), "Initialize");
    }

    #[test]
    fn test_instruction_find_parameter() {
        let mut instruction = Instruction::new("initialize", "pub");
        instruction.add_parameter(Parameter::new("amount", "u64", false));
        instruction.add_parameter(Parameter::new("recipient", "Pubkey", false));

        let param = instruction.find_parameter("amount");
        assert!(param.is_some());
        assert_eq!(param.unwrap().ty, "u64");

        let param = instruction.find_parameter("unknown");
        assert!(param.is_none());
    }

    #[test]
    fn test_instruction_has_context() {
        let mut instruction1 = Instruction::new("initialize", "pub");
        instruction1.add_parameter(Parameter::new("amount", "u64", false));
        assert!(!instruction1.has_context());

        let mut instruction2 = Instruction::new("initialize", "pub");
        instruction2.add_parameter(Parameter::new_context("ctx", "Initialize"));
        assert!(instruction2.has_context());
    }

    #[test]
    fn test_instruction_builder_methods() {
        let params = vec![
            Parameter::new_context("ctx", "Initialize"),
            Parameter::new("amount", "u64", false),
        ];

        // Test with_parameters
        let instruction = Instruction::new("initialize", "pub")
            .with_parameters(params.clone())
            .with_return_type("Result<()>")
            .with_context_type("Initialize");

        assert_eq!(instruction.parameters.len(), 2);
        assert_eq!(instruction.return_type, Some("Result<()>".to_string()));
        assert_eq!(instruction.context_type, Some("Initialize".to_string()));

        // Test with_parameter
        let instruction = Instruction::new("initialize", "pub")
            .with_parameter(params[0].clone())
            .with_parameter(params[1].clone())
            .with_return_type("Result<()>");

        assert_eq!(instruction.parameters.len(), 2);
        assert_eq!(instruction.parameters[0].name, "ctx");
        assert_eq!(instruction.parameters[1].name, "amount");
    }

    #[test]
    fn test_parameter_new() {
        let param = Parameter::new("amount", "u64", false);
        assert_eq!(param.name, "amount");
        assert_eq!(param.ty, "u64");
        assert!(!param.is_context);
    }

    #[test]
    fn test_parameter_default() {
        let param = Parameter::default();
        assert_eq!(param.name, "");
        assert_eq!(param.ty, "");
        assert!(!param.is_context);
    }

    #[test]
    fn test_parameter_new_context() {
        let param = Parameter::new_context("ctx", "Initialize");
        assert_eq!(param.name, "ctx");
        assert_eq!(param.ty, "Context<Initialize>");
        assert!(param.is_context);
    }

    #[test]
    fn test_string_conversions() {
        // Test flexibility in Instruction::new
        let instr1 = Instruction::new("static", "pub");
        let instr2 = Instruction::new(String::from("owned"), "pub");
        let name = String::from("reference");
        let instr3 = Instruction::new(&name, "pub");

        assert_eq!(instr1.name, "static");
        assert_eq!(instr2.name, "owned");
        assert_eq!(instr3.name, "reference");

        // Test flexibility in Parameter::new
        let param1 = Parameter::new("static", "u64", false);
        let param2 = Parameter::new(String::from("owned"), "u64", false);
        let name = String::from("reference");
        let param3 = Parameter::new(&name, "u64", false);

        assert_eq!(param1.name, "static");
        assert_eq!(param2.name, "owned");
        assert_eq!(param3.name, "reference");
    }
}
