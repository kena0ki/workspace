import { token as tk, Token, TokenSet, } from './tokenizer';
import { types, DataType } from './data-types';
import { keywords, Keyword } from './keywords';
import { exprs, Expr, values, Value, ops } from './expressions';

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
  constructor(
    public name: Ident,
    public dataType: DataType,
    public collation: ObjectName|undefined,
    public options: ColumnOptionDef[],
  ) {}
}
class ColumnOptionDef {
  constructor(
    public name: Ident|undefined,
    public option: ColumnOption,
  ) {}
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
// A referential integrity constraint (`[ CONSTRAINT <name> ] FOREIGN KEY (<columns>)
// REFERENCES <foreign_table> (<referred_columns>)`)
class ForeignKey implements TableConstraint {
  constructor(
    public name: Ident|undefined,
    public columns: Ident[],
    public foreignTable: ObjectName,
    public referredColumns: Ident[],
  ) {}
}
// `[ CONSTRAINT <name> ] CHECK (<expr>)`
class Check implements TableConstraint {
  constructor(
    public name: Ident|undefined,
    public expr: Expr,
  ) {}
}
class SqlOption {}
class Ident {
  constructor(
    public value: string,
    public quoteStyle?: string,
  ) {}
}
class FileFormat {}

type ParseResult<T> = [number,T];

class Eof {
  private eof = undefined;
  public value = 'EOF';
}
const EOF = new Eof();

class ParseError extends Error {}

export const parse = (src: string) => {
  const tokenSet = tk.tokenize(src);
  const statements: Statement[] = [];
  let expectingStatementDelimiter = false;
  let idx: number = 0;
  for(;;) {
    for(;;) { // ignore empty statements (between consecutive semicolons)
      const optIdx = consumeToken(tokenSet, idx, tk.SEMICOLON);
      if (!optIdx) break;
      idx = optIdx;
      expectingStatementDelimiter=false;
    }
    if(tokenSet.length <= idx) break; // EOF
    if(expectingStatementDelimiter) throw new Error();
    const stmt = parseCreateStatement(tokenSet, idx);
    statements.push(stmt);
  }
};
const parseCreateStatement = (tokenSet: TokenSet, start: number): CreateTableStatement => {
  // let i = nextMeaningfulToken(tokenSet, start); // TODO next?
  let [idx,found] = parseKeyword(tokenSet, start, 'CREATE');
  if(!found) {
    throw getError('a create statement', peekToken(tokenSet, idx));
  }
  [idx, found] = parseKeywords(tokenSet, idx, ['OR', 'REPLACE']);
  const orReplace = found;
  return parseCreateTableStatement(tokenSet, idx, orReplace);
};
const parseCreateTableStatement = (tokenSet: TokenSet, start: number, orReplace: boolean): CreateTableStatement => {
  let idx;
  const [,ifNotExists] = [idx] = parseKeywords(tokenSet, start, ['IF','NOT','EXISTS']);
  const [,objectName] = [idx] = parseObjectName(tokenSet, idx);
  const [,columns] = [idx] = parseObjectName(tokenSet, idx);
  return {} as CreateTableStatement;
  // return new CreateTableStatement(
  //   orReplace,
  //   false,
  //   ifNotExists,
  // );
};
type ColumnsAndConstraints = [ColumnDef[], TableConstraint[]];
const parseColumns = (tokenSet: TokenSet, start: number): ParseResult<ColumnsAndConstraints> => {
  const columns: ColumnDef[] = [];
  const constraints: TableConstraint[] = [];
  let optIdx: number|undefined = start;
  if (!(consumeToken(tokenSet, optIdx, tk.LPAREN)) || (optIdx = consumeToken(tokenSet, optIdx, tk.RPAREN)) ) { // TODO ??
    return [optIdx, [columns, constraints]];
  }
  const [idx,optConstraints] = parseOptionalTableConstraint(tokenSet, start);
};
const parseOptionalTableConstraint = (tokenSet: TokenSet, start: number): ParseResult<TableConstraint|undefined> => {
  let idx;
  const [,found] = [idx] = parseKeyword(tokenSet, start, 'CONSTRAINT');
  if (!found) return [start, undefined];
  const [,name] = [idx] = parseIdentifier(tokenSet, idx);
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if (token instanceof Token){
    if (inKeywords(token, ['PRIMARY', 'UNIQUE'])) {
      const isPrimary = equalToKeyword(token, 'PRIMARY');
      if (isPrimary) idx = expectKeyword(tokenSet, idx, 'KEY');
      const [,columns] = [idx] = parseParenthesizedColumnList(tokenSet, idx, false);
      return [idx, new Unique(name, columns, isPrimary)];
    } else if (equalToKeyword(token, 'FOREIGN')) {
      idx = expectKeyword(tokenSet, idx, 'KEY');
      const [,columns] = [idx] = parseParenthesizedColumnList(tokenSet, idx, false);
      idx = expectKeyword(tokenSet, idx, 'REFERENCES');
      const [,foreignTable] = [idx] = parseObjectName(tokenSet, idx);
      const [,referredColumns] = [idx] = parseParenthesizedColumnList(tokenSet, idx, false);
      return [idx, new ForeignKey(name, columns, foreignTable, referredColumns)];
    } else if (equalToKeyword(token, 'CHECK')) {
      idx = expectToken(tokenSet, idx, tk.LPAREN);
      const [,expr] = [idx] = parseExpr(tokenSet, idx);
      idx = expectToken(tokenSet, idx, tk.RPAREN);
    }
  }
  throw getError('PRIMARY, UNIQUE, FOREIGN, or CHECK', peekToken(tokenSet, idx));
};
const PRECEDENCE = {
  DEFAULT: 0,
  UNARY_NOT_PREC: 15,
  BETWEEN_PREC: 20,
  PLUS_MINUS_PREC: 30,
} as const;
type Precedence = typeof PRECEDENCE[keyof typeof PRECEDENCE]; // valueof PRECEDENCE
const parseExpr = (tokenSet: TokenSet, start: number, precedence: Precedence = PRECEDENCE.DEFAULT): ParseResult<Expr> => {
  let idx: number;
  const [,dataType] = [idx] = tryParseDataType(tokenSet, start);
  if (dataType instanceof DataType) {
    const [,value] = [idx] = expectLiteralString(tokenSet, start);
    return [idx, new exprs.TypedString(dataType, value)];
  }
  const prevIdx = idx;
  let [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if (token instanceof tk.Word) {
    if (keywords.isOneOfKeywords(token.value, ['TRUE','FALSE','NULL'])) {
      return expectValue(tokenSet, prevIdx);
    } else if (keywords.isKeyword(token.value, 'CASE')) {
      return parseCase(tokenSet, idx);
    } else if (keywords.isKeyword(token.value, 'CAST')) {
      return parseCase(tokenSet, idx);
    } else if (keywords.isKeyword(token.value, 'EXISTS')) {
      throw unsupportedExprssion(peekToken(tokenSet, idx));
    } else if (keywords.isKeyword(token.value, 'EXTRACT')) {
      throw unsupportedExprssion(peekToken(tokenSet, idx));
    } else if (keywords.isKeyword(token.value, 'INTERVAL')) {
      throw unsupportedExprssion(peekToken(tokenSet, idx));
    } else if (keywords.isKeyword(token.value, 'LISTAGG')) {
      throw unsupportedExprssion(peekToken(tokenSet, idx));
    } else if (keywords.isKeyword(token.value, 'NOT')) {
      const [,expr] = [idx] = parseExpr(tokenSet, idx);
      return [idx, new exprs.UnaryOp(new ops.UnaryOperator('NOT'), expr)];
    }
    token = peekToken(tokenSet, idx); // TODO why can't use const
    if (token === tk.LPAREN || token === tk.PERIOD) {
    }
  }
};
const parseCast = (tokenSet: TokenSet, start: number): ParseResult<Expr> => {
  let idx: number = start;
  idx = expectToken(tokenSet, idx, tk.LPAREN);
  const [,expr] = [idx] = parseExpr(tokenSet, idx);
  idx = expectKeyword(tokenSet, idx, 'AS');
  const [,dataType] = [idx] = parseDataType(tokenSet, idx);
  idx = expectToken(tokenSet, idx, tk.RPAREN);
  return [idx, new exprs.Cast(expr,dataType)];
};
const parseCase = (tokenSet: TokenSet, start: number): ParseResult<Expr> => {
  let idx:number=start;
  let [,found] = [idx] = parseKeyword(tokenSet, idx, 'WHEN');
  let operand;
  if (!found) {
    [,operand] = [idx] = parseExpr(tokenSet, idx);
    idx = expectKeyword(tokenSet, idx, 'WHEN');
  }
  const conditions = [];
  const results = [];
  do {
    let [,expr] = [idx] = parseExpr(tokenSet, idx);
    conditions.push(expr);
    idx = expectKeyword(tokenSet, idx, 'WHEN');
    [,expr] = [idx] = parseExpr(tokenSet, idx);
    results.push(expr);
    [,found] = [idx] = parseKeyword(tokenSet, idx, 'WHEN');
  } while(found);
  [,found] = [idx] = parseKeyword(tokenSet, idx, 'ELSE');
  let elseResult;
  if (found) elseResult = parseExpr(tokenSet, idx);
  idx = expectKeyword(tokenSet, idx, 'END');
  return [idx, new exprs.Case(operand, conditions, results, elseResult)];
};
const expectValue = (tokenSet: TokenSet, start: number): ParseResult<Value<boolean|string|undefined>> => {
    const [idx, token] = nextMeaningfulToken(tokenSet, start);
    if (token instanceof tk.Word) {
      if (keywords.isOneOfKeywords(token.value, ['TRUE','FALSE'])) return [idx, new values.Boolean(token.value)];
      else if (keywords.isKeyword(token.value, 'NULL')) return [idx, new values.Null];
      throw getError('a concrete value', token);
    }
    if (token instanceof tk.Number) return [idx, new values.Number(token.value)]; // TODO should validate?
    else if (token instanceof tk.SingleQuotedString) return [idx, new values.SingleQuotedString(token.value)];
    else if (token instanceof tk.NationalStringLiteral) return [idx, new values.NationalStringLiteral(token.value)];
    else if (token instanceof tk.HexStringLiteral) return [idx, new values.HexStringLiteral(token.value)];
    throw getError('a value', token);
};
const expectLiteralString = (tokenSet: TokenSet, start: number): ParseResult<string> => {
  const [idx, token] = nextMeaningfulToken(tokenSet, start);
  if (token instanceof tk.SingleQuotedString) return [idx, token.content];
  throw getError('literal string', token);
};
const parsePrefix = (tokenSet: TokenSet, start: number): ParseResult<Expr> => {
};
const tryParseDataType = (tokenSet: TokenSet, start: number): ParseResult<undefined|DataType> => {
  try {
    return parseDataType(tokenSet, start);
  } catch(err) {
    if (err instanceof ParseError) return [start, undefined];
    throw err;
  }
};
const parseDataType = (tokenSet: TokenSet, start: number): ParseResult<DataType> => {
  let idx: number;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, start);
  if (token instanceof tk.Word){
    if (types.inDataTypeNameL(token.value)) {
      const [,length] = [idx] = parseLength(tokenSet, idx);
      return [idx, types.mapperL[token.value](length)];
    } else if (types.inDataTypeNameOptPS(token.value)) {
      const [,ps] = [idx] = parseOptionalPrecisionScale(tokenSet, idx);
      if (ps instanceof Array) return [idx, types.mapperOptPS[token.value](...ps)];
      else return [idx, types.mapperOptPS[token.value]()];
    } else if (types.inDataTypeNameOptP(token.value)) {
      const [,p] = [idx] = parseOptionalPrecision(tokenSet, idx);
      return [idx,types.mapperOptP[token.value](typeof p === 'number' ? p : undefined)];
    } else if (equalToKeyword(token, 'DOUBLE')) {
      const [,found] = [idx] = parseKeyword(tokenSet, idx, 'PRECISION');
      let dbl = token.value;
      if (found) dbl += ' PRECISION'; // TODO keyword
      return [idx, types.mapperNoArgs[dbl as types.DataTypeNameNoArgs]]; // TODO type assertion
    } else if (inKeywords(token, ['TIME', 'TIMESTAMP'])) {
      const withOrWithout = (['WITH','WITHOUT'] as const).some(word => {
        const [,found] = [idx] = parseKeyword(tokenSet, idx, word);
        return found;
      });
      if (withOrWithout) idx = expectKeywords(tokenSet, idx, ['TIME', 'ZONE']);
      return [idx,types.mapperNoArgs[token.value as types.DataTypeNameNoArgs]]; // TODO type assertion
    } else if (types.inDataTypeNameNoArgs(token.value)) {
      return [idx,types.mapperNoArgs[token.value]];
    }
  }
  throw getError('a data type name', token);
};
const parseLength = (tokenSet: TokenSet, start: number): ParseResult<number> => {
  let idx = expectToken(tokenSet, start, tk.LPAREN);
  const [,length] = [idx] = parseLiteralUint(tokenSet, idx);
  return [expectToken(tokenSet, idx, tk.RPAREN), length];
};
const parseOptionalPrecisionScale = (tokenSet: TokenSet, start: number): ParseResult<undefined|[number,number]> => {
  const optIdx = consumeToken(tokenSet, start, tk.LPAREN);
  if (optIdx) {
    let idx;
    const [,precision] = [idx] = parseLiteralUint(tokenSet, optIdx);
    idx = expectToken(tokenSet, idx, tk.COMMA);
    const [,scale] = [idx] = parseLiteralUint(tokenSet, idx);
    return [idx, [precision, scale]];
  }
  return [start, undefined];
};
const parseOptionalPrecision = (tokenSet: TokenSet, start: number): ParseResult<undefined|number> => {
  const optIdx = consumeToken(tokenSet, start, tk.LPAREN);
  if (optIdx) {
    const [idx,precision] = parseLiteralUint(tokenSet, optIdx);
    return [expectToken(tokenSet, idx, tk.RPAREN), precision];
  }
  return [start, undefined];
};
const parseLiteralUint = (tokenSet: TokenSet, start: number): [number,number] => {
  const [idx, token] = nextMeaningfulToken(tokenSet, start);
  if (token instanceof tk.Number) return [idx, parseInt(token.value)];
  throw getError('literal int', token);
};

const parseInfix = (tokenSet: TokenSet, start: number, expr: Expr, precedence: Precedence): ParseResult<Expr> => {
  let idx;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, start);
  if (token instanceof Eof) throw new ParseError('unexpected EOF while parsing infix');  // Can only happen if `getNextPrecedence` got out of sync with this function
  if (token instanceof tk.Operator /* TODO precise condition  */ || inKeywords(token, ['AND','OR','LIKE'])) {
    const [right] = [idx] = parseExpr(tokenSet, idx);
    return [idx, new exprs.BinaryOp(expr, new ops.BinaryOperator(token.value), right)];
  }
  const [,found] = [idx] = parseKeywords(tokenSet, idx, ['NOT', 'LIKE']);
  if(found) {
    const [ right ] = [idx] = parseExpr(tokenSet, idx, precedence);
    return [idx, new exprs.BinaryOp(expr, new ops.BinaryOperator('NOT LIKE'), right)];
  }
  if (equalToKeyword(token, 'IS')) {
    let [,found] = [idx] = parseKeyword(tokenSet, idx, 'NULL');
    if (found) return [idx, new exprs.IsNull(expr)];
    [,found] = [idx] = parseKeywords(tokenSet, idx, ['NOT', 'NULL']);
    if (found) return [idx, new exprs.IsNotNull(expr)];
  } else if (inKeywords(token, ['NOT','IN','BETWEEN'])) {
    const negated = equalToKeyword(token, 'NOT');
    let [,found] = [idx] = parseKeyword(tokenSet, idx, 'IN');
    if (found) return parseIn(tokenSet, idx, expr, negated);
    [,found] = [idx] = parseKeyword(tokenSet, idx, 'BETWEEN');
    if (found) return parseBetween(tokenSet, idx, expr, negated);
    throw getError('IN or BETWEEN after NOT', peekToken(tokenSet, idx));
  }
  throw new ParseError('No infix parser for token '+token.value); // Can only happen if `getNextPrecedence` got out of sync with this function
};
const parseBetween = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): ParseResult<exprs.Between> => {
  let idx;
  const [,low] = [idx] = parseExpr(tokenSet, start, PRECEDENCE.BETWEEN_PREC);
  idx = expectKeyword(tokenSet, idx, 'AND');
  const [,high] = [idx] = parseExpr(tokenSet, idx, PRECEDENCE.BETWEEN_PREC);
  return [idx, new exprs.Between(expr, negated, low, high)];
};
const parseIn = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): ParseResult<exprs.InList> => {
  let idx = expectToken(tokenSet, start, tk.LPAREN);
  const [,found] = [idx] = parseKeywords(tokenSet, idx, ['SELECT','WITH']);
  if (found) { // subquery is not supported
    throw getError('columns (subquery is not supported)', peekToken(tokenSet, idx));
  }
  const [,exs] = [idx] = parseCommaSeparated(tokenSet, idx, parseExpr);
  return [expectToken(tokenSet, idx, tk.RPAREN), new exprs.InList(expr, exs, negated)];
};
const parseParenthesizedColumnList = (tokenSet: TokenSet, start: number, isOptional: boolean): ParseResult<Ident[]> => {
  const optIdx = consumeToken(tokenSet, start, tk.LPAREN);
  if (optIdx) {
    const [idx, idents] = parseCommaSeparated(tokenSet, optIdx, parseIdentifier);
    return [expectToken(tokenSet, idx, tk.RPAREN), idents];
  } else if (isOptional) {
    return [start, []];
  } else {
    throw getError('a list of columns in parentheses', peekToken(tokenSet, start));
  }
};
function parseCommaSeparated<T>(tokenSet: TokenSet, start: number, callback: (tokenSet: TokenSet, idx: number) => [number,T]): ParseResult<T[]> {
  const values: T[] = [];
  let result;
  let idx: number|undefined = start;
  do {
    result = callback(tokenSet, idx);
    values.push(result[1]);
    idx = consumeToken(tokenSet, result[0], tk.COMMA);
  } while (idx);
  return [idx||result[0],values];
}
const parseObjectName = (tokenSet: TokenSet, start: number): ParseResult<ObjectName> => {
  const idents: Ident[] = [];
  let result;
  let idx: number|undefined =start;
  do {
    result = parseIdentifier(tokenSet, idx);
    idents.push(result[1]);
    idx = consumeToken(tokenSet, result[0], tk.COLON);
  } while (idx);
  return [idx||result[0],new ObjectName(idents)];
};
const parseIdentifier = (tokenSet: TokenSet, start: number): ParseResult<Ident> => {
  const [idx, token] = nextMeaningfulToken(tokenSet, start);
  if(token instanceof Eof) throw getError('identifier', token);
  if(token instanceof tk.DelimitedIdent) {
    return [idx, new Ident(token.content, token.delimiter)];
  } else if(token instanceof tk.Word) { // TODO exclude keyword
    return [idx, new Ident(token.value)];
  } else {
    throw getError('identifier', token);
  }
};
const expectKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): number => {
  const token = peekToken(tokenSet, start);
  if (equalToKeyword(token, keyword)) return nextMeaningfulToken(tokenSet, start)[0];
  throw getError(keyword, token);
};
const expectKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): number => {
  let idx = start;
  keywords.forEach(keyword => idx = expectKeyword(tokenSet, idx, keyword));
  return idx;
};
const parseKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): ParseResult<boolean> => {
  const token = peekToken(tokenSet, start);
  return equalToKeyword(token, keyword) ? [nextMeaningfulToken(tokenSet, start)[0], true] : [start, false];
};
const parseKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): ParseResult<boolean> => {
  let idx=start;
  for (let j=0; idx<tokenSet.length && j<keywords.length; j++) {
    const [,found] = [idx] = parseKeyword(tokenSet, idx, keywords[j]);
    if(!found) return [start, false];
  }
  return [idx, true];
};
const expectToken = (tokenSet: TokenSet, start: number, expected: Token): number => {
  const [idx, token] = nextMeaningfulToken(tokenSet, start);
  if (token instanceof Token && token.value === expected.value) return idx;
  throw getError(expected.value, token);
};
const consumeToken = (tokenSet: TokenSet, start: number, consumedToken: Token): number|undefined => {
  const [idx, token] = nextMeaningfulToken(tokenSet, start);
  if(token instanceof Token && token.value === consumedToken.value) return idx;
};
type NextTokenResult = Token|Eof
const nextMeaningfulToken = (tokenSet: TokenSet, start: number): ParseResult<NextTokenResult> => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof tk.Whitespace) i++;
  return tokenSet.length<=i ? [Infinity, EOF] : [i+1, tokenSet[i]];
};
const peekToken = (tokenSet: TokenSet, start: number): Token|Eof => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof tk.Whitespace) i++;
  return tokenSet.length<=i ? EOF : tokenSet[i];
};
const equalToKeyword = (token: Token|Eof, keyword: Keyword): boolean => token instanceof tk.Word && token.value === keyword;
const inKeywords = (token: Token, keywords: Keyword[]): boolean => keywords.some(keyword => tk.tokenUtil.equalToKeyword(token, keyword)); // TODO delete
const getError = (expected: string, actual: Token|string|Eof): ParseError => new ParseError();
const unsupportedExprssion = (token: Token|Eof): ParseError => new ParseError('Unsupported expression found: ' + token.value);

