import {
  tokenize,
  Token,
  TokenSet,
  tokenUtil,
  Whitespace,
  Word,
  DelimitedIdent,
  Keyword,
  COLON,
  SEMICOLON,
  LPAREN,
  RPAREN,
  COMMA,
} from './tokenizer';

interface Ast {}
interface Statement extends Ast {}
class CreateTableStatement implements Statement {
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
  ) { }
}
class ObjectName {
  constructor(
    public value: Ident[]
  ) {}
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
interface TableConstraint {}
class Unique implements TableConstraint {
  constructor(
    public name: Ident|undefined,
    public columns: Ident[],
    public isPrimary: boolean, // Whether this is a `PRIMARY KEY` or just a `UNIQUE` constraint
  ) {}
}
/// A referential integrity constraint (`[ CONSTRAINT <name> ] FOREIGN KEY (<columns>)
/// REFERENCES <foreign_table> (<referred_columns>)`)
class ForeignKey implements TableConstraint {
  constructor(
    public name: Ident|undefined,
    public columns: Ident[],
    public foreignTable: ObjectName,
    public referredColumns: Ident[],
  ) {}
}
/// `[ CONSTRAINT <name> ] CHECK (<expr>)`
class Check implements TableConstraint {
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
  constructor(
    public value: string,
    public quoteStyle?: string,
  ) {}
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

type ResultContent = ObjectName|Ident|Ident[]|[ColumnDef[],TableConstraint[]]|Unique;
interface ParseResult {
  idx: number // for NotFound, idx should be the value before try parse
  content?: ResultContent
}
class NotFound implements ParseResult {
  constructor(
    public idx: number,
  ) {}
}
const N = new NotFound(0);
const notFound = (idx: number) => { // reuse object
  N.idx = idx;
  return N;
};
class Found implements ParseResult {
  constructor(
    public idx: number,
    public content?: ResultContent,
  ) {}
}
const F = new Found(0);
const found = (idx: number, content?: ResultContent) => { // reuse object
  F.idx = idx;
  F.content = content;
  return F;
};
class Eof implements ParseResult {
  idx: number // necessary to implement ParseResult
}

export const parse = (src: string) => {
  const tokenSet = tokenize(src);
  const statements: Statement[] = [];
  let expectingStatementDelimiter = false;
  let result: ParseResult = found(0);
  for(;;) {
    for(;;) { // consume semicolons
      result = consumeToken(tokenSet, result.idx, SEMICOLON);
      if (result instanceof NotFound || result instanceof Eof) break;
      expectingStatementDelimiter=false;
    }
    if(result instanceof Eof) break;
    if(expectingStatementDelimiter) throw new Error();
    const stmt = parseCreateStatement(tokenSet, result.idx);
    statements.push(stmt);
  }
};
const parseCreateStatement = (tokenSet: TokenSet, start: number): CreateTableStatement => {
  // let i = nextMeaningfulTokenIdx(tokenSet, start); // TODO next?
  let result: ParseResult;
  if( (result = parseKeyword(tokenSet, start, 'CREATE')) instanceof NotFound) {
    throw expected('a create statement', tokenSet[result.idx]);
  }
  result = parseKeywords(tokenSet, result.idx, ['OR', 'REPLACE']);
  const orReplace = result instanceof NotFound;
  return parseCreateTableStatement(tokenSet, result.idx, orReplace);
};
const parseCreateTableStatement = (tokenSet: TokenSet, start: number, orReplace: boolean): CreateTableStatement => {
  let result: ParseResult = parseKeywords(tokenSet, start, ['IF','NOT','EXISTS']);
  const ifNotExists = result instanceof Found;
  result = parseObjectName(tokenSet, result.idx);
  result = parseColumns(tokenSet, result.idx);
  return {} as CreateTableStatement;
  // return new CreateTableStatement(
  //   orReplace,
  //   false,
  //   ifNotExists,
  // );
};
const parseColumns = (tokenSet: TokenSet, start: number): ParseResult => {
  let result: ParseResult = found(start);
  const columns: ColumnDef[] = [];
  const constraints: TableConstraint[] = [];
  if ( (result = consumeToken(tokenSet, result.idx, LPAREN)) instanceof NotFound ||
       (result = consumeToken(tokenSet, result.idx, RPAREN)) instanceof Found ) {
    return found(result.idx, [columns, constraints]);
  }
  result = parseTableConstraint(tokenSet, result.idx);
}
const parseTableConstraint = (tokenSet: TokenSet, start: number): ParseResult => {
  let result = parseKeyword(tokenSet, start, 'CONSTRAINT');
  const name = result instanceof Found ? result.content as Ident : undefined;
  result = nextMeaningfulTokenIdx(tokenSet, result.idx);
  const token = tokenSet[result.idx];
  if (inKeywords(token, ['PRIMARY', 'UNIQUE'])) {
    const isPrimary = equalToKeyword(token, 'PRIMARY');
    if (isPrimary) {
      result = expectKeyword(tokenSet, result.idx, 'KEY');
    }
    result = parseParenthesizedColumnList(tokenSet, start, false);
    result.content = new Unique(name, result.content as Ident[], isPrimary);
    return result;
  } else if (equalToKeyword(token, 'FOREIGN')) {
    result = expectKeyword(tokenSet, result.idx, 'KEY');
    result = parseParenthesizedColumnList(tokenSet, start, false);
    const columns = result.content as Ident[];
    result = expectKeyword(tokenSet, result.idx, 'REFERENCES');
    result = parseObjectName(tokenSet, result.idx);
    const foreignTable = result.content as ObjectName;
    result = parseParenthesizedColumnList(tokenSet, start, false);
    const referredColumns = result.content as Ident[];
    result.content = new ForeignKey(name, columns, foreignTable, referredColumns));
  } else {
    
    throw expected('PRIMARY, UNIQUE, FOREIGN, or CHECK', tokenSet[result.idx]);
  }

}
const parseParenthesizedColumnList = (tokenSet: TokenSet, start: number, isOptional: boolean): ParseResult => {
  let result = consumeToken(tokenSet, start, LPAREN);
  if (!(result instanceof NotFound)) {
    result = parseCommaSeparated(tokenSet, result.idx, parseIdentifier);
    result = expectToken(tokenSet, result.idx, RPAREN);
    return result;
  } else if (isOptional) {
    result.content = [];
    return result;
  } else {
    throw expected('a list of columns in parentheses', tokenSet[result.idx]);
  }
};
const parseCommaSeparated = (tokenSet: TokenSet, start: number, callback: (TokenSet, number) => Found): ParseResult => {
  const values: ResultContent[] = [];
  let result: ParseResult = notFound(start);
  for(;;) {
    result = callback(tokenSet, result.idx);
    values.push(result.content!); // TODO bang
    result = consumeToken(tokenSet, result.idx, COMMA);
    if (result instanceof NotFound) break;
  }
  return result;
}
const parseObjectName = (tokenSet: TokenSet, start: number): ParseResult => {
  let result: Found = found(start);
  const idents: Ident[] = [];
  for(;;) {
    result = parseIdentifier(tokenSet, result.idx);
    idents.push(result.content as Ident);
    result = nextMeaningfulTokenIdx(tokenSet, result.idx); // TODO unnecessary?
    result = consumeToken(tokenSet, result.idx, COLON);
    if(!(result instanceof NotFound)) result = nextMeaningfulTokenIdx(tokenSet, result.idx);
    else break;
  }
  result.content = new ObjectName(idents);
  return result;
};
const parseIdentifier = (tokenSet: TokenSet, start: number): Found => {
  const result = nextMeaningfulTokenIdx(tokenSet, start);
  const token = tokenSet[result.idx];
  if (!(result instanceof Found)) {
    throw expected('identifier', 'EOF');
  } else if(token instanceof DelimitedIdent) {
    result.content = new Ident(token.content, token.delimiter);
  } else if(token instanceof Word) { // TODO exclude keyword
    result.content = new Ident(token.value);
  } else {
    throw expected('identifier', token);
  }
  return result;
};
const expectKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): ParseResult => {
  if (equalToKeyword(tokenSet[start], keyword)) return nextMeaningfulTokenIdx(tokenSet, start);
  throw expected(keyword, tokenSet[start]);
};
const parseKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): ParseResult => {
  return equalToKeyword(tokenSet[start], keyword) ? nextMeaningfulTokenIdx(tokenSet, start) : notFound(start);
};
const parseKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): ParseResult => {
  let result: ParseResult = notFound(start);
  for (let j=0; result.idx<tokenSet.length && j<keywords.length; j++) {
    if((result = parseKeyword(tokenSet, start, keywords[j])) instanceof NotFound) break;
  }
  return result;
};
const expectToken = (tokenSet: TokenSet, start: number, token: Token): ParseResult => {
  if (tokenSet[start] === token) return nextMeaningfulTokenIdx(tokenSet, start);
  throw expected(token.value, tokenSet[start]);
};
const consumeToken = (tokenSet: TokenSet, start: number, consumedToken: Token): ParseResult => {
  let result: ParseResult = notFound(start);
  if(tokenSet[result.idx].value === consumedToken.value) result = nextMeaningfulTokenIdx(tokenSet, result.idx);
  return result;
};
const nextMeaningfulTokenIdx = (tokenSet: TokenSet, start: number): ParseResult => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof Whitespace) i++;
  return i>=tokenSet.length ? new Eof : found(i);
};
const equalToKeyword = tokenUtil.equalToKeyword;
const inKeywords = (token: Token, keywords: Keyword[]): boolean => keywords.some(keyword => tokenUtil.equalToKeyword(token, keyword));
const expected = (expected: string, actual: Token|string): Error => new Error();

