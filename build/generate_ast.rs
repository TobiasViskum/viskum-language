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
        vec![
            "Binary   : left: Expr, operator: Token, right: Expr".to_string(),
            "Grouping : expression: Expr".to_string(),
            "Literal  : value: Literal".to_string(),
            "Unary    : operator: Token, right: Expr".to_string()
        ]
    )?;

    Ok(())
}

fn define_ast(output_dir: &String, base_name: &String, types: Vec<String>) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types: Vec<TreeType> = Vec::new();

    writeln!(file, "use crate::error_handler::ViskumError;")?;
    writeln!(file, "use crate::token::Token;")?;
    writeln!(file, "use crate::token::Literal;\n")?;

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

    writeln!(file, "pub enum {base_name} {{")?;
    for tree_type in &tree_types {
        let base_class_name = &tree_type.base_class_name;
        let class_name = &tree_type.class_name;
        writeln!(file, "    {base_class_name}({class_name}),")?;
    }
    writeln!(file, "}}")?;

    for tt in &tree_types {
        // let base_class_name = &tree_type.base_class_name;
        let class_name = &tt.class_name;

        writeln!(file, "pub struct {class_name} {{")?;
        for field in &tt.fields {
            let field_type = field.replace("Expr", "Box<Expr>");
            writeln!(file, "    {field_type},")?;
        }
        writeln!(file, "}}")?;
    }

    writeln!(file, "pub trait ExprVisitor<T> {{")?;

    for tt in &tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, expr: &{});",
            tt.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            tt.class_name
        )?;
    }

    writeln!(file, "}}\n")?;

    for tt in &tree_types {
        writeln!(file, "impl {} {{", tt.class_name)?;
        writeln!(file, "    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) {{")?;
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
