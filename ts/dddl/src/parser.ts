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

class ParseResult<T> {
  constructor(
    public idx: number, // for NotFound, idx should be the value before try parse
    public content: T,
  ) {}
}
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
    if(tokenSet.length <= idx) break;
    if(expectingStatementDelimiter) throw new Error();
    const stmt = parseCreateStatement(tokenSet, idx);
    statements.push(stmt);
  }
};
const parseCreateStatement = (tokenSet: TokenSet, start: number): CreateTableStatement => {
  // let i = nextMeaningfulToken(tokenSet, start); // TODO next?
  let result = parseKeyword(tokenSet, start, 'CREATE');
  if(result.content instanceof Nothing) {
    throw getError('a create statement', peekToken(tokenSet, result.idx));
  }
  result = parseKeywords(tokenSet, result.idx, ['OR', 'REPLACE']);
  const orReplace = !!result.content;
  return parseCreateTableStatement(tokenSet, result.idx, orReplace);
};
const parseCreateTableStatement = (tokenSet: TokenSet, start: number, orReplace: boolean): CreateTableStatement => {
  let result: ParseResult = parseKeywords(tokenSet, start, ['IF','NOT','EXISTS']);
  const ifNotExists = result.content instanceof Token;
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
  let result: ParseResult<ColumnDef|TableConstraint|ColumnsAndConstraints> = new ParseResult(start, Nothing);
  const columns: ColumnDef[] = [];
  const constraints: TableConstraint[] = [];
  let idx: number|undefined = start;
  if (!(consumeToken(tokenSet, start, tk.LPAREN)) || (idx = consumeToken(tokenSet, start, tk.RPAREN)) ) {
    return new ParseResult(idx, [columns, constraints]);
  }
  result = parseOptionalTableConstraint(tokenSet, start);
};
const parseOptionalTableConstraint = (tokenSet: TokenSet, start: number): ParseResult<TableConstraint> => {
  let result: ParseResult<Nothing|Ident|Ident[]|ObjectName|Unique|ForeignKey|Expr> = parseKeyword(tokenSet, start, 'CONSTRAINT');
  const name = result.content instanceof Ident ? result.content : undefined;
  result = nextMeaningfulToken(tokenSet, result.idx);
  const token = result.content instanceof Token ? result.content : undefined;
  if (!token) throw getError('PRIMARY, UNIQUE, FOREIGN, or CHECK', peekToken(tokenSet, result.idx));
  if (inKeywords(token, ['PRIMARY', 'UNIQUE'])) {
    const isPrimary = equalToKeyword(token, 'PRIMARY');
    if (isPrimary) result.idx = expectKeyword(tokenSet, result.idx, 'KEY');
    const { content: columns } = result = parseParenthesizedColumnList(tokenSet, result.idx, false);
    result.content = new Unique(name, columns, isPrimary);
    return result as ParseResult<Unique>;
  } else if (equalToKeyword(token, 'FOREIGN')) {
    result.idx = expectKeyword(tokenSet, result.idx, 'KEY');
    const { content: columns } = result = parseParenthesizedColumnList(tokenSet, result.idx, false);
    result.idx = expectKeyword(tokenSet, result.idx, 'REFERENCES');
    const { content: foreignTable } = result = parseObjectName(tokenSet, result.idx);
    const { content: referredColumns } = result = parseParenthesizedColumnList(tokenSet, result.idx, false);
    result.content = new ForeignKey(name, columns, foreignTable, referredColumns);
  } else if (equalToKeyword(token, 'CHECK')) {
    result.idx = expectToken(tokenSet, result.idx, tk.LPAREN);
    result = parseExpr(tokenSet, result.idx);
    result.idx = expectToken(tokenSet, result.idx, tk.RPAREN);
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
const parseExpr = (tokenSet: TokenSet, start: number, precedence: Precedence = PRECEDENCE.DEFAULT): [number, Expr] => {
  // let result: ParseResult<ParseKeywordResult|DataType|string|exprs.TypedString> = tryParseDataType(tokenSet, start);
  let idx: number;
  const [,dataType] = [idx] = tryParseDataType(tokenSet, start);
  if (dataType instanceof DataType) {
    const { content: value } = { idx } = expectLiteralString(tokenSet, start);
    return [idx, new exprs.TypedString(dataType, value)];
  }
  const prevIdx = result.idx;
  const  = nextMeaningfulToken(tokenSet, result.idx);
  if (result.content instanceof tk.Word) {
    if (keywords.isOneOfKeywords(result.content.value, ['TRUE','FALSE','NULL'])) {
      return expectValue(tokenSet, result.idx);
    } else if (keywords.isKeyword(result.content.value, 'CASE')) {
      return parseCase(tokenSet, result.idx);
    }

  }
};
const expectCase = (tokenSet: TokenSet, start: number): ParseResult<Expr> => {
  let result: ParseResult<ParseKeywordResult|Expr>;
  result = parseKeyword(tokenSet, start, 'WHEN');
  if (result instanceof Nothing) {
    result = parseExpr(tokenSet, start);
    result.idx = expectKeyword(tokenSet, start, 'WHEN');
  }
};
const expectValue = (tokenSet: TokenSet, start: number): ParseResult<Value<boolean|string|undefined>> => {
    let result: ParseResult<NextTokenResult|Value<string|boolean|undefined>>;
    const { content } = result = nextMeaningfulToken(tokenSet, start);
    if (content instanceof tk.Word) {
      if (keywords.isOneOfKeywords(content.value, ['TRUE','FALSE'])) {
        result.content = new values.Boolean(content.value);
        return result as ParseResult<Value<boolean>>;
      } else if (keywords.isKeyword(content.value, 'NULL')) {
        result.content = new values.Null;
        return result as ParseResult<Value<undefined>>;
      }
      throw getError('a concrete value', content);
    }
    if (content instanceof tk.Number) {
      result.content = new values.Number(content.value); // TODO should validate?
      return result as ParseResult<Value<string>>;
    } else if (content instanceof tk.SingleQuotedString) {
      result.content = new values.SingleQuotedString(content.value);
      return result as ParseResult<Value<string>>;
    } else if (content instanceof tk.NationalStringLiteral) {
      result.content = new values.NationalStringLiteral(content.value);
      return result as ParseResult<Value<string>>;
    } else if (content instanceof tk.HexStringLiteral) {
      result.content = new values.HexStringLiteral(content.value);
      return result as ParseResult<Value<string>>;
    }
    throw getError('a value', content);
};
const expectLiteralString = (tokenSet: TokenSet, start: number): ParseResult<string> => {
  const result: ParseResult<string|Token|Eof> = nextMeaningfulToken(tokenSet, start);
  if (result.content instanceof tk.SingleQuotedString) {
    result.content = result.content.content;
    return result as ParseResult<string>;
  }
  throw getError('literal string', result.content);
};
const parsePrefix = (tokenSet: TokenSet, start: number): ParseResult<Expr> => {
};
const tryParseDataType = (tokenSet: TokenSet, start: number): [number, Nothing|DataType] => {
  try {
    return parseDataType(tokenSet, start);
  } catch(err) {
    if (err instanceof ParseError) return [start, NOTHING];
    throw err;
  }
};
const parseDataType = (tokenSet: TokenSet, start: number): [number,DataType] => {
  let result: ParseResult<ParseKeywordResult|number|[number,number]|DataType>;
  const { content } = result = nextMeaningfulToken(tokenSet, start);
  if (content instanceof tk.Word){
    if (types.inDataTypeNameL(content.value)) {
      const { content: length } = result = parseLength(tokenSet, result.idx);
      result.content = types.mapperL[content.value](length);
    } else if (types.inDataTypeNameOptPS(content.value)) {
      result = parseOptionalPrecisionScale(tokenSet, result.idx);
      if (result.content instanceof Array) {
        const ps = result.content;
        result.content = types.mapperOptPS[content.value](...ps);
      } else {
        result.content = types.mapperOptPS[content.value]();
      }
    } else if (types.inDataTypeNameOptP(content.value)) {
      result = parseOptionalPrecision(tokenSet, result.idx);
      if (typeof result.content === 'number') {
        const precision = result.content;
        result.content = types.mapperOptP[content.value](precision);
      } else {
        result.content = types.mapperOptP[content.value]();
      }
    } else if (equalToKeyword(content, 'DOUBLE')) {
      result = parseKeyword(tokenSet, result.idx, 'PRECISION');
      let dbl = content.value;
      if (result instanceof Token) dbl += ' ' + result.content;
      result.content = types.mapperNoArgs[dbl as types.DataTypeNameNoArgs]; // TODO type assertion
    } else if (inKeywords(content, ['TIME', 'TIMESTAMP'])) {
      const withOrWithout = (['WITH','WITHOUT'] as const).some(word => {
        result = parseKeyword(tokenSet, result.idx, word);
        return result.content instanceof Token;
      });
      if (withOrWithout)  result.idx = expectKeywords(tokenSet, result.idx, ['TIME', 'ZONE']);
      result.content = types.mapperNoArgs[content.value as types.DataTypeNameNoArgs]; // TODO type assertion
    } else if (types.inDataTypeNameNoArgs(content.value)) {
      result.content = types.mapperNoArgs[content.value];
    } else {
      throw getError('a data type name', content);
    }
  } else {
    throw getError('a data type name', content);
  }
  return [result.idx, result.content];
};
const parseLength = (tokenSet: TokenSet, start: number): ParseResult<number> => {
  const idx = expectToken(tokenSet, start, tk.LPAREN);
  const result = parseLiteralUint(tokenSet, idx);
  result.idx = expectToken(tokenSet, result.idx, tk.RPAREN);
  return result;
};
const parseOptionalPrecisionScale = (tokenSet: TokenSet, start: number): ParseResult<Nothing|[number,number]> => {
  const idx = consumeToken(tokenSet, start, tk.LPAREN);
  if (idx) {
    const ps: number[] = [];
    let result: ParseResult<number|[number,number]>;
    const { content: precision } = result = parseLiteralUint(tokenSet, idx);
    ps.push(precision);
    result.idx = expectToken(tokenSet, result.idx, tk.COMMA);
    const { content: scale } = result = parseLiteralUint(tokenSet, idx);
    ps.push(scale);
    result.content = ps as [number,number];
    return result as ParseResult<[number,number]>;
  }
  return new ParseResult(start, NOTHING);
};
const parseOptionalPrecision = (tokenSet: TokenSet, start: number): ParseResult<Nothing|number> => {
  const idx = consumeToken(tokenSet, start, tk.LPAREN);
  if (idx) {
    const result = parseLiteralUint(tokenSet, idx);
    result.idx = expectToken(tokenSet, result.idx, tk.RPAREN);
    return result;
  }
  return new ParseResult(start, NOTHING);
};
const parseLiteralUint = (tokenSet: TokenSet, start: number): ParseResult<number> => {
  let result: ParseResult<NextTokenResult|number>;
  const { content } = result = nextMeaningfulToken(tokenSet, start);
  if (content instanceof tk.Number) {
    result.content = parseInt(content.value);
  } else {
    throw getError('literal int', content);
  }
  return result as ParseResult<number>;
};

const parseInfix = (tokenSet: TokenSet, start: number, expr: Expr, precedence: number): ParseResult<Expr> => {
  let result: ParseResult<{}> = nextMeaningfulToken(tokenSet, start);
  if (result.content instanceof Eof) throw new ParseError('unexpected EOF while parsing infix');  // Can only happen if `getNextPrecedence` got out of sync with this function
  const token = result.content as Token;
  if (token instanceof tk.Operator /* TODO precise condition  */ || inKeywords(token, ['AND','OR','LIKE'])) {
    const { content: right } = result = parseExpr(tokenSet, result.idx);
    result.content = new exprs.BinaryOp(expr, new ops.BinaryOperator(token.value), right);
    return result as ParseResult<Expr>;
  } else {
    result = parseKeywords(tokenSet, result.idx, ['NOT', 'LIKE']);
    if(result.content) {
      const { content: right } = result = parseExpr(tokenSet, result.idx);
      result.content = new exprs.BinaryOp(expr, new ops.BinaryOperator('NOT LIKE'), right);
      return result as ParseResult<Expr>;
    }
  }
  if (equalToKeyword(token, 'IS')) {
    result = parseKeyword(tokenSet, result.idx, 'NULL');
    if (result.content instanceof Token) {
      result.content = new exprs.IsNull(expr);
      return result as ParseResult<Expr>;
    } else {
      result = parseKeywords(tokenSet, result.idx, ['NOT', 'NULL']);
      if (result.content instanceof Token) {
        result.content = new exprs.IsNotNull(expr);
        return result as ParseResult<Expr>;
      }
    }
  } else if (inKeywords(token, ['NOT','IN','BETWEEN'])) {
    const negated = equalToKeyword(token, 'NOT');
    result = parseKeyword(tokenSet, result.idx, 'IN');
    if (result.content instanceof Token) {
      return parseIn(tokenSet, result.idx, expr, negated);
    } else {
      result = parseKeyword(tokenSet, result.idx, 'BETWEEN');
      if (result.content instanceof Token) {
        return parseBetween(tokenSet, result.idx, expr, negated);
      } else {
        throw getError('IN or BETWEEN after NOT', peekToken(tokenSet, result.idx));
      }
    }
  }
  throw new ParseError('No infix parser for token '+token.value); // Can only happen if `getNextPrecedence` got out of sync with this function
};
const parseBetween = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): ParseResult<exprs.Between> => {
  let result: ParseResult<Expr|exprs.Between>;
  const { content: low } = result = parseExpr(tokenSet, start, PRECEDENCE.BETWEEN_PREC);
  result.idx = expectKeyword(tokenSet, result.idx, 'AND');
  const { content: high } = result = parseExpr(tokenSet, result.idx, PRECEDENCE.BETWEEN_PREC);
  result.content = new exprs.Between(expr, negated, low, high);
  return result as ParseResult<exprs.Between>;
};
const parseIn = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): ParseResult<exprs.InList> => {
  let result: ParseResult<ParseKeywordResult|Expr[]|exprs.InList>;
  const idx = expectToken(tokenSet, start, tk.LPAREN);
  result = parseKeywords(tokenSet, idx, ['SELECT','WITH']);
  if (result.content instanceof Token) { // subquery is not supported
    throw getError('columns (subquery is not supported)', result.content as Token);
  }
  result = parseCommaSeparated(tokenSet, idx, parseExpr);
  result.content = new exprs.InList(expr, result.content as Expr[], negated);
  result.idx = expectToken(tokenSet, result.idx, tk.RPAREN);
  return result as ParseResult<exprs.InList>;
};
const parseParenthesizedColumnList = (tokenSet: TokenSet, start: number, isOptional: boolean): ParseResult<Ident[]> => {
  const idx = consumeToken(tokenSet, start, tk.LPAREN);
  let result: ParseResult<void|Token|Ident[]>;
  if (idx) {
    result = parseCommaSeparated(tokenSet, idx, parseIdentifier);
    result.idx = expectToken(tokenSet, result.idx, tk.RPAREN);
  } else if (isOptional) {
    result = new ParseResult(start, [] as Ident[]);
  } else {
    throw getError('a list of columns in parentheses', peekToken(tokenSet, start));
  }
  return result as ParseResult<Ident[]>;
};
function parseCommaSeparated<T>(tokenSet: TokenSet, start: number, callback: (tokenSet: TokenSet, idx: number) => ParseResult<T>): ParseResult<T[]> {
  const values: T[] = [];
  let result: ParseResult<Token|T|T[]>;
  let idx: number|undefined = start;
  for(;;) {
    result = callback(tokenSet, idx);
    values.push(result.content as T);
    idx = consumeToken(tokenSet, result.idx, tk.COMMA);
    if(!idx) break;
  }
  result.content = values;
  return result as ParseResult<T[]>;
}
const parseObjectName = (tokenSet: TokenSet, start: number): ParseResult<ObjectName> => {
  let result: ParseResult<Ident|ObjectName>;
  const idents: Ident[] = [];
  let idx: number|undefined =start;
  for(;;) {
    result = parseIdentifier(tokenSet, idx);
    idents.push(result.content as Ident);
    idx = consumeToken(tokenSet, result.idx, tk.COLON);
    if(!idx) break;
  }
  result.content = new ObjectName(idents);
  return result as ParseResult<ObjectName>;
};
const parseIdentifier = (tokenSet: TokenSet, start: number): ParseResult<Ident> => {
  const result: ParseResult<NextTokenResult|Ident> = nextMeaningfulToken(tokenSet, start);
  if(result.content instanceof Eof) throw getError('identifier', result.content);
  const token = result.content as Token;
  if(token instanceof tk.DelimitedIdent) {
    result.content = new Ident(token.content, token.delimiter);
  } else if(token instanceof tk.Word) { // TODO exclude keyword
    result.content = new Ident(token.value);
  } else {
    throw getError('identifier', token);
  }
  return result as ParseResult<Ident>;
};
const expectKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): number => {
  const token = peekToken(tokenSet, start);
  if (equalToKeyword(token, keyword)) return nextMeaningfulToken(tokenSet, start).idx;
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
  return equalToKeyword(token, keyword) ? nextMeaningfulToken(tokenSet, start) : new ParseResult(start, NOTHING);
};
const parseKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): ParseResult<ParseKeywordResult> => {
  let result: ParseResult<ParseKeywordResult>|undefined;
  let idx=start;
  for (let j=0; idx<tokenSet.length && j<keywords.length; j++) {
    result = parseKeyword(tokenSet, idx, keywords[j]);
    idx = result.idx;
    if(result.content instanceof Nothing) {
      result.idx = start;
      break;
    }
  }
  return result ? result : new ParseResult(start, NOTHING);
};
const expectToken = (tokenSet: TokenSet, start: number, expected: Token): number => {
  const result = nextMeaningfulToken(tokenSet, start);
  if (result.content instanceof Token && result.content.value === expected.value) return result.idx;
  throw getError(expected.value, result.content);
};
const consumeToken = (tokenSet: TokenSet, start: number, consumedToken: Token): number|undefined => {
  const result = nextMeaningfulToken(tokenSet, start);
  if(result.content instanceof Token && result.content.value === consumedToken.value) return result.idx;
};
type NextTokenResult = Token|Eof
const nextMeaningfulToken = (tokenSet: TokenSet, start: number): ParseResult<NextTokenResult> => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof tk.Whitespace) i++;
  return tokenSet.length<=i ? new ParseResult(Infinity, EOF) : new ParseResult(i+1, tokenSet[i]);
};
const peekToken = (tokenSet: TokenSet, start: number): Token|Eof => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof tk.Whitespace) i++;
  return tokenSet.length<=i ? EOF : tokenSet[i];
};
const equalToKeyword = (token: Token|Eof, keyword: Keyword): boolean => token instanceof tk.Word && token.value === keyword;
const inKeywords = (token: Token, keywords: Keyword[]): boolean => keywords.some(keyword => tk.tokenUtil.equalToKeyword(token, keyword)); // TODO delete
const getError = (expected: string, actual: Token|string|Eof): ParseError => new ParseError();

