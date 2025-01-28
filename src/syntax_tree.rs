use crate::expression::Expression;
use crate::parser::ProgramData;

pub struct SyntaxNode{
    syntax_node_variant: SyntaxNodeVariant,
    depth: i32
}

impl SyntaxNode{
    pub fn new(syntax_node_variant: SyntaxNodeVariant, depth: i32) -> SyntaxNode{
        SyntaxNode{syntax_node_variant, depth}
    }
}

pub enum SyntaxNodeVariant{
    Block{

        instructions: Vec<SyntaxNode>,
    },
    Declaration {name: String, equals_to: Expression},
    Expression(Expression),
    Yap(Expression),
}

impl SyntaxNode{
    pub fn eval(self, program_data: &mut ProgramData){
        match self.syntax_node_variant {
            SyntaxNodeVariant::Declaration {name,  equals_to}  =>{
                let gotten_with_depth = program_data.get_variable_with_depth(name.as_str(), self.depth);

                if gotten_with_depth.is_some() && gotten_with_depth.as_ref().unwrap().1 == self.depth{
                    panic!("Variable {} already exists in depth {}!", name,self.depth);
                } else{
                    let evaluated =equals_to.eval(program_data);
                    program_data.set_variable(name.as_str(),self.depth, evaluated);
                }

            },

            SyntaxNodeVariant::Yap(expression)=> {
                println!("{}",expression.eval(program_data))
            }
            SyntaxNodeVariant::Block {   instructions} => {
                for i in instructions{
                    i.eval(program_data);
                }
                program_data.erase_depth(self.depth);
            }
            _ => {}
        }
    }
}