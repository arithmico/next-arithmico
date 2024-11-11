#[derive(Debug, PartialEq, Clone)]
pub enum EvaluateNodeErrorKind {
    UnsupportedOperation,
    UnsupportedDataType,
    UnknownSymbol,
    RuntimeError,
    InvalidNode,
    IncompatibleVectorDimensions,
    IncompatibleMatrixDimensions,
    DivisionByZero,
    InvalidNumberOfArguments,
    MissingParameter,
    InvalidRepeatableParameterCount,
    InvalidParameterType,
    TooManyParameters,
}
