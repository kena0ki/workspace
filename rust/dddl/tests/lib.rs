use dddl::dddl::parse;
use sqlparser::dialect::GenericDialect;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select() {
        const SQL: &str = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";
        let result = parse(SQL, GenericDialect{});
        const EXPECTED: &str = "AST: [Query(Query { with: None, body: Select(Select { distinct: false, top: None, projection: [UnnamedExpr(Identifier(Ident { value: \"a\", quote_style: None })), UnnamedExpr(Identifier(Ident { value: \"b\", quote_style: None })), UnnamedExpr(Value(Number(\"123\"))), UnnamedExpr(Function(Function { name: ObjectName([Ident { value: \"myfunc\", quote_style: None }]), args: [Unnamed(Identifier(Ident { value: \"b\", quote_style: None }))], over: None, distinct: false }))], from: [TableWithJoins { relation: Table { name: ObjectName([Ident { value: \"table_1\", quote_style: None }]), alias: None, args: [], with_hints: [] }, joins: [] }], selection: Some(BinaryOp { left: BinaryOp { left: Identifier(Ident { value: \"a\", quote_style: None }), op: Gt, right: Identifier(Ident { value: \"b\", quote_style: None }) }, op: And, right: BinaryOp { left: Identifier(Ident { value: \"b\", quote_style: None }), op: Lt, right: Value(Number(\"100\")) } }), group_by: [], having: None }), order_by: [OrderByExpr { expr: Identifier(Ident { value: \"a\", quote_style: None }), asc: Some(false), nulls_first: None }, OrderByExpr { expr: Identifier(Ident { value: \"b\", quote_style: None }), asc: None, nulls_first: None }], limit: None, offset: None, fetch: None })]";
        assert_eq!(EXPECTED, result);
    }
    #[test]
    fn minus() {
        const SQL: &str = "select -1 from A";
        let result = parse(SQL, GenericDialect{});
        const EXPECTED: &str = "";
        assert_eq!(EXPECTED, result);
    }
    #[test]
    fn create() {
        const SQL: &str = "create table A ( char char(1) );";
        let result = parse(SQL, GenericDialect{});
        const EXPECTED: &str = "";
        assert_eq!(EXPECTED, result);
    }
}
