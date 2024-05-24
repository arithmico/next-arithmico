use super::structs::{
    Boolean, Definition, Division, Function, FunctionCall,
    HostApiFunctionEndpoint, Negate, Number, Power, Product, Sum, Symbol,
    Tensor,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Number(Number),
    Symbol(Symbol),
    Boolean(Boolean),
    Negate(Negate),
    Sum(Sum),
    Product(Product),
    Division(Division),
    Power(Power),
    Tensor(Tensor),
    FunctionCall(FunctionCall),
    Function(Function),
    HostApiFunctionEndpoint(HostApiFunctionEndpoint),
    Definition(Definition),
}

impl From<Number> for Node {
    fn from(value: Number) -> Node {
        Node::Number(value)
    }
}

impl From<Symbol> for Node {
    fn from(value: Symbol) -> Node {
        Node::Symbol(value)
    }
}

impl From<Boolean> for Node {
    fn from(value: Boolean) -> Node {
        Node::Boolean(value)
    }
}

impl From<Negate> for Node {
    fn from(value: Negate) -> Node {
        Node::Negate(value)
    }
}

impl From<Sum> for Node {
    fn from(value: Sum) -> Node {
        Node::Sum(value)
    }
}

impl From<Product> for Node {
    fn from(value: Product) -> Node {
        Node::Product(value)
    }
}

impl From<Division> for Node {
    fn from(value: Division) -> Node {
        Node::Division(value)
    }
}

impl From<Power> for Node {
    fn from(value: Power) -> Node {
        Node::Power(value)
    }
}

impl From<Tensor> for Node {
    fn from(value: Tensor) -> Node {
        Node::Tensor(value)
    }
}

impl From<FunctionCall> for Node {
    fn from(value: FunctionCall) -> Node {
        Node::FunctionCall(value)
    }
}

impl From<Function> for Node {
    fn from(value: Function) -> Node {
        Node::Function(value)
    }
}

impl From<HostApiFunctionEndpoint> for Node {
    fn from(value: HostApiFunctionEndpoint) -> Node {
        Node::HostApiFunctionEndpoint(value)
    }
}

impl From<Definition> for Node {
    fn from(value: Definition) -> Node {
        Node::Definition(value)
    }
}
