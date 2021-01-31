use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::DataType;

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
create_function!(foo);
create_function!(bar);

pub mod dddl {
    use super::*;
    pub fn parse(sql: &str, dialect: GenericDialect) -> String{
        foo();
        let ast = Parser::parse_sql(&dialect, sql).unwrap();
        format!("AST: {:?}", ast)
    }
}
pub struct Column {
    pub content: String,
    pub col_no: u32,
}
impl Column {
    fn to_csv_string(&self, delimiter: &str) -> String {
        let mut csv = String::from(self.content.as_str());
        csv // TODO escape
    }
}
pub struct ColumnDefinition {
    pub data_type: DataType,
    pub length: u32,
    pub constraints: Vec<Constraints>,
    pub field_name: String,
    pub col_no: u32,
}
struct Condition {
    line_no: u32,
    pre_content: String,
    data_size: u32,
}
fn gen_column(col_def: &ColumnDefinition, condition: &Condition) -> String {
    match col_def.data_type {
        DataType::Boolean       => gen_bool(col_def, condition),
        DataType::Int           => gen_int(col_def, condition),
        DataType::SmallInt      => gen_small_int(col_def, condition),
        DataType::BigInt        => gen_big_int(col_def, condition),
        DataType::Float(p)      => gen_float(col_def, condition, &p.unwrap_or(64)),
        DataType::Double        => gen_float(col_def, condition, &64u64),
        DataType::Decimal(p,_)  => gen_float(col_def, condition, &p.unwrap_or(64)),
        DataType::Date          => gen_date(col_def, condition),
        DataType::Time          => gen_time(col_def, condition),
        DataType::Timestamp     => gen_timestamp(col_def, condition),
        DataType::Char(n)       => gen_char(col_def, condition, &n.unwrap_or(1)),
        _ => "".into(),
    }
}
fn gen_bool(col_def: &ColumnDefinition, condition: &Condition) -> String {
    let content = !condition.pre_content.eq("true");
    "\"".to_string() + &content.to_string() + "\""
}
fn gen_int(col_def: &ColumnDefinition, condition: &Condition) -> String {
    let mut content: i64 = condition.pre_content.parse::<i64>().expect(&format!("Unable parse to int: {}", condition.pre_content)) + 1;
    if content > i32::MAX as i64 {
        content = i32::MIN as i64;
    }
    "\"".to_string() + &content.to_string() + "\""
}
fn gen_small_int(col_def: &ColumnDefinition, condition: &Condition) -> String {
    let mut content: i32 = condition.pre_content.parse::<i32>().expect(&format!("Unable parse to small int: {}", condition.pre_content)) + 1;
    if content > i16::MAX as i32 {
        content = i16::MIN as i32;
    }
    "\"".to_string() + &content.to_string() + "\""
}
fn gen_big_int(col_def: &ColumnDefinition, condition: &Condition) -> String {
    let mut content: i128 = condition.pre_content.parse::<i128>().expect(&format!("Unable parse to big int: {}", condition.pre_content)) + 1;
    if content > i64::MAX as i128 {
        content = i64::MIN as i128;
    }
    "\"".to_string() + &content.to_string() + "\""
}
fn gen_float(col_def: &ColumnDefinition, condition: &Condition, p: &u64) -> String {
    let pre_content = &condition.pre_content;
    let content: String = if !pre_content.starts_with('-') {
        match pre_content.rfind(|c| (c != '9' && c != '.')) {
            Some(i) => {
                let (left, right) = (pre_content[..i].to_string(), pre_content[i..].to_string());
                let right = &right.replace(|c| c != '.', "0");
                let left_len = left.len()-1;
                let left = left[..left_len-1].to_string() + &(left[left_len-1..].parse::<u8>().expect("Parse to u8 failed") + 1).to_string();
                left + &right
            }
            None => { // All 9
                if pre_content.len() as u64 <= *p {
                    "1".to_string() + &pre_content.replace(|c| c != '.', "0")
                } else { // if overflow, negate value
                    "-".to_string() + &pre_content
                }
            }
        }
    } else { // negative
        match pre_content.rfind(|c| (c != '0' && c != '.' && c != '-')) {
            Some(i) => {
                let (left, right) = (pre_content[..i].to_string(), pre_content[i..].to_string());
                let right = &right.replace(|c| c != '.', "9");
                let left_len = left.len()-1;
                let left = left[..left_len-1].to_string() + &(left[left_len-1..].parse::<u8>().expect("Parse to u8 failed") - 1).to_string();
                left + &right
            }
            None => { // All 0
                pre_content.replace("-", "")
            }
        }
    };
    "\"".to_string() + &content.to_string() + "\""
}
fn is_leap_year(year: &u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
const MONTH_LIST: [u8; 5] = [2,4,6,9,11];
fn add_date(base_date: &str) -> String {
    let date_vec: Vec<&str> = base_date.split('-').collect();
    let mut year = date_vec[0].parse::<u16>().expect("Parse error");
    let mut month = date_vec[1].parse::<u8>().expect("Parse error");
    let mut date = date_vec[2].parse::<u8>().expect("Parse error");
    if month == 12 && date == 31 {
        year += 1;
        month = 1;
        date = 1;
    } else if (is_leap_year(&year) && month == 2 && date == 29)
        || (!is_leap_year(&year) && month == 2 && date == 28)
        || MONTH_LIST.contains(&month) && date == 30
        || !MONTH_LIST.contains(&month) && date == 31 {
        month += 1;
        date = 1;
    } else {
        date += 1;
    }
    format!("{:04}", year) + "-" + &format!("{:02}", month) + "-" + &format!("{:02}", date)
}
fn gen_date(col_def: &ColumnDefinition, condition: &Condition) -> String {
    let content = add_date(&condition.pre_content);
    "\"".to_string() + &content.to_string() + "\""
}
fn add_sec(base_time: &str) -> String {
    let time_vec: Vec<&str> = base_time.split('.').collect();
    let msec = time_vec[1];
    let time_vec: Vec<&str> = time_vec[0].split(':').collect();
    let mut hour = time_vec[0].parse::<i16>().expect("Parse error");
    let mut minute = time_vec[1].parse::<u8>().expect("Parse error");
    let mut second = time_vec[2].parse::<u8>().expect("Parse error");
    if hour < 0 {
        if minute == 59 && second == 59 {
            hour += 1;
            minute = 0;
            second = 0;
        } else if second == 59 {
            minute += 1;
            second = 0;
        } else {
            second += 1;
        }
    } else { // negative
        if minute == 0 && second == 0 {
            hour += 1;
            minute = 59;
            second = 59;
        } else if second == 0 {
            minute -= 1; // decrement since minute is positive here
            second = 59;
        } else {
            second -= 1; // decrement since second is positive here
        }
    }
    format!("{:02}", hour) + &format!("{:02}", minute) + &format!("{:02}", second) + msec
}
fn gen_time(col_def: &ColumnDefinition, condition: &Condition) -> String {
    let content = add_sec(&condition.pre_content);
    "\"".to_string() + &content.to_string() + "\""
}
fn gen_timestamp(col_def: &ColumnDefinition, condition: &Condition) -> String {
    let content = add_date(&condition.pre_content[..10]) + &condition.pre_content[11..];
    "\"".to_string() + &content.to_string() + "\""
}
fn gen_char(col_def: &ColumnDefinition, condition: &Condition, n: &u64) -> String {
    let mut data = String::from("\"");
    data.push_str("\"");
    data
}
pub struct Row {
    pub columns: Vec<Column>,
    pub line_no: u32,
}
// pub enum QuotesPreference {
//     Always,
//     TypeDependent,
// }
impl Row {
    // pub fn to_csv_string(&self, delimiter: &str, quotes: &QuotesPreference) -> String {
    fn to_csv_string(&self, delimiter: &str) -> String {
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

pub struct Table {
    pub rows: Vec<Row>,
    pub col_defs: Vec<ColumnDefinition>,
}

impl Table {
    pub fn to_csv_string(&self, delimiter: &str, line_separator: &str, has_header: bool) -> String {
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
            let row_csv = r.to_csv_string(delimiter);
            csv.push_str(row_csv.as_str());
            csv.push_str(line_separator);
        }
        csv
    }
    pub fn to_insert_statements() {

    }
}

pub struct Options {
}

pub mod generator {
    use super::*;
    pub fn generate(ddl: &str, options: &Options) -> Table {
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
                length: 0,
                constraints: [].into(),
                field_name: "".into(),
                col_no: 0,
            }),
        }
    }
}
