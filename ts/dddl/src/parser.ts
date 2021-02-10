import {
  tokenize,
  Token,
  TokenSet,
  tokenUtil,
  Whitespace,
  Word,
  DelimitedIdent,
  Num,
  Keyword,
  COLON,
  SEMICOLON,
  LPAREN,
  RPAREN,
  COMMA,
  Operator,
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
interface Expr {}
class BinaryOp implements Expr {
  constructor(
    public left: Expr,
    public op: BinaryOperator,
    public right: Expr,
  ) {}
}
class IsNull implements Expr {
  constructor(
    public expr: Expr,
  ) {}
}
class IsNotNull implements Expr {
  constructor(
    public expr: Expr,
  ) {}
}
class InList implements Expr {
  constructor(
    public expr: Expr,
    public list: Expr[],
    public negated: boolean,
  ) {}
}
class Between implements Expr {
  constructor(
    public expr: Expr,
    public negated: boolean,
    public low: Expr,
    public high: Expr,
  ) {}
}
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
const DATA_TYPE_NAMES = [
  'CHAR',
  'VARCHAR',
  'UUID',
  'CLOB',
  'BINARY',
  'VARBINARY',
  'BLOB',
  'DECIMAL',
  'NUMBER',
  'FLOAT',
  'SMALLINT',
  'INT',
  'BIGINT',
  'REAL',
  'DOUBLE',
  'BOOLEAN',
  'DATE',
  'TIME',
  'TIMESTAMPE',
  'INTERVAL',
// 'REGCLASS', // not supported
  'TEXT',
  'BITEA',
// 'CUSTOM', // not supported
// 'ARRAY', // not supported
] as const;
type DataTypeName = typeof DATA_TYPE_NAMES[number];
class DataType {
  constructor(
    public name: DataTypeName
  ) {}
}
class Char extends DataType { // Fixed-length character type e.g. CHAR(10)
  constructor() { super('CHAR'); }
}
class Varchar extends DataType { // Variable-length character type e.g. VARCHAR(10)
  constructor(public length?: number) { super('VARCHAR'); }
}
class Uuid extends DataType { // Uuid type
  constructor() { super('UUID'); }
}
class Clob extends DataType { // Large character object e.g. CLOB(1000)
  constructor(public length: number) { super('CLOB'); }
}
class Binary extends DataType { // Fixed-length binary type e.g. BINARY(10)
  constructor(public length: number) { super('BINARY'); }
}
class Varbinary extends DataType { // Variable-length binary type e.g. VARBINARY(10)
  constructor(public length: number) { super('VARBINARY'); }
}
class Blob extends DataType { // Large binary object e.g. BLOB(1000)
  constructor(public length: number) { super('BINARY'); }
}
class Decimal extends DataType { // Decimal type with optional precision and scale e.g. DECIMAL(10,2)
  constructor( public precision: number, public scale: number) { super('DECIMAL'); }
}
class Float extends DataType { // Floating point with optional precision e.g. FLOAT(8)
  constructor( public precision: number) { super('FLOAT'); }
}
class SmallInt extends DataType { // Small integer
  constructor() { super('SMALLINT'); }
}
class Int extends DataType { // Integer
  constructor() { super('INT'); }
}
class BigInt extends DataType { // Big integer
  constructor() { super('BIGINT'); }
}
class Real extends DataType { // Floating point e.g. REAL
  constructor() { super('BIGINT'); }
}
class Double extends DataType { // Double e.g. DOUBLE PRECISION
  constructor() { super('DOUBLE'); }
}
class Boolean extends DataType { // Boolean
  constructor() { super('BOOLEAN'); }
}
class Date extends DataType { // Date
  constructor() { super('DATE'); }
}
class Time extends DataType { // Time
  constructor() { super('DATE'); }
}
class Timestamp extends DataType { // Timestamp
  constructor() { super('DATE'); }
}
class Interval extends DataType { // Interval
  constructor() { super('DATE'); }
}
class Text extends DataType { // Text
  constructor() { super('DATE'); }
}
class Bytea extends DataType { // Bytea
  constructor() { super('DATE'); }
}

class BinaryOperator {
  constructor(
    public value: string, // TODO literal type
  ) {}
}

interface ParseResult<T=void,N=number> {
  idx: N // for NotFound, idx should be the value before try parse
  content?: T
}
class NotFound implements ParseResult {
  constructor(
    public idx: number,
  ) {}
}
const N = new NotFound(0);
const notFound = (idx: number) => { // reuse object. not thread safe!
  N.idx = idx;
  return N;
};
class Found<T=void> implements ParseResult<T> {
  constructor(
    public idx: number,
    public content?: T,
  ) {}
}
const F = new Found<any>(0); // TODO any
function found<T>(idx: number, content?: T): Found<T> { // reuse object. not thread safe!
  F.idx = idx;
  F.content = content;
  return F;
}
class Eof extends Token {} // TODO don't want to extend Token
const EOF = new Eof('EOF');

export const parse = (src: string) => {
  const tokenSet = tokenize(src);
  const statements: Statement[] = [];
  let expectingStatementDelimiter = false;
  let idx: number = 0;
  for(;;) {
    try {
      for(;;) { // ignore empty statements (between consecutive semicolons)
        const optIdx = consumeToken(tokenSet, idx, SEMICOLON); // TODO bang
        if (!optIdx) break;
        idx = optIdx;
        expectingStatementDelimiter=false;
      }
    } catch(eof) {
      if(eof instanceof Eof) break;
    }
    if(expectingStatementDelimiter) throw new Error();
    const stmt = parseCreateStatement(tokenSet, idx);
    statements.push(stmt);
  }
};
const parseCreateStatement = (tokenSet: TokenSet, start: number): CreateTableStatement => {
  // let i = nextMeaningfulToken(tokenSet, start); // TODO next?
  let result: NotFound|Found<Token> = parseKeyword(tokenSet, start, 'CREATE');
  if(result instanceof NotFound) {
    throw getError('a create statement', peekToken(tokenSet, result.idx));
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
type ColumnsAndConstraints = [ColumnDef[], TableConstraint[]];
const parseColumns = (tokenSet: TokenSet, start: number): ParseResult<ColumnsAndConstraints> => {
  let result: ParseResult<void|ColumnDef|TableConstraint|ColumnsAndConstraints> = found(start); // TODO should add Token to generic?
  const columns: ColumnDef[] = [];
  const constraints: TableConstraint[] = [];
  let idx: number|undefined = start;
  if (!(consumeToken(tokenSet, start, LPAREN)) ||
       (idx = consumeToken(tokenSet, start, RPAREN)) ) {
    return found(idx, [columns, constraints]);
  }
  result = parseTableConstraint(tokenSet, start);
}
const parseTableConstraint = (tokenSet: TokenSet, start: number): ParseResult<TableConstraint> => {
  let result: ParseResult<void|Ident|Ident[]|ObjectName|Unique|ForeignKey|Expr> = parseKeyword(tokenSet, start, 'CONSTRAINT');
  const name = result instanceof Found ? result.content as Ident : undefined;
  result = nextMeaningfulToken(tokenSet, result.idx);
  const token = result.content as Token;
  if (inKeywords(token, ['PRIMARY', 'UNIQUE'])) {
    const isPrimary = equalToKeyword(token, 'PRIMARY');
    if (isPrimary) {
      result = expectKeyword(tokenSet, result.idx, 'KEY');
    }
    result = parseParenthesizedColumnList(tokenSet, result.idx, false);
    result.content = new Unique(name, result.content as Ident[], isPrimary);
    return result as Found<Unique>;
  } else if (equalToKeyword(token, 'FOREIGN')) {
    result = expectKeyword(tokenSet, result.idx, 'KEY');
    const { content: columns } = result = parseParenthesizedColumnList(tokenSet, result.idx, false);
    result = expectKeyword(tokenSet, result.idx, 'REFERENCES');
    const { content: foreignTable } = result = parseObjectName(tokenSet, result.idx);
    const { content: referredColumns } = result = parseParenthesizedColumnList(tokenSet, result.idx, false);
    result.content = new ForeignKey(name, columns!, foreignTable!, referredColumns!);
  } else if (equalToKeyword(token, 'CHECK')) {
    result = expectToken(tokenSet, result.idx, LPAREN);
    result = parseExpr(tokenSet, result.idx);
    result = expectToken(tokenSet, result.idx, RPAREN);
  } else {
    throw getError('PRIMARY, UNIQUE, FOREIGN, or CHECK', peekToken(tokenSet, result.idx));
  }
};
const PRECEDENCE = {
  DEFAULT: 0,
  UNARY_NOT_PREC: 15,
  BETWEEN_PREC: 20,
  PLUS_MINUS_PREC: 30,
} as const;
type Precedence = typeof PRECEDENCE[keyof typeof PRECEDENCE]; // valueof PRECEDENCE
const parseExpr = (tokenSet: TokenSet, start: number, precedence: Precedence = PRECEDENCE.DEFAULT): ParseResult<Expr> => {
};
const parseDataType = (tokenSet: TokenSet, start: number, expr: Expr, precedence: number): Found<Expr> => {
  let result: ParseResult<Token|number>
  const { content } = result = nextMeaningfulToken(tokenSet, start);
  if (content instanceof Word){
  }
};
const parseOptionalPrecision = (tokenSet: TokenSet, start: number): Found<number|undefined> => {
  const idx = consumeToken(tokenSet, start, LPAREN);
  if (idx) {
    const result = parseLiteralUint(tokenSet, idx);
    result.idx = expectToken(tokenSet, result.idx, RPAREN).idx;
    return result;
  }
  return found(start);
};
const parseLiteralUint = (tokenSet: TokenSet, start: number): Found<number> => {
  let result: ParseResult<Token|number>
  const { content } = result = nextMeaningfulToken(tokenSet, start);
  if (result.content instanceof Num) {
    result.content = parseInt(result.content.value);
  } else {
    throw getError('literal int', content!);
  }
  return result as Found<number>;
};

const parseInfix = (tokenSet: TokenSet, start: number, expr: Expr, precedence: number): Found<Expr> => {
  let result: ParseResult<void|Expr> = nextMeaningfulToken(tokenSet, start);
  const token = result.content as Token;
  if (token instanceof Operator /* TODO precise condition  */ || inKeywords(token, ['AND','OR','LIKE'])) {
    const { content: right } = result = parseExpr(tokenSet, result.idx);
    result.content = new BinaryOp(expr, new BinaryOperator(token.value), right!);
    return result as Found<Expr>;
  } else {
    result = parseKeywords(tokenSet, result.idx, ['NOT', 'LIKE']);
    if(!(result instanceof NotFound)) {
      const { content: right } = result = parseExpr(tokenSet, result.idx);
      result.content = new BinaryOp(expr, new BinaryOperator('NOT LIKE'), right!);
      return result as Found<Expr>;
    }
  }
  if (equalToKeyword(token, 'IS')) {
    result = parseKeyword(tokenSet, result.idx, 'NULL');
    if (result instanceof Found) {
      result.content = new IsNull(expr);
      return result as Found<Expr>;
    } else {
      result = parseKeywords(tokenSet, result.idx, ['NOT', 'NULL']);
      if (result instanceof Found) {
        result.content = new IsNotNull(expr);
        return result as Found<Expr>;
      }
    }
  } else if (inKeywords(token, ['NOT','IN','BETWEEN'])) {
    const negated = equalToKeyword(token, 'NOT');
    if ((result = parseKeyword(tokenSet, result.idx, 'IN')) instanceof Found) {
      return parseIn(tokenSet, result.idx, expr, negated);
    } else if ((result = parseKeyword(tokenSet, result.idx, 'BETWEEN')) instanceof Found) {
      return parseBetween(tokenSet, result.idx, expr, negated);
    } else {
      throw getError('IN or BETWEEN after NOT', peekToken(tokenSet, result.idx));
    }
  }
  throw new Error('No infix parser for token '+token); // // Can only happen if `getNextPrecedence` got out of sync with this function
};
const parseBetween = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): Found<Between> => {
  let result: ParseResult<Expr|Between>;
  const { content: low } = result = parseExpr(tokenSet, start, PRECEDENCE.BETWEEN_PREC);
  result = expectKeyword(tokenSet, result.idx, 'AND');
  const { content: high } = result = parseExpr(tokenSet, start, PRECEDENCE.BETWEEN_PREC);
  result.content = new Between(expr, negated, low!, high!);
  return result as Found<Between>;
};
const parseIn = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): Found<InList> => {
  let result: ParseResult<Token|Expr[]|InList>;
  result = expectToken(tokenSet, start, LPAREN);
  if ((result = parseKeywords(tokenSet, result.idx, ['SELECT','WITH'])) instanceof Found) { // subquery is not supported
    throw getError('unsupported keyword (SELECT, WITH) after IN', result.content as Token);
  }
  result = parseCommaSeparated(tokenSet, start, parseExpr);
  result.content = new InList(expr, result.content as Expr[], negated);
  result.idx = expectToken(tokenSet, result.idx, RPAREN).idx;
  return result as Found<InList>;
};
const parseParenthesizedColumnList = (tokenSet: TokenSet, start: number, isOptional: boolean): ParseResult<Ident[]> => {
  const idx = consumeToken(tokenSet, start, LPAREN);
  let result: ParseResult<void|Token|Ident[]>;
  if (idx) {
    result = parseCommaSeparated(tokenSet, idx, parseIdentifier);
    result = expectToken(tokenSet, result.idx, RPAREN);
  } else if (isOptional) {
    result = found(start, [] as Ident[]);
  } else {
    throw getError('a list of columns in parentheses', peekToken(tokenSet, start));
  }
  return result as ParseResult<Ident[]>;
};
function parseCommaSeparated<T>(tokenSet: TokenSet, start: number, callback: (tokenSet: TokenSet, idx: number) => Found<T>): ParseResult<T[]> {
  const values: T[] = [];
  let result: ParseResult<void|Token|T|T[]> = notFound(start);
  for(;;) {
    result = callback(tokenSet, result.idx);
    values.push(result.content as T);
    const idx = consumeToken(tokenSet, result.idx, COMMA);
    if(!idx) break;
  }
  result.content = values;
  return result as ParseResult<T[]>;
}
const parseObjectName = (tokenSet: TokenSet, start: number): Found<ObjectName> => {
  let result: Found<void|Ident|ObjectName> = found(start);
  const idents: Ident[] = [];
  for(;;) {
    result = parseIdentifier(tokenSet, result.idx);
    idents.push(result.content as Ident);
    result = nextMeaningfulToken(tokenSet, result.idx); // TODO unnecessary?
    const idx = consumeToken(tokenSet, result.idx, COLON);
    if(idx) result = nextMeaningfulToken(tokenSet, idx);
    else break;
  }
  result!.content = new ObjectName(idents); // TODO why needs bang?
  return result as Found<ObjectName>;
};
const parseIdentifier = (tokenSet: TokenSet, start: number): Found<Ident> => {
  let result: ParseResult<void|Ident>;
  try {
    result = nextMeaningfulToken(tokenSet, start);
  } catch(cause) {
    if(cause instanceof Eof) throw getError('identifier', 'EOF');
    else throw cause;
  }
  const token = result.content as Token;
  if(token instanceof DelimitedIdent) {
    result.content = new Ident(token.content, token.delimiter);
  } else if(token instanceof Word) { // TODO exclude keyword
    result.content = new Ident(token.value);
  } else {
    throw getError('identifier', token);
  }
  return result as ParseResult<Ident>;
};
const expectKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): Found<Token> => {
  const token = peekToken(tokenSet, start);
  if (equalToKeyword(token, keyword)) return nextMeaningfulToken(tokenSet, start);
  throw getError(keyword, token);
};
const parseKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): NotFound|Found<Token> => {
  const token = peekToken(tokenSet, start);
  return equalToKeyword(token, keyword) ? nextMeaningfulToken(tokenSet, start) : notFound(start);
};
const parseKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): NotFound|Found<Token> => {
  let result: ParseResult<void|Token> = notFound(start);
  for (let j=0; result.idx<tokenSet.length && j<keywords.length; j++) {
    if((result = parseKeyword(tokenSet, result.idx, keywords[j])) instanceof NotFound) break;
  }
  return result;
};
const expectToken = (tokenSet: TokenSet, start: number, expected: Token): Found<Token> => {
  const actual = peekToken(tokenSet, start);
  if (actual.value === expected.value) return nextMeaningfulToken(tokenSet, start);
  throw getError(expected.value, actual.value);
};
const consumeToken = (tokenSet: TokenSet, start: number, consumedToken: Token): number|undefined => {
  const token = peekToken(tokenSet, start);
  if(token.value === consumedToken.value) return nextMeaningfulToken(tokenSet, start).idx;
};
const nextMeaningfulToken = (tokenSet: TokenSet, start: number): Found<Token> => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof Whitespace) i++;
  if (tokenSet.length<=i) throw EOF;
  return new Found<Token>(i+1, tokenSet[i]);
};
const peekToken = (tokenSet: TokenSet, start: number): Token|Eof => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof Whitespace) i++;
  return tokenSet.length<=i ? EOF : tokenSet[i];
};
const equalToKeyword = tokenUtil.equalToKeyword;
const inKeywords = (token: Token, keywords: Keyword[]): boolean => keywords.some(keyword => tokenUtil.equalToKeyword(token, keyword));
const getError = (expected: string, actual: Token|string): Error => new Error();

