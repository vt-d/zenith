use super::{Identifier, Span, expressions::Expression};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Bool,
    Char,
    Str,
    Unit,
    Never,

    Array(Box<Type>, Option<Box<Expression>>),
    Slice(Box<Type>),
    Pointer(Box<Type>, Mutability),
    Reference(Box<Type>, Mutability),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>),

    Named(TypePath),
    Generic(Box<Type>, Vec<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypePath {
    pub segments: Vec<TypePathSegment>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypePathSegment {
    pub ident: Identifier,
    pub generic_args: Option<Vec<Type>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Mutability {
    Mutable,
    Immutable,
}

impl Type {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::I128
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::U128
                | Type::F32
                | Type::F64
                | Type::Bool
                | Type::Char
                | Type::Str
                | Type::Unit
                | Type::Never
        )
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::I128
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::U128
                | Type::F32
                | Type::F64
        )
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::I128
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::U128
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Type::F32 | Type::F64)
    }

    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::F32 | Type::F64
        )
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::I128 => write!(f, "i128"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::U128 => write!(f, "u128"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::Str => write!(f, "str"),
            Type::Unit => write!(f, "()"),
            Type::Never => write!(f, "!"),
            Type::Array(ty, size) => {
                write!(f, "[")?;
                ty.fmt(f)?;
                if let Some(size) = size {
                    write!(f, "; {:?}", size)?;
                }
                write!(f, "]")
            }
            Type::Slice(ty) => write!(f, "[{}]", ty),
            Type::Pointer(ty, mutability) => {
                match mutability {
                    Mutability::Mutable => write!(f, "*mut ")?,
                    Mutability::Immutable => write!(f, "*const ")?,
                }
                ty.fmt(f)
            }
            Type::Reference(ty, mutability) => {
                match mutability {
                    Mutability::Mutable => write!(f, "&mut ")?,
                    Mutability::Immutable => write!(f, "&")?,
                }
                ty.fmt(f)
            }
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    ty.fmt(f)?;
                }
                write!(f, ")")
            }
            Type::Function(params, return_ty) => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    param.fmt(f)?;
                }
                write!(f, ") -> ")?;
                return_ty.fmt(f)
            }
            Type::Named(path) => {
                for (i, segment) in path.segments.iter().enumerate() {
                    if i > 0 {
                        write!(f, "::")?;
                    }
                    write!(f, "{}", segment.ident.name)?;
                    if let Some(args) = &segment.generic_args {
                        write!(f, "<")?;
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            arg.fmt(f)?;
                        }
                        write!(f, ">")?;
                    }
                }
                Ok(())
            }
            Type::Generic(base, args) => {
                base.fmt(f)?;
                write!(f, "<")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    arg.fmt(f)?;
                }
                write!(f, ">")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_span() -> Span {
        Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }

    #[test]
    fn test_primitive_types() {
        assert!(Type::I32.is_primitive());
        assert!(Type::F64.is_primitive());
        assert!(Type::Bool.is_primitive());
        assert!(!Type::Array(Box::new(Type::I32), None).is_primitive());
    }

    #[test]
    fn test_numeric_types() {
        assert!(Type::I32.is_numeric());
        assert!(Type::F64.is_numeric());
        assert!(!Type::Bool.is_numeric());
        assert!(Type::I32.is_integer());
        assert!(!Type::F64.is_integer());
        assert!(Type::F64.is_float());
    }

    #[test]
    fn test_complex_types() {
        let array_type = Type::Array(Box::new(Type::I32), None);
        let ptr_type = Type::Pointer(Box::new(Type::I32), Mutability::Immutable);
        let ref_type = Type::Reference(Box::new(Type::I32), Mutability::Mutable);
        let tuple_type = Type::Tuple(vec![Type::I32, Type::F64]);
        let fn_type = Type::Function(vec![Type::I32], Box::new(Type::Bool));

        assert!(!array_type.is_primitive());
        assert!(!ptr_type.is_primitive());
        assert!(!ref_type.is_primitive());
        assert!(!tuple_type.is_primitive());
        assert!(!fn_type.is_primitive());
    }

    #[test]
    fn test_generic_types() {
        let generic_path = TypePath {
            segments: vec![TypePathSegment {
                ident: Identifier::new("Vec".to_string(), dummy_span()),
                generic_args: Some(vec![Type::I32]),
                span: dummy_span(),
            }],
            span: dummy_span(),
        };

        let named_type = Type::Named(generic_path);
        assert!(!named_type.is_primitive());
    }

    #[test]
    fn test_type_display() {
        assert_eq!(Type::I32.to_string(), "i32");
        assert_eq!(Type::Array(Box::new(Type::I32), None).to_string(), "[i32]");
        assert_eq!(
            Type::Pointer(Box::new(Type::I32), Mutability::Mutable).to_string(),
            "*mut i32"
        );
        assert_eq!(
            Type::Tuple(vec![Type::I32, Type::F64]).to_string(),
            "(i32, f64)"
        );
    }
}
