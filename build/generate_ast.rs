use std::{ io::Write, fs::File, io };

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &String) -> io::Result<()> {
    define_ast(
        output_dir,
        &"Expr".to_string(),
        vec!["error_handler::ViskumError", "token::Token", "token::Literal"],
        vec![
            "Binary   : left: Box<Expr>, operator: Token, right: Box<Expr>",
            "Grouping : expression: Box<Expr>",
            "Literal  : value: Option<Literal>",
            "Prefix   : operator: Token, right: Box<Expr>",
            "Postfix  : left: Box<Expr>, operator: Token",
            "Ternary  : condition: Box<Expr>, true_expr: Box<Expr>, false_expr: Box<Expr>"
        ]
    )?;

    define_ast(
        output_dir,
        &"Stmt".to_string(),
        vec!["error_handler::ViskumError", "token::Token", "token::Literal", "expr::Expr"],
        vec!["Expression : expression: Box<Expr>", "Print      : expression: Box<Expr>"]
    )?;

    Ok(())
}

fn define_ast(
    output_dir: &String,
    base_name: &String,
    crates: Vec<&str>,
    types: Vec<&str>
) -> io::Result<()> {
    let base_name_lowercase = base_name.to_lowercase();
    let path: String = format!("{}/{}.rs", output_dir, base_name_lowercase);
    let mut file = File::create(path)?;
    let mut tree_types: Vec<TreeType> = Vec::new();
    let return_type = "Result<T, ViskumError>";

    for crate_ in crates {
        writeln!(file, "use crate::{};", crate_)?;
    }

    writeln!(file)?;

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split = args.trim().split(", ");
        let mut fields = Vec::new();
        for arg in arg_split {
            fields.push(arg.to_string());
        }

        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    writeln!(file, "#[derive(Debug)]")?;
    writeln!(file, "pub enum {base_name} {{")?;
    for tree_type in &tree_types {
        let base_class_name = &tree_type.base_class_name;
        let class_name = &tree_type.class_name;
        writeln!(file, "    {base_class_name}({class_name}),")?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "impl {base_name} {{")?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, {base_name_lowercase}_visitor: &dyn {base_name}Visitor<T>) -> {return_type} {{"
    )?;
    writeln!(file, "        match self {{")?;
    for tree_type in &tree_types {
        let base_class_name = &tree_type.base_class_name;
        writeln!(
            file,
            "            {base_name}::{base_class_name}({base_name_lowercase}) => {base_name_lowercase}.accept({base_name_lowercase}_visitor),"
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    for tt in &tree_types {
        // let base_class_name = &tree_type.base_class_name;
        let class_name = &tt.class_name;
        writeln!(file, "#[derive(Debug)]")?;
        writeln!(file, "pub struct {class_name} {{")?;
        for field in &tt.fields {
            writeln!(file, "    pub {field},")?;
        }
        writeln!(file, "}}")?;
    }

    writeln!(file, "pub trait {base_name}Visitor<T> {{")?;

    for tt in &tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, {base_name_lowercase}: &{}) -> {return_type};",
            tt.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            tt.class_name
        )?;
    }

    writeln!(file, "}}\n")?;

    for tt in &tree_types {
        writeln!(file, "impl {} {{", tt.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {base_name}Visitor<T>) -> {return_type} {{"
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            tt.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
    }

    Ok(())
}
