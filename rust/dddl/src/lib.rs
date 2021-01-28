use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub mod dddl {
    use super::*;
    pub fn parse(sql: &str, dialect: GenericDialect) -> String{
        let ast = Parser::parse_sql(&dialect, sql).unwrap();
        format!("AST: {:?}", ast)
    }
}

struct Type {
}

struct Column {
    content: String,
    dataType: Type,
}

struct Row {
    columns: Vec<Column>,
    no: u32,
}

pub struct Table {
    rows: Vec<Row>,
}


pub mod generator {
    use super::*;
    pub fn generate(dialect: &GenericDialect, ddl: &str) -> Table {
        Table{
            rows: vec!(Row{
                columns: vec!(Column{
                    content: "".into(),
                    dataType: Type{}
                }),
                no: 0,
            })
        }
    }
}
