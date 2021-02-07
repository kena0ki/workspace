import {
  tokenize,
  Token,
  TokenSet,
  tokenUtil as tu,
  tokenSetUtil as tsu,
} from './tokenizer';

class Statement {}
class CreateTableStatement extends Statement{
  constructor(
    public or_replace: boolean,
    public external: boolean,
    public ifNotExists: boolean,
    public name: ObjectName, // table name
    public columns: ColumnDef[], // optional schema
    public constraints: TableConstraint[],
    public withOptions: SqlOption[],
    public withoutRowid: boolean,
    public fileFormat?: FileFormat,
    public location?: string,
    // public query?: Query,  // Not supported
  ) { super(); }
}
class ObjectName {
  value: Ident
}
class ColumnDef {
  name: Ident
  dataType: DataType
  collation?: ObjectName
  options: ColumnOptionDef[]
}
class ColumnOptionDef {
  name?: Ident
  option: ColumnOption
}
class ColumnOption {}
class TableConstraint {}
class Unique extends TableConstraint {
    name?: Ident
    columns: Ident[]
    isPrimary: boolean // Whether this is a `PRIMARY KEY` or just a `UNIQUE` constraint
}
/// A referential integrity constraint (`[ CONSTRAINT <name> ] FOREIGN KEY (<columns>)
/// REFERENCES <foreign_table> (<referred_columns>)`)
class ForeignKey extends TableConstraint {
    name?: Ident
    columns: Ident[]
    foreignTable: ObjectName
    referredColumns: Ident[]
}
/// `[ CONSTRAINT <name> ] CHECK (<expr>)`
class Check extends TableConstraint {
    name?: Ident
    expr: Expr
}
class Expr {}
class SqlOption {
  name: Ident
  value: Value
}
class Value {}
class Ident {
  value: string
  quoteStyle: string
}
class FileFormat {}
class DataType {}
class DataTypeWithLength {
  length: number
}
class DataTypeWithOptLength {
  length?: number
}
class Char extends DataTypeWithOptLength {} // Fixed-length character type e.g. CHAR(10)
class Varchar extends DataTypeWithOptLength {} // Variable-length character type e.g. VARCHAR(10)
class Uuid extends DataType {} // Uuid type
class Clob extends DataTypeWithLength {} // Large character object e.g. CLOB(1000)
class Binary extends DataTypeWithLength {} // Fixed-length binary type e.g. BINARY(10)
class Varbinary extends DataTypeWithLength {} // Variable-length binary type e.g. VARBINARY(10)
class Blob extends DataTypeWithLength {} // Large binary object e.g. BLOB(1000)
class Decimal extends DataType { // Decimal type with optional precision and scale e.g. DECIMAL(10,2)
  precision: number
  scale: number
}
class Float extends DataType { // Floating point with optional precision e.g. FLOAT(8)
  precision: number
}
class SmallInt extends DataType {} // Small integer
class Int extends DataType {} // Integer
class BigInt extends DataType {} // Big integer
class Real extends DataType {} // Floating point e.g. REAL
class Double extends DataType {} // Double e.g. DOUBLE PRECISION
class Boolean extends DataType {} // Boolean
class Date extends DataType {} // Date
class Time extends DataType {} // Time
class Timestamp extends DataType {} // Timestamp
class Interval extends DataType {} // Interval
class Regclass extends DataType {} // Regclass used in postgresql serial
class Text extends DataType {} // Text
class Bytea extends DataType {} // Bytea
class Custom extends DataType {} // Custom type such as enums. not supported.
class Array extends DataType {} // Arrays. not supported.

export const parseTable = (src: string) => {
  const tokenSet = tokenize(src);
  const statements: Statement[] = [];
  let expectingStatementDelimiter = false;
  let i=0;
  for(;;) {
    while(i < tokenSet.length && tokenSet[i].value === ';') { // consume semicolons
      i++;
      expectingStatementDelimiter=false;
    }
    if(i>=tokenSet.length) break;
    if(expectingStatementDelimiter) throw new Error();
    const stmt = parseCreateStatement(tokenSet, i);
    statements.push(stmt);
  }
};
const parseCreateStatement = (tokenSet: TokenSet, start: number): Statement => {
  // let i = tsu.nextMeaningfulTokenIdx(tokenSet, start); // TODO next?
  let i=start;
  if(tsu.parseKeyword(tokenSet, i, 'CREATE') < 0) {
    abort('a create statement', tokenSet[i]);
  }
  i = tsu.nextMeaningfulTokenIdx(tokenSet, i);
  const nextIdx = tsu.parseKeywords(tokenSet, i, ['OR', 'REPLACE']);
  if (nextIdx < 0) {
    return parseCreateTableStatement(tokenSet, i, false);
  } else {
    return parseCreateTableStatement(tokenSet, nextIdx, true);
  }
};
const parseCreateTableStatement = (tokenSet: TokenSet, start: number, orReplace: boolean): CreateTableStatement => {
  let i=start;
  let ifNotExists;
  let nextIdx=0;
  if( (nextIdx = tsu.parseKeywords(tokenSet, start, ['IF','NOT','EXISTS'])) >= 0 ){
    ifNotExists = true;
    i=nextIdx;
  }
  return new CreateTableStatement(
    orReplace,
    false,
    ifNotExists,
  );
};
const abort = (expected: string, token: Token) => '';

