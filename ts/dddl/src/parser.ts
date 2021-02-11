import {
  tokenize,
  Token,
  TokenSet,
  tokenUtil,
  Whitespace,
  Word,
  DelimitedIdent,
  Num,
  COLON,
  SEMICOLON,
  LPAREN,
  RPAREN,
  COMMA,
  Operator,
  SingleQuotedString,
} from './tokenizer';
import { types } from './data-types';
import { Keyword } from './keywords';

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
    public dataType: types.DataType,
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
class TypedString implements Expr {
  constructor(
    public dataType: types.DataType,
    public value: string,
  ) {}
}
class SqlOption {
  constructor(
    public name: Ident,
    public value: Value,
  ) {}
}
class Value {}
class Ident {
  constructor(
    public value: string,
    public quoteStyle?: string,
  ) {}
}
class FileFormat {}

class BinaryOperator {
  constructor(
    public value: string, // TODO literal type
  ) {}
}

class ParseResult<T=Nothing> {
  constructor(
    public idx: number, // for NotFound, idx should be the value before try parse
    public content: T,
  ) {}
}
class Eof extends Token {} // TODO don't want to extend Token
const EOF = new Eof('EOF');
class Nothing {}
const NOTHING = new Nothing;

class ParseError extends Error {}

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
  let result: ParseResult<ColumnDef|TableConstraint|ColumnsAndConstraints> = new ParseResult(start, Nothing); // TODO should add Token to generic?
  const columns: ColumnDef[] = [];
  const constraints: TableConstraint[] = [];
  let idx: number|undefined = start;
  if (!(consumeToken(tokenSet, start, LPAREN)) ||
       (idx = consumeToken(tokenSet, start, RPAREN)) ) {
    return new ParseResult(idx, [columns, constraints]);
  }
  result = parseOptionalTableConstraint(tokenSet, start);
}
const parseOptionalTableConstraint = (tokenSet: TokenSet, start: number): ParseResult<TableConstraint> => {
  let result: ParseResult<Nothing|Ident|Ident[]|ObjectName|Unique|ForeignKey|Expr> = parseKeyword(tokenSet, start, 'CONSTRAINT');
  const name = result.content instanceof Ident ? result.content : undefined;
  result = nextMeaningfulToken(tokenSet, result.idx);
  const token = result.content instanceof Token ? result.content : undefined;
  if (!token) throw getError('PRIMARY, UNIQUE, FOREIGN, or CHECK', peekToken(tokenSet, result.idx));
  if (inKeywords(token, ['PRIMARY', 'UNIQUE'])) {
    const isPrimary = equalToKeyword(token, 'PRIMARY');
    if (isPrimary) {
      result.idx = expectKeyword(tokenSet, result.idx, 'KEY');
    }
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
    result.idx = expectToken(tokenSet, result.idx, LPAREN);
    result = parseExpr(tokenSet, result.idx);
    result.idx = expectToken(tokenSet, result.idx, RPAREN);
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
  let result: ParseResult<Nothing|types.DataType|string|TypedString> = tryParseDataType(tokenSet, start);
  if (result.content instanceof types.DataType) {
    const dataType = result.content;
    const { content: value } = result = expectLiteralString(tokenSet, start);
    result.content = new TypedString(dataType!, value!);
    return result as ParseResult<TypedString>;
  }
  result = nextMeaningfulToken(tokenSet, result.idx);
};
const expectLiteralString = (tokenSet: TokenSet, start: number): ParseResult<string> => {
  const result: ParseResult<string|Token|Eof> = nextMeaningfulToken(tokenSet, start);
  if (result.content instanceof SingleQuotedString) {
    result.content = result.content.value;
    return result as ParseResult<string>;
  }
  throw getError('literal string', result.content);
};
const parsePrefix = (tokenSet: TokenSet, start: number): ParseResult<Expr> => {
};
const tryParseDataType = (tokenSet: TokenSet, start: number): ParseResult<types.DataType|Nothing> => {
  try {
    return parseDataType(tokenSet, start);
  } catch(err) {
    if (err instanceof ParseError) return new ParseResult(start, NOTHING);
    throw err;
  }
}
const parseDataType = (tokenSet: TokenSet, start: number): ParseResult<types.DataType> => {
  let result: ParseResult<Nothing|Token|number|[number,number]|types.DataType>;
  const { content } = result = nextMeaningfulToken(tokenSet, start);
  if (content instanceof Word){
    if (types.isDataTypeNameL(content.value)) {
      const { content: length } = result = parseLength(tokenSet, result.idx);
      result.content = types.mapperL[content.value](length!);
    } else if (types.isDataTypeNameOptPS(content.value)) {
      result = parseOptionalPrecisionScale(tokenSet, result.idx);
      if (result.content instanceof Array) {
        const ps = result.content;
        result.content = types.mapperOptPS[content.value](...ps);
      } else {
        result.content = types.mapperOptPS[content.value]();
      }
    } else if (types.isDataTypeNameOptP(content.value)) {
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
      if (result instanceof Word) dbl += ' ' + result.content;
      result.content = types.mapperNoArgs[dbl as types.DataTypeNameNoArgs]; // TODO type assertion
    } else if (inKeywords(content, ['TIME', 'TIMESTAMP'])) {
      if ((result = parseKeyword(tokenSet, result.idx, 'WITH')) instanceof Found ||
          (result = parseKeyword(tokenSet, result.idx, 'WITHOUT')) instanceof Found) {
        result.idx = expectKeywords(tokenSet, result.idx, ['WITH', 'ZONE']);
      }
      result.content = types.mapperNoArgs[content.value as types.DataTypeNameNoArgs]; // TODO type assertion
    } else if (types.isDataTypeNameNoArgs(content.value)) {
      result.content = types.mapperNoArgs[content.value];
    } else {
      throw getError('a data type name', content);
    }
  } else {
    throw getError('a data type name', content!);
  }
  return result as ParseResult<types.DataType>;
};
const parseLength = (tokenSet: TokenSet, start: number): ParseResult<number> => {
  const idx = expectToken(tokenSet, start, LPAREN);
  const result = parseLiteralUint(tokenSet, idx);
  result.idx = expectToken(tokenSet, result.idx, RPAREN);
  return result;
};
const parseOptionalPrecisionScale = (tokenSet: TokenSet, start: number): ParseResult<Nothing|[number,number]> => {
  const idx = consumeToken(tokenSet, start, LPAREN);
  if (idx) {
    const ps: number[] = [];
    let result: ParseResult<number|[number,number]>;
    const { content: precision } = result = parseLiteralUint(tokenSet, idx);
    ps.push(precision!);
    result.idx = expectToken(tokenSet, result.idx, COMMA);
    const { content: scale } = result = parseLiteralUint(tokenSet, idx);
    ps.push(scale!);
    result.content = ps as [number,number];
    return result as ParseResult<[number,number]>;
  }
  return new ParseResult(start, NOTHING);
};
const parseOptionalPrecision = (tokenSet: TokenSet, start: number): ParseResult<Nothing|number> => {
  const idx = consumeToken(tokenSet, start, LPAREN);
  if (idx) {
    const result = parseLiteralUint(tokenSet, idx);
    result.idx = expectToken(tokenSet, result.idx, RPAREN);
    return result;
  }
  return new ParseResult(start, NOTHING);
};
const parseLiteralUint = (tokenSet: TokenSet, start: number): ParseResult<number> => {
  let result: ParseResult<Token|number>;
  const { content } = result = nextMeaningfulToken(tokenSet, start);
  if (content instanceof Num) {
    result.content = parseInt(content.value);
  } else {
    throw getError('literal int', content!);
  }
  return result as ParseResult<number>;
};

const parseInfix = (tokenSet: TokenSet, start: number, expr: Expr, precedence: number): ParseResult<Expr> => {
  let result: ParseResult<Nothing|Expr> = nextMeaningfulToken(tokenSet, start);
  if (result.content instanceof Eof) throw new ParseError('unexpected EOF while parsing infix');  // Can only happen if `getNextPrecedence` got out of sync with this function
  const token = result.content as Token;
  if (token instanceof Operator /* TODO precise condition  */ || inKeywords(token, ['AND','OR','LIKE'])) {
    const { content: right } = result = parseExpr(tokenSet, result.idx);
    result.content = new BinaryOp(expr, new BinaryOperator(token.value), right!);
    return result as ParseResult<Expr>;
  } else {
    result = parseKeywords(tokenSet, result.idx, ['NOT', 'LIKE']);
    if(result.content) {
      const { content: right } = result = parseExpr(tokenSet, result.idx);
      result.content = new BinaryOp(expr, new BinaryOperator('NOT LIKE'), right!);
      return result as ParseResult<Expr>;
    }
  }
  if (equalToKeyword(token, 'IS')) {
    result = parseKeyword(tokenSet, result.idx, 'NULL');
    if (result.content instanceof Word) {
      result.content = new IsNull(expr);
      return result as ParseResult<Expr>;
    } else {
      result = parseKeywords(tokenSet, result.idx, ['NOT', 'NULL']);
      if (result.content instanceof Word) {
        result.content = new IsNotNull(expr);
        return result as ParseResult<Expr>;
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
  throw new ParseError('No infix parser for token '+token.value); // Can only happen if `getNextPrecedence` got out of sync with this function
};
const parseBetween = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): ParseResult<Between> => {
  let result: ParseResult<Expr|Between>;
  const { content: low } = result = parseExpr(tokenSet, start, PRECEDENCE.BETWEEN_PREC);
  result.idx = expectKeyword(tokenSet, result.idx, 'AND');
  const { content: high } = result = parseExpr(tokenSet, result.idx, PRECEDENCE.BETWEEN_PREC);
  result.content = new Between(expr, negated, low!, high!);
  return result as ParseResult<Between>;
};
const parseIn = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): ParseResult<InList> => {
  let result: ParseResult<Nothing|Token|Expr[]|InList>;
  const idx = expectToken(tokenSet, start, LPAREN);
  result = parseKeywords(tokenSet, idx, ['SELECT','WITH']);
  if (result.content instanceof Word) { // subquery is not supported
    throw getError('columns (subquery is not supported)', result.content as Token);
  }
  result = parseCommaSeparated(tokenSet, idx, parseExpr);
  result.content = new InList(expr, result.content as Expr[], negated);
  result.idx = expectToken(tokenSet, result.idx, RPAREN);
  return result as ParseResult<InList>;
};
const parseParenthesizedColumnList = (tokenSet: TokenSet, start: number, isOptional: boolean): ParseResult<Ident[]> => {
  const idx = consumeToken(tokenSet, start, LPAREN);
  let result: ParseResult<void|Token|Ident[]>;
  if (idx) {
    result = parseCommaSeparated(tokenSet, idx, parseIdentifier);
    result.idx = expectToken(tokenSet, result.idx, RPAREN);
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
    idx = consumeToken(tokenSet, result.idx, COMMA);
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
    idx = consumeToken(tokenSet, result.idx, COLON);
    if(!idx) break;
  }
  result.content = new ObjectName(idents);
  return result as ParseResult<ObjectName>;
};
const parseIdentifier = (tokenSet: TokenSet, start: number): ParseResult<Ident> => {
  const result: ParseResult<Ident|Token> = nextMeaningfulToken(tokenSet, start);
  if(result.content instanceof Eof) throw getError('identifier', result.content);
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
const parseKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): ParseResult<Token|Eof|Nothing> => {
  const token = peekToken(tokenSet, start);
  return equalToKeyword(token, keyword) ? nextMeaningfulToken(tokenSet, start) : new ParseResult(start, NOTHING);
};
const parseKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): ParseResult<Token|Nothing> => {
  let result: ParseResult<Nothing|Token>|undefined;
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
  throw getError(expected.value, result.content!);
};
const consumeToken = (tokenSet: TokenSet, start: number, consumedToken: Token): number|undefined => {
  const result = nextMeaningfulToken(tokenSet, start);
  if(result.content instanceof Token && result.content.value === consumedToken.value) return result.idx;
};
const nextMeaningfulToken = (tokenSet: TokenSet, start: number): ParseResult<Token|Eof> => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof Whitespace) i++;
  return tokenSet.length<=i ? new ParseResult(Infinity, EOF) : new ParseResult(i+1, tokenSet[i]);
};
const peekToken = (tokenSet: TokenSet, start: number): Token|Eof => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof Whitespace) i++;
  return tokenSet.length<=i ? EOF : tokenSet[i];
};
const equalToKeyword = tokenUtil.equalToKeyword;
const inKeywords = (token: Token, keywords: Keyword[]): boolean => keywords.some(keyword => tokenUtil.equalToKeyword(token, keyword));
const getError = (expected: string, actual: Token|string|Eof): ParseError => new ParseError();

