use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::DataType;

pub mod dddl {
    use super::*;
    pub fn parse(sql: &str, dialect: GenericDialect) -> String{
        let ast = Parser::parse_sql(&dialect, sql).unwrap();
        format!("AST: {:?}", ast)
    }
}

pub struct Column {
    pub content: String,
    pub col_no: u32,
}
impl Column {
    pub fn to_csv_string(&self, delimiter: &str) -> String {
        let mut csv = String::from(self.content.as_str());
        csv // TODO escape
    }
}

pub struct Row {
    pub columns: Vec<Column>,
    pub line_no: u32,
}
pub enum QuotesPreference {
    Always,
    TypeDependent,
}
impl Row {
    pub fn to_csv_string(&self, delimiter: &str, quotes: &QuotesPreference) -> String {
        let mut csv = String::from("");
        let mut iter = self.columns.iter();
        if let Some(c) = iter.next() {
            csv.push_str("\"");
            let csv_str = c.to_csv_string(delimiter);
            csv.push_str(csv_str.as_str());
            csv.push_str("\"");
            for c in iter {
                csv.push_str(delimiter);
                csv.push_str("\"");
                let csv_str = c.to_csv_string(delimiter);
                csv.push_str(csv_str.as_str());
                csv.push_str("\"");
            }
        }
        csv
    }
}

pub enum Constraints {
    PrimaryKey,
    Unique,
    ForeignKey, // TODO necessary?
    NotNull,
}

pub struct ColumnDefinition {
    pub data_type: DataType,
    pub constraints: Vec<Constraints>,
    pub field_name: String,
    pub col_no: u32,
}

pub struct Table {
    pub rows: Vec<Row>,
    pub col_defs: Vec<ColumnDefinition>,
}

impl Table {
    pub fn to_csv_string(&self, delimiter: &str, quotes: &QuotesPreference, line_separator: &str, has_header: bool) -> String {
        let mut csv = String::from("");
        if has_header {
            let mut iter = self.col_defs.iter();
            let cur_opt = iter.next();
            let mut next_opt = iter.next();
            if let Some(mut cur) = cur_opt {
                csv.push_str("\"");
                while let Some(next) = next_opt {
                    csv.push_str(cur.field_name.as_str());
                    csv.push_str("\"");
                    csv.push_str(delimiter);
                    csv.push_str("\"");
                    cur = next;
                    next_opt = iter.next();
                }
                csv.push_str(cur.field_name.as_str());
                csv.push_str("\"");
            }
        }
        if has_header {
            let mut iter = self.col_defs.iter();
            if let Some(d) = iter.next() {
                csv.push_str("\"");
                let csv_str = &d.field_name;
                csv.push_str(csv_str.as_str());
                csv.push_str("\"");
                for d in iter {
                    csv.push_str(delimiter);
                    csv.push_str("\"");
                    let csv_str = &d.field_name;
                    csv.push_str(csv_str.as_str());
                    csv.push_str("\"");
                }
            }
        }
        for r in &self.rows {
            let row_csv = r.to_csv_string(delimiter, quotes);
            csv.push_str(row_csv.as_str());
            csv.push_str(line_separator);
        }
        csv
    }
    pub fn to_insert_statements() {

    }
}

pub mod generator {
    use super::*;
    pub fn generate(dialect: &GenericDialect, ddl: &str) -> Table {
        Table{
            rows: vec!(Row{
                columns: vec!(Column{
                    content: "".into(),
                    col_no: 0,
                }),
                line_no: 0,
            }),
            col_defs: vec!(ColumnDefinition {
                data_type: DataType::Int,
                constraints: [].into(),
                field_name: "".into(),
                col_no: 0,
            }),
        }
    }
}
