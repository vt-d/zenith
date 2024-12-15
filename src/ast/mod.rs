pub mod expressions;
pub mod statements;
pub mod types;

use expressions::{Expression, Literal};
use statements::Statement;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    pub fn dummy() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Declaration>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Function(FunctionDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Union(UnionDecl),
    Variable(VarDecl),
    Constant(ConstDecl),
    Module(ModuleDecl),
    Macro(MacroDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub return_type: Option<Box<Type>>,
    pub body: Block,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: Identifier,
    pub args: Vec<AttributeArg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeArg {
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: Identifier,
    pub fields: Vec<StructField>,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: Identifier,
    pub ty: Type,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: Identifier,
    pub variants: Vec<EnumVariant>,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: Identifier,
    pub data: Option<Type>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDecl {
    pub name: Identifier,
    pub fields: Vec<UnionField>,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionField {
    pub name: Identifier,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub mutable: bool,
    pub initializer: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDecl {
    pub name: Identifier,
    pub ty: Type,
    pub value: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDecl {
    pub name: Identifier,
    pub items: Vec<Declaration>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDecl {
    pub name: Identifier,
    pub params: Vec<MacroParam>,
    pub body: MacroBody,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroParam {
    pub name: Identifier,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroBody {
    pub tokens: Vec<MacroToken>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroToken {
    Literal(String),
    Variable(Identifier),
    Group(Vec<MacroToken>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Simple(Identifier),
    Pointer(Box<Type>),
    Reference(Box<Type>),
    Array(Box<Type>, Box<Expression>),
    Function(Vec<Type>, Box<Type>),
    Generic(Box<Type>, Vec<Type>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        let span = Span::new(0, 3, 1, 1);
        let ident = Identifier::new("foo".to_string(), span.clone());
        assert_eq!(ident.name, "foo");
        assert_eq!(ident.span, span);
    }

    #[test]
    fn test_function_declaration() {
        let span = Span::new(0, 50, 1, 1);
        let fn_decl = FunctionDecl {
            name: Identifier::new("test".to_string(), Span::dummy()),
            params: vec![],
            return_type: None,
            body: Block {
                statements: vec![],
                span: Span::dummy(),
            },
            attributes: vec![],
            span,
        };

        assert_eq!(fn_decl.name.name, "test");
        assert_eq!(fn_decl.params.len(), 0);
        assert!(fn_decl.return_type.is_none());
    }

    #[test]
    fn test_struct_declaration() {
        let span = Span::new(0, 30, 1, 1);
        let struct_decl = StructDecl {
            name: Identifier::new("Point".to_string(), Span::dummy()),
            fields: vec![StructField {
                name: Identifier::new("x".to_string(), Span::dummy()),
                ty: Type::Simple(Identifier::new("i32".to_string(), Span::dummy())),
                attributes: vec![],
                span: Span::dummy(),
            }],
            attributes: vec![],
            span,
        };

        assert_eq!(struct_decl.name.name, "Point");
        assert_eq!(struct_decl.fields.len(), 1);
        assert_eq!(struct_decl.fields[0].name.name, "x");
    }

    #[test]
    fn test_type_constructions() {
        let i32_type = Type::Simple(Identifier::new("i32".to_string(), Span::dummy()));
        let ptr_type = Type::Pointer(Box::new(i32_type.clone()));
        let ref_type = Type::Reference(Box::new(i32_type.clone()));

        match ptr_type {
            Type::Pointer(inner) => match *inner {
                Type::Simple(ident) => assert_eq!(ident.name, "i32"),
                _ => panic!("Expected simple type"),
            },
            _ => panic!("Expected pointer type"),
        }

        match ref_type {
            Type::Reference(inner) => match *inner {
                Type::Simple(ident) => assert_eq!(ident.name, "i32"),
                _ => panic!("Expected simple type"),
            },
            _ => panic!("Expected reference type"),
        }
    }
}
