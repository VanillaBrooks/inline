
fn main() {
    println!("Hello, world!");


    equation!{asd}
}

type f = f64;

enum Operator{
    Add,
    Sub,
    Mul,
    Div
}

impl Operator{
    fn eval(&self, lhs: f, rhs: &f)-> f {
        let rhs = *rhs;

        match self {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs
        }
    }
}

enum Funct{
    Sin,
    Cos,
    Tan,
    Constant(f)
}
impl Funct{
    pub(crate) fn eval(&self, value: f) -> f {
        match self{
            Sin => value.sin(),
            Cos => value.cos(),
            Tan => value.tan(),
            Funct::Constant(c)=> *c
        }
    }
}


struct Unit <'a>{
    pub varibale: &'a str,
    lhs: Funct,
    op: Option<Operator>,
    rhs: Option<Box<  Unit<'a>  >  >
}

impl <'a> Unit <'a> {
    fn new(var: &'a str, lhs: Funct, op: Option<Operator>, rhs: Option<Box<Unit<'a>>>) -> Self {

        if op.is_some() && rhs.is_none(){ panic!{"operator but no RHS"}}
        Self{
            varibale: var,
            lhs: lhs,
            op: op,
            rhs: rhs
        }
    }
    
    fn eval(&self, value: f) -> f {
        
        if let Some(operator) = &self.op {
            operator.eval(self.lhs.eval(value), &self.rhs.as_ref().unwrap().eval(value) )
        }
        else{
            self.lhs.eval(value)
        }
        
    }
}