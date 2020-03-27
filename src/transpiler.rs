use crate::parser::{Node};

const TAB: &str = "    ";

pub fn transpile(nodes: &Vec<Node>) -> String {
    let mut out = String::from("fn main() {\n");
    for n in nodes {
        out += &node(n, 1);
    }
    out += "}\n";
    out
}

fn node(n: &Node, indent: usize) -> String {
    match n {
        Node::Line(vd) => line(&*vd, indent),
        Node::VarDefinition(name, val) => var_definition(&*name, &*val, indent),
        Node::VarName(s) => var_name(s),
        Node::Value(s) => value(s),
    }
}

fn line(n: &Node, indent: usize) -> String {
    node(n, indent)
}

fn var_definition(n1: &Node, n2: &Node, indent: usize) -> String {
    let mut out = String::from(TAB).repeat(indent);
    out += "let ";
    out += &node(n1, indent);
    out += " = ";
    out += &node(n2, indent);
    out += ";\n";
    out
}

fn var_name(s: &String) -> String {
    s.to_string()
}

fn value(s: &String) -> String {
    s.to_string()
}

// テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpile_test() {
        assert_eq!(
            transpile(
                &vec![Node::Line(
                    Box::new(Node::VarDefinition(
                        Box::new(Node::VarName(String::from("name"))),
                        Box::new(Node::Value(String::from("value"))),
                    ))
                )]
            ),
            String::from(
"fn main() {
    let name = value;
}
"
            )
        )
    }

}
