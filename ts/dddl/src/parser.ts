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

class Eof { private eof = undefined ; }
const EOF = new Eof();
class Nothing { private nothing = undefined ; }
const NOTHING = new Nothing;

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
  let [idx,result] = parseKeyword(tokenSet, start, 'CREATE');
  if(result instanceof Nothing) {
    throw getError('a create statement', peekToken(tokenSet, idx));
  }
  ([idx, result] = parseKeywords(tokenSet, idx, ['OR', 'REPLACE']));
  const orReplace = !(result instanceof Nothing);
  return parseCreateTableStatement(tokenSet, idx, orReplace);
};
const parseCreateTableStatement = (tokenSet: TokenSet, start: number, orReplace: boolean): CreateTableStatement => {
  let idx;
  const [,result] = [idx] = parseKeywords(tokenSet, start, ['IF','NOT','EXISTS']);
  const ifNotExists = result instanceof Token;
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
  if (!(consumeToken(tokenSet, optIdx, tk.LPAREN)) || (optIdx = consumeToken(tokenSet, optIdx, tk.RPAREN)) ) {
    return [optIdx, [columns, constraints]];
  }
  const [idx,optConstraints] = parseOptionalTableConstraint(tokenSet, start);
};
const parseOptionalTableConstraint = (tokenSet: TokenSet, start: number): ParseResult<TableConstraint|Nothing> => {
  let idx;
  const [,result] = [idx] = parseKeyword(tokenSet, start, 'CONSTRAINT');
  if (result instanceof Nothing) {
    return [start, NOTHING];
  } else {
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
  // let result: ParseResult<ParseKeywordResult|DataType|string|exprs.TypedString> = tryParseDataType(tokenSet, start);
  let idx: number;
  const [,dataType] = [idx] = tryParseDataType(tokenSet, start);
  if (dataType instanceof DataType) {
    const [,value] = [idx] = expectLiteralString(tokenSet, start);
    return [idx, new exprs.TypedString(dataType, value)];
  }
  const prevIdx = idx;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if (token instanceof tk.Word) {
    if (keywords.isOneOfKeywords(token.value, ['TRUE','FALSE','NULL'])) {
      return expectValue(tokenSet, prevIdx);
    } else if (keywords.isKeyword(token.value, 'CASE')) {
      return parseCase(tokenSet, idx);
    }
  }
};
const parseCase = (tokenSet: TokenSet, start: number): ParseResult<Expr> => {
  let idx;
  const [,result] = idx = parseKeyword(tokenSet, start, 'WHEN');
  if (result instanceof Nothing) {
    const [,expr] = [idx] = parseExpr(tokenSet, start);
    idx = expectKeyword(tokenSet, start, 'WHEN');
  }
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
const tryParseDataType = (tokenSet: TokenSet, start: number): ParseResult<Nothing|DataType> => {
  try {
    return parseDataType(tokenSet, start);
  } catch(err) {
    if (err instanceof ParseError) return [start, NOTHING];
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
      const [,result] = [idx] = parseKeyword(tokenSet, idx, 'PRECISION');
      let dbl = token.value;
      if (result instanceof Token) dbl += ' ' + result.value;
      return [idx, types.mapperNoArgs[dbl as types.DataTypeNameNoArgs]]; // TODO type assertion
    } else if (inKeywords(token, ['TIME', 'TIMESTAMP'])) {
      const withOrWithout = (['WITH','WITHOUT'] as const).some(word => {
        const [,result] = [idx] = parseKeyword(tokenSet, idx, word);
        return !(result instanceof Nothing);
      });
      if (withOrWithout)  idx = expectKeywords(tokenSet, idx, ['TIME', 'ZONE']);
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
const parseOptionalPrecisionScale = (tokenSet: TokenSet, start: number): ParseResult<Nothing|[number,number]> => {
  const optIdx = consumeToken(tokenSet, start, tk.LPAREN);
  if (optIdx) {
    let idx;
    const [,precision] = [idx] = parseLiteralUint(tokenSet, optIdx);
    idx = expectToken(tokenSet, idx, tk.COMMA);
    const [,scale] = [idx] = parseLiteralUint(tokenSet, idx);
    return [idx, [precision, scale]];
  }
  return [start, NOTHING];
};
const parseOptionalPrecision = (tokenSet: TokenSet, start: number): ParseResult<Nothing|number> => {
  const optIdx = consumeToken(tokenSet, start, tk.LPAREN);
  if (optIdx) {
    const [idx,precision] = parseLiteralUint(tokenSet, optIdx);
    return [expectToken(tokenSet, idx, tk.RPAREN), precision];
  }
  return [start, NOTHING];
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
  } else {
    const [,result] = [idx] = parseKeywords(tokenSet, idx, ['NOT', 'LIKE']);
    if(!(result instanceof Nothing)) {
      const [ right ] = [idx] = parseExpr(tokenSet, idx, precedence);
      return [idx, new exprs.BinaryOp(expr, new ops.BinaryOperator('NOT LIKE'), right)];
    }
  }
  if (equalToKeyword(token, 'IS')) {
    const [,result] = [idx] = parseKeyword(tokenSet, idx, 'NULL');
    if (result instanceof Token) {
      return [idx, new exprs.IsNull(expr)];
    } else {
      const [,result] = [idx] = parseKeywords(tokenSet, idx, ['NOT', 'NULL']);
      if (!(result instanceof Nothing)) {
        return [idx, new exprs.IsNotNull(expr)];
      }
    }
  } else if (inKeywords(token, ['NOT','IN','BETWEEN'])) {
    const negated = equalToKeyword(token, 'NOT');
    const [,result] = [idx] = parseKeyword(tokenSet, idx, 'IN');
    if (!(result instanceof Nothing)) {
      return parseIn(tokenSet, idx, expr, negated);
    } else {
      const [,result] = [idx] = parseKeyword(tokenSet, idx, 'BETWEEN');
      if (!(result instanceof Nothing)) {
        return parseBetween(tokenSet, idx, expr, negated);
      } else {
        throw getError('IN or BETWEEN after NOT', peekToken(tokenSet, idx));
      }
    }
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
  const [,token] = [idx] = parseKeywords(tokenSet, idx, ['SELECT','WITH']);
  if (token instanceof Token) { // subquery is not supported
    throw getError('columns (subquery is not supported)', token);
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
  for(;;) {
    result = callback(tokenSet, idx);
    values.push(result[1]);
    idx = consumeToken(tokenSet, result[0], tk.COMMA);
    if(!idx) break;
  }
  return [idx||result[0],values];
}
const parseObjectName = (tokenSet: TokenSet, start: number): ParseResult<ObjectName> => {
  const idents: Ident[] = [];
  let result;
  let idx: number|undefined =start;
  for(;;) {
    result = parseIdentifier(tokenSet, idx);
    idents.push(result[1]);
    idx = consumeToken(tokenSet, result[0], tk.COLON);
    if(!idx) break;
  }
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
type ParseKeywordResult = Nothing|Eof|Token;
const parseKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): ParseResult<ParseKeywordResult> => {
  const token = peekToken(tokenSet, start);
  return equalToKeyword(token, keyword) ? nextMeaningfulToken(tokenSet, start) : [start, NOTHING];
};
const parseKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): ParseResult<ParseKeywordResult> => {
  let result: [number,ParseKeywordResult] = [start, NOTHING];
  for (let j=0; result[0]<tokenSet.length && j<keywords.length; j++) {
    result = parseKeyword(tokenSet, result[0], keywords[j]);
    if(result instanceof Nothing) return [start, NOTHING];
  }
  return result;
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

