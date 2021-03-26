// Copyright 2018 - 2020 sqlparser-rs contributors. All rights reserved.
// Copyright kena0ki. All rights reserved.
//
// This file was translated and ported from the rust project, available at
// https://github.com/andygrove/sqlparser-rs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { token as tk, Token, TokenSet, TokenizeError } from './tokenizer';
import { dataTypes as types, DataType } from './data-types';
import { keywords, Keyword } from './keywords';
import { exprs, Expr } from './expressions';
import { values, Value } from './values';
import { operators as ops } from './operators';
import { ColumnOption, columnOptions as colOpts } from './column-options';
import { logger } from './util';

export class CreateTableStatement {
  constructor(
    public readonly name: ObjectName, // table name
    public readonly columns: ColumnDef[], // optional schema
    public readonly constraints: TableConstraint[],
    public readonly withOptions: SqlOption[],
    public readonly orReplace: boolean,
    public readonly ifNotExists: boolean,
    public readonly withoutRowid: boolean,
    public readonly external: boolean,
    public readonly fileFormat?: FileFormat,
    public readonly location?: string,
    public readonly query?: Query,  // Not supported
  ) {}
}
export class ObjectName {
  constructor(
    public readonly value: Ident[]
  ) {}
}
export class ColumnDef {
  constructor(
    public readonly name: Ident,
    public readonly dataType: DataType,
    public readonly collation: ObjectName|undefined,
    public readonly options: ColumnOptionDef[],
  ) {}
}
class ColumnOptionDef {
  constructor(
    public readonly name: Ident|undefined,
    public readonly option: ColumnOption,
  ) {}
}
export class TableConstraint {
  public tag = 'TableConstraint'
}
export class Unique extends TableConstraint {
  constructor(
    public readonly name: Ident|undefined,
    public readonly columns: Ident[],
    public readonly isPrimary: boolean, // Whether this is a `PRIMARY KEY` or just a `UNIQUE` constraint
  ) { super(); }
}
// A referential integrity constraint (`[ CONSTRAINT <name> ] FOREIGN KEY (<columns>)
// REFERENCES <foreign_table> (<referred_columns>)`)
export class ForeignKey extends TableConstraint {
  constructor(
    public readonly name: Ident|undefined,
    public readonly columns: Ident[],
    public readonly foreignTable: ObjectName,
    public readonly referredColumns: Ident[],
  ) { super(); }
}
// `[ CONSTRAINT <name> ] CHECK (<expr>)`
export class Check extends TableConstraint {
  constructor(
    public readonly name: Ident|undefined,
    public readonly expr: Expr,
  ) { super(); }
}
class SqlOption {}
class Query {}
export class Ident {
  constructor(
    public readonly value: string,
    public readonly quoteStyle?: string,
  ) {}
}
class FileFormat {}
type ReferencialActionName = 'RESTRICT'|'CASCADE'|'SET NULL'|'NO ACTION'|'SET DEFAULT';
export class ReferencialAction {
  constructor(public readonly name: ReferencialActionName) {}
}

type ParseResult<T> = [number,T];

class Eof {
  private _eof?: never;
  public readonly value = 'EOF';
}
const EOF = new Eof;

export class ParseError extends Error {
  public tag = 'ParseError'
}

export const parse = (src: string): CreateTableStatement[]|ParseError => {
  const result = tk.tokenize(src);
  if (result instanceof TokenizeError) {
    const err = new ParseError(result.message);
    err.stack = result.stack;
    return err;
  }
  const tokenSet = result;
  const statements: CreateTableStatement[] = [];
  let expectingStatementDelimiter = false;
  let idx: number = 0;
  try {
    for(;;) {
      for(;;) { // ignore empty statements (between consecutive semicolons)
        const optIdx = consumeToken(tokenSet, idx, tk.SEMICOLON);
        if (!optIdx) break;
        idx = optIdx;
        expectingStatementDelimiter=false;
      }
      if(peekToken(tokenSet, idx) instanceof Eof) break; // EOF
      if(expectingStatementDelimiter) return unexpectedToken(tokenSet, idx, 'end of statement');
      const [,stmt] = [idx] = parseCreateStatement(tokenSet, idx);
      statements.push(stmt);
      logger.log(stmt,idx);
    }
  } catch (err: unknown) {
    logger.log(tokenSet, idx);
    logger.log(err);
    if (err instanceof ParseError) return err;
    throw err;
  }
  return statements;
};
const parseCreateStatement = (tokenSet: TokenSet, start: number): ParseResult<CreateTableStatement> => {
  let idx=start;
  let [,found] = [idx] = parseKeyword(tokenSet, idx, 'CREATE');
  if(!found) {
    let token;
    [idx,token] =nextMeaningfulToken(tokenSet, idx);
    logger.log(idx, token);
    throw unexpectedToken(tokenSet, idx, 'a create statement');
  }
  [idx, found] = parseKeywords(tokenSet, idx, ['OR', 'REPLACE']);
  const orReplace = found;
  [idx, found] = parseKeyword(tokenSet, idx, 'TABLE');
  if (!found) throw unexpectedToken(tokenSet, idx, 'TABLE after CREATE'); // only support TABLE
  return parseCreateTableStatement(tokenSet, idx, orReplace);
};
const parseCreateTableStatement = (tokenSet: TokenSet, start: number, orReplace: boolean): ParseResult<CreateTableStatement> => {
  let idx=start;
  const [,ifNotExists] = [idx] = parseKeywords(tokenSet, idx, ['IF','NOT','EXISTS']);
  const [,tableName] = [idx] = parseObjectName(tokenSet, idx);
  const [,[columns,constraints]] = [idx] = parseColumns(tokenSet, idx);
  const [,withoutRowid] = [idx] = parseKeywords(tokenSet, idx, ['WITHOUT','ROWID']); // sqlite diarect
  const [,withOptionsFound] = [idx] = parseKeyword(tokenSet, idx, 'WITH');
  let withOptions: SqlOption[] = [];
  if (withOptionsFound) {
    idx = expectToken(tokenSet, idx, tk.LPAREN);
    [idx, withOptions] = skipToClosingParen(tokenSet, idx, [new SqlOption]);
    idx = expectToken(tokenSet, idx, tk.RPAREN);
  }
  const [,asFound] = [idx] = parseKeyword(tokenSet, idx, 'AS');
  if (asFound) throw unsupportedExprssion(peekToken(tokenSet, idx));
  return [idx, new CreateTableStatement(
    tableName,
    columns,
    constraints,
    withOptions,
    orReplace,
    ifNotExists,
    withoutRowid,
    false,
  )];
};
type ColumnsAndConstraints = [ColumnDef[], TableConstraint[]];
const parseColumns = (tokenSet: TokenSet, start: number): ParseResult<ColumnsAndConstraints> => {
  let idx:number=start;
  const columns: ColumnDef[] = [];
  const constraints: TableConstraint[] = [];
  let optIdx: number|undefined = start;
  optIdx = consumeToken(tokenSet, optIdx, tk.LPAREN);
  if (!optIdx) return [idx, [columns, constraints]]; // no starting parenthesis
  idx=optIdx;
  optIdx = consumeToken(tokenSet, optIdx, tk.RPAREN);
  if (optIdx) return [optIdx, [columns, constraints]]; // empty body
  let optIdxRParen:number|undefined;
  do {
    const [,optConstraints] = [idx] = parseOptionalTableConstraint(tokenSet, idx);
    if (optConstraints) {
      constraints.push(optConstraints);
    } else {
      const token = peekToken(tokenSet, idx);
      if (token instanceof tk.WordIdent) {
        const [,columnDef] = [idx] = parseColumnDef(tokenSet, idx);
        logger.log(JSON.stringify(columnDef));
        columns.push(columnDef);
      } else {
        throw unexpectedToken(tokenSet, idx, 'column name or constraint definition');
      }
    }
    const optIdxComma = consumeToken(tokenSet, idx, tk.COMMA); // allow a trailing comma, even though it's not in standard
    optIdxRParen = consumeToken(tokenSet, optIdxComma || idx, tk.RPAREN);
    if (!optIdxComma && !optIdxRParen) throw unexpectedToken(tokenSet, idx, `',' or ')' after column definition`);
    idx = optIdxRParen||optIdxComma||idx;
  } while(!optIdxRParen);
  return [idx, [columns, constraints]];
};
const parseColumnDef = (tokenSet: TokenSet, start: number): ParseResult<ColumnDef> => {
  let idx=start;
  const [,name] = [idx] = parseIdentifier(tokenSet, idx);
  const [,dataType] = [idx] = parseDataType(tokenSet, idx);
  const [,found] = [idx] = parseKeyword(tokenSet, idx, 'COLLATE');
  let collation: ObjectName|undefined;
  if (found) [,collation] = [idx] = parseObjectName(tokenSet, idx);
  const options: ColumnOptionDef[] = [];
  for (;;) {
    const [,found] = [idx] = parseKeyword(tokenSet, idx, 'CONSTRAINT');
    if (found) {
      const [,name] = [idx] = parseIdentifier(tokenSet, idx);
      const [,option] = [idx] = parseOptionalColumnOption(tokenSet, idx);
      if (option) options.push(new ColumnOptionDef(name, option));
      else throw unexpectedToken(tokenSet, idx, `constraint details after CONSTRAINT <name>`);
    } else {
      const [,option] = [idx] = parseOptionalColumnOption(tokenSet, idx);
      if (option) options.push(new ColumnOptionDef(undefined, option));
      else break;
    }
  }
  return [idx, new ColumnDef(name, dataType, collation, options)];
};
const parseOptionalColumnOption = (tokenSet: TokenSet, start: number): ParseResult<ColumnOption|undefined> => {
  let idx=start;
  let found: boolean;
  [idx, found] = parseKeywords(tokenSet, idx, ['NOT','NULL']);
  if (found) return [idx, new colOpts.NotNull()];
  [idx, found] = parseKeyword(tokenSet, idx, 'NULL');
  if (found) return [idx, new colOpts.Null()];
  [idx, found] = parseKeyword(tokenSet, idx, 'DEFAULT');
  if (found) {
    const [,expr] = [idx] = parseExpr(tokenSet, idx);
    return [idx, new colOpts.Default(expr)];
  }
  [idx, found] = parseKeywords(tokenSet, idx, ['PRIMARY','KEY']);
  if (found) return [idx, new colOpts.Unique(true)];
  [idx, found] = parseKeyword(tokenSet, idx, 'UNIQUE');
  if (found) return [idx, new colOpts.Unique(false)];
  [idx, found] = parseKeyword(tokenSet, idx, 'REFERENCES');
  if (found) {
    const [,foreignTable] = [idx] = parseObjectName(tokenSet, idx);
    const [,referredColumns] = [idx] = parseParenthesizedColumnList(tokenSet, idx, true);
    let onDelete: ReferencialAction|undefined, onUpdate: ReferencialAction|undefined;
    do {
      if (!onDelete) {
        [idx, found] = parseKeywords(tokenSet, idx, ['ON','DELETE']);
        if (found) [,onDelete] = [idx] = parseReferencialAction(tokenSet, idx);
      }
      if (!onUpdate) {
        [idx, found] = parseKeywords(tokenSet, idx, ['ON','UPDATE']);
        if (found) [,onUpdate] = [idx] = parseReferencialAction(tokenSet, idx);
      }
    } while ((!onDelete || !onUpdate) && found);
    return [idx, new colOpts.Foreign(foreignTable, referredColumns, onDelete, onUpdate)];
  }
  [idx, found] = parseKeyword(tokenSet, idx, 'CHECK');
  if (found) {
    idx = expectToken(tokenSet, idx, tk.LPAREN);
    const [,expr] = [idx] = parseExpr(tokenSet, idx);
    idx = expectToken(tokenSet, idx, tk.RPAREN);
    return [idx, new colOpts.Check(expr)];
  }
  [idx, found] = parseKeyword(tokenSet, idx, 'AUTO_INCREMENT');
  if (found) return [idx, new colOpts.DiarectSpecific('AUTO_INCREMENT')];
  [idx, found] = parseKeyword(tokenSet, idx, 'AUTOINCREMENT');
  if (found) return [idx, new colOpts.DiarectSpecific('AUTOINCREMENT')];
  return [idx, undefined];
};
const parseReferencialAction = (tokenSet: TokenSet, start: number): ParseResult<ReferencialAction> => {
  let idx=start;
  let found:boolean;
  [idx, found] = parseKeyword(tokenSet, idx, 'RESTRICT');
  if (found) return [idx, new ReferencialAction('RESTRICT')];
  [idx, found] = parseKeyword(tokenSet, idx, 'CASCADE');
  if (found) return [idx, new ReferencialAction('CASCADE')];
  [idx, found] = parseKeywords(tokenSet, idx, ['SET','NULL']);
  if (found) return [idx, new ReferencialAction('SET NULL')];
  [idx, found] = parseKeywords(tokenSet, idx, ['NO','ACTION']);
  if (found) return [idx, new ReferencialAction('NO ACTION')];
  [idx, found] = parseKeywords(tokenSet, idx, ['SET','DEFAULT']);
  if (found) return [idx, new ReferencialAction('SET DEFAULT')];
  throw unexpectedToken(tokenSet, idx, 'one of RESTRICT, CASCADE, SET NULL, NO ACTION or SET DEFAULT');
};
const parseOptionalTableConstraint = (tokenSet: TokenSet, start: number): ParseResult<TableConstraint|undefined> => {
  let idx=start;
  const [,found] = [idx] = parseKeyword(tokenSet, idx, 'CONSTRAINT');
  if (!found) return [idx, undefined];
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
      return [idx, new Check(name, expr)];
    }
  }
  throw unexpectedToken(tokenSet, idx, 'PRIMARY, UNIQUE, FOREIGN, or CHECK');
};
const PRECEDENCE = {
  DEFAULT_0: 0,
  OR_5: 5,
  AND_10: 10,
  UNARY_NOT_15: 15,
  IS_17: 17,
  BETWEEN_20: 20,
  EQ_20: 20,
  PIPE_21: 21,
  CARET_22: 22,
  AMPERSAND_23: 23,
  PLUS_MINUS_30: 30,
  MULT_40: 40,
  DOUBLE_COLON_50: 50,
} as const;
type Precedence = typeof PRECEDENCE[keyof typeof PRECEDENCE]; // values of PRECEDENCE
const parseExpr = (tokenSet: TokenSet, start: number, precedence: Precedence = PRECEDENCE.DEFAULT_0): ParseResult<Expr> => {
  let idx=start;
  let [, expr] = [idx] = parsePrefix(tokenSet, idx);
  for(;;) {
    const nextPrecedence = getNextPrecedence(tokenSet, idx);
    if (precedence >= nextPrecedence) break;
    [idx,expr] = parseInfix(tokenSet, idx, expr, nextPrecedence);
  }
  return [idx, expr];
};
const getNextPrecedence = (tokenSet: TokenSet, start: number): Precedence => {
  let idx=start;
  let [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if (token instanceof tk.Word) {
    if (keywords.isKeyword(token.value, 'OR')) return PRECEDENCE.OR_5;
    if (keywords.isKeyword(token.value, 'AND')) return PRECEDENCE.AND_10;
    if (keywords.isKeyword(token.value, 'NOT')) {
      [,token] = nextMeaningfulToken(tokenSet, idx);
      if (inKeywords(token, ['IN','BETWEEN','LIKE'])) return PRECEDENCE.BETWEEN_20;
      return PRECEDENCE.DEFAULT_0;
    }
    if (inKeywords(token, ['IN','BETWEEN','LIKE'])) return PRECEDENCE.BETWEEN_20;
    if (keywords.isKeyword(token.value, 'IS')) return PRECEDENCE.IS_17;
  }
  if (token instanceof Token) {
    if ([tk.EQ,tk.LT,tk.LTEQ,tk.NEQ,tk.NEQ2,tk.GT,tk.GTEQ].includes(token)) return PRECEDENCE.EQ_20;
    if (tk.PIPE === token) return PRECEDENCE.PIPE_21;
    if ([tk.CARET,tk.HASH,tk.SHIFT_RIGHT,tk.SHIFT_LEFT].includes(token)) return PRECEDENCE.CARET_22;
    if (tk.AMPERSAND === token) return PRECEDENCE.AMPERSAND_23;
    if ([tk.PLUS,tk.MINUS].includes(token)) return PRECEDENCE.PLUS_MINUS_30;
    if ([tk.MULT,tk.MOD,tk.DIV,tk.CONCAT].includes(token)) return PRECEDENCE.MULT_40;
    if ([tk.DOUBLE_COLON,tk.EXCLAMATION_MARK].includes(token)) return PRECEDENCE.DOUBLE_COLON_50;
  }
  return PRECEDENCE.DEFAULT_0;
};
const parsePrefix = (tokenSet: TokenSet, start: number): ParseResult<Expr> => {
  let idx=start;
  const [,dataType] = [idx] = tryParseDataType(tokenSet, idx);
  if (dataType instanceof DataType) {
    const [,value] = [idx] = expectLiteralString(tokenSet, idx);
    return [idx, new exprs.TypedString(dataType, value)];
  }
  const prevIdx = idx;
  let [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  const [,expr] = [idx] = ((): ParseResult<Expr> => {
    if (token instanceof tk.Word) { // check if valid keyword
      const word = token;
      if (keywords.isOneOfKeywords(word.value, ['TRUE','FALSE','NULL'])) {
        return expectValue(tokenSet, prevIdx);
      } else if (keywords.isKeyword(word.value, 'CASE')) {
        return parseCase(tokenSet, idx);
      } else if (keywords.isKeyword(word.value, 'CAST')) {
        return parseCast(tokenSet, idx);
      } else if (keywords.isKeyword(word.value, 'EXISTS')) {
        idx = expectToken(tokenSet, idx, tk.LPAREN);
        return skipToClosingParen(tokenSet, idx, new exprs.Exists);
      } else if (keywords.isKeyword(word.value, 'EXTRACT')) {
        idx = expectToken(tokenSet, idx, tk.LPAREN);
        return skipToClosingParen(tokenSet, idx, new exprs.Extract);
      } else if (keywords.isKeyword(word.value, 'INTERVAL')) {
        throw unsupportedExprssion(peekToken(tokenSet, idx));
      } else if (keywords.isKeyword(word.value, 'LISTAGG')) {
        idx = expectToken(tokenSet, idx, tk.LPAREN);
        return skipToClosingParen(tokenSet, idx, new exprs.ListAgg);
      } else if (keywords.isKeyword(word.value, 'NOT')) {
        const [,expr] = [idx] = parseExpr(tokenSet, idx);
        return [idx, new exprs.UnaryOp(new ops.UnaryOperator('NOT'), expr)];
      }
    }
    if (token instanceof tk.WordIdent) { // here the token was not a keyword so it is an identifier
      const word = token;
      token = peekToken(tokenSet, idx);
      if (token === tk.LPAREN || token === tk.PERIOD) {
        const idents = [toIdent(word)];
        let endWithWildcard = false;
        let optIdx;
        while ((optIdx = consumeToken(tokenSet, idx, tk.PERIOD))) {
          const [,token] = [idx] = nextMeaningfulToken(tokenSet, optIdx);
          if (token instanceof tk.WordIdent) {
            idents.push(toIdent(token));
          } else if (token === tk.MULT) {
            endWithWildcard = true;
            break;
          }
          throw unexpectedToken(tokenSet, idx, `an identifier or a '*' after '.'`);
        }
        if (endWithWildcard) {
          return [idx, new exprs.QualifiedWildcard(idents)];
        }
        optIdx = consumeToken(tokenSet, idx, tk.LPAREN);
        if (optIdx) {
          // here we expect a function but we not support it. so just skip to the function end.
          return skipToClosingParen(tokenSet, idx, new exprs.Function);
        }
        return [idx, new exprs.CompoundIdentifier(idents)];
      }
      return [idx, new exprs.Identifier(toIdent(word))];
    }
    if (token === tk.MULT) return [idx, new exprs.Wildcard];
    if (token === tk.PLUS || token == tk.MINUS) {
      const [,expr] = [idx] = parseExpr(tokenSet, idx, PRECEDENCE.PLUS_MINUS_30);
      return [idx, new exprs.UnaryOp(new ops.UnaryOperator(token.value), expr)];
    }
    if (token instanceof tk.Number || token instanceof tk.StringLiteral) {
      return expectValue(tokenSet, prevIdx);
    }
    if (token === tk.LPAREN) {
      return skipToClosingParen(tokenSet, idx, new exprs.Subquery);
    }
    throw unexpectedToken(tokenSet, idx, 'an expression');
  })();
  const [,found] = [idx] = parseKeyword(tokenSet, idx, 'COLLATE');
  if (found) {
    const [,collate] = [idx] = parseObjectName(tokenSet, idx);
    return [idx, new exprs.Collate(expr, collate)];
  }
  return [idx, expr];
};
function skipToClosingParen<T>(tokenSet: TokenSet, start: number, dummy: T): ParseResult<T> { // skip unsupported expressions to the closing parenthesis
  let lparenCounter=1;
  let idx=start;
  do {
    const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
    if (token === tk.RPAREN) lparenCounter--;
    else if (token === tk.LPAREN) lparenCounter++;
    else if (token instanceof Eof) throw unexpectedToken(tokenSet, idx, `')'`);
  } while(lparenCounter);
  return [idx, dummy];
}
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
  const conditions: Expr[] = [];
  const results: Expr[] = [];
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
  if (found) [,elseResult] = parseExpr(tokenSet, idx);
  idx = expectKeyword(tokenSet, idx, 'END');
  return [idx, new exprs.Case(operand, conditions, results, elseResult)];
};
const expectValue = (tokenSet: TokenSet, start: number): ParseResult<Value<boolean|string|undefined>> => {
  let idx=start;
  const [,token] = [idx] =nextMeaningfulToken(tokenSet, idx);
  if (token instanceof tk.Word) {
    if (keywords.isOneOfKeywords(token.value, ['TRUE','FALSE'])) return [idx, new values.Boolean(token.value)];
    else if (keywords.isKeyword(token.value, 'NULL')) return [idx, new values.Null];
    throw unexpectedToken(tokenSet, idx, 'a concrete value');
  }
  if (token instanceof tk.Number) {
    return [idx, new values.Number(token.value)]; // TODO should validate?
  } else if (token instanceof tk.SingleQuotedString) {
    return [idx, new values.SingleQuotedString(token.content)];
  } else if (token instanceof tk.NationalStringLiteral) {
    return [idx, new values.NationalStringLiteral(token.content, token.prefix)];
  } else if (token instanceof tk.HexStringLiteral) {
    return [idx, new values.HexStringLiteral(token.content, token.prefix)];
  }
  throw unexpectedToken(tokenSet, idx, 'a value');
};
const expectLiteralString = (tokenSet: TokenSet, start: number): ParseResult<string> => {
  let idx=start;
  const [,token] = [idx] =nextMeaningfulToken(tokenSet, idx);
  if (token instanceof tk.SingleQuotedString) return [idx, token.content];
  throw unexpectedToken(tokenSet, idx, 'literal string');
};
const tryParseDataType = (tokenSet: TokenSet, start: number): ParseResult<undefined|DataType> => {
  const idx=start;
  try {
    return parseDataType(tokenSet, idx);
  } catch(err) {
    if (err instanceof ParseError
      || 'parseError' in err // ts-node(ts-jest) incorrectly return false even if err is an instance of ParseError. so we need an extra route for it.
    ) return [idx, undefined];
    throw err;
  }
};
const parseDataType = (tokenSet: TokenSet, start: number): ParseResult<DataType> => {
  let idx=start;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if (token instanceof tk.Word){
    const typeName = token.value.toUpperCase();
    if (types.inDataTypeNameL(typeName)) {
      const [,length] = [idx] = parseLength(tokenSet, idx);
      // TODO support [CHARACTER SET], [COLLATE]
      return [idx, types.mapperL[typeName](length)];
    } else if (types.inDataTypeNameOptPS(typeName)) {
      const [,ps] = [idx] = parseOptionalPrecisionScale(tokenSet, idx);
      if (ps instanceof Array) return [idx, types.mapperOptPS[typeName](...ps)];
      else return [idx, types.mapperOptPS[typeName]()];
    } else if (types.inDataTypeNameOptP(typeName)) {
      const [,p] = [idx] = parseOptionalPrecision(tokenSet, idx);
      return [idx,types.mapperOptP[typeName](typeof p === 'number' ? p : undefined)];
    } else if (equalToKeyword(token, 'DOUBLE')) {
      const [,found] = [idx] = parseKeyword(tokenSet, idx, 'PRECISION');
      let dbl = typeName;
      if (found) dbl += ' PRECISION'; // TODO keyword
      return [idx, types.mapperNoArgs[dbl as types.DataTypeNameNoArgs]]; // TODO type assertion
    } else if (inKeywords(token, ['TIME', 'TIMESTAMP'])) {
      const withOrWithout = (['WITH','WITHOUT'] as const).some(word => {
        const [,found] = [idx] = parseKeyword(tokenSet, idx, word);
        return found;
      });
      if (withOrWithout) idx = expectKeywords(tokenSet, idx, ['TIME', 'ZONE']);
      return [idx,types.mapperNoArgs[typeName as types.DataTypeNameNoArgs]]; // TODO type assertion
    } else if (types.inDataTypeNameNoArgs(typeName)) {
      return [idx,types.mapperNoArgs[typeName]];
    }
  }
  throw unexpectedToken(tokenSet, idx, 'a data type name');
};
const parseLength = (tokenSet: TokenSet, start: number): ParseResult<number> => {
  let idx=start;
  idx = expectToken(tokenSet, idx, tk.LPAREN);
  const [,length] = [idx] = parseLiteralUint(tokenSet, idx);
  return [expectToken(tokenSet, idx, tk.RPAREN), length];
};
const parseOptionalPrecisionScale = (tokenSet: TokenSet, start: number): ParseResult<undefined|[number,number]> => {
  const idx=start;
  const optIdx = consumeToken(tokenSet, idx, tk.LPAREN);
  if (optIdx) {
    let idx;
    const [,precision] = [idx] = parseLiteralUint(tokenSet, optIdx);
    idx = expectToken(tokenSet, idx, tk.COMMA);
    const [,scale] = [idx] = parseLiteralUint(tokenSet, idx);
    return [expectToken(tokenSet, idx, tk.RPAREN), [precision, scale]];
  }
  return [idx, undefined];
};
const parseOptionalPrecision = (tokenSet: TokenSet, start: number): ParseResult<undefined|number> => {
  const idx=start;
  const optIdx = consumeToken(tokenSet, idx, tk.LPAREN);
  if (optIdx) {
    const [idx,precision] = parseLiteralUint(tokenSet, optIdx);
    return [expectToken(tokenSet, idx, tk.RPAREN), precision];
  }
  return [idx, undefined];
};
const parseLiteralUint = (tokenSet: TokenSet, start: number): [number,number] => {
  let idx=start;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if (token instanceof tk.Number) return [idx, parseInt(token.value)];
  throw unexpectedToken(tokenSet, idx, 'literal int');
};

const parseInfix = (tokenSet: TokenSet, start: number, expr: Expr, precedence: Precedence): ParseResult<Expr> => {
  let idx=start;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if (token instanceof Eof) throw new ParseError('unexpected EOF while parsing infix');  // Can only happen if `getNextPrecedence` got out of sync with this function
  logger.log('infix', token);
  if (token instanceof tk.Operator /* TODO precise condition  */ || inKeywords(token, ['AND','OR','LIKE'])) {
    const [,right] = [idx] = parseExpr(tokenSet, idx);
    return [idx, new exprs.BinaryOp(expr, new ops.BinaryOperator(token.value), right)];
  }
  const [,found] = [idx] = parseKeywords(tokenSet, idx, ['NOT', 'LIKE']);
  if(found) {
    const [,right] = [idx] = parseExpr(tokenSet, idx, precedence);
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
    throw unexpectedToken(tokenSet, idx, 'IN or BETWEEN after NOT');
  }
  throw new ParseError('No infix parser for token '+token.value); // Can only happen if `getNextPrecedence` got out of sync with this function
};
const parseBetween = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): ParseResult<exprs.Between> => {
  let idx=start;
  const [,low] = [idx] = parseExpr(tokenSet, idx, PRECEDENCE.BETWEEN_20);
  idx = expectKeyword(tokenSet, idx, 'AND');
  const [,high] = [idx] = parseExpr(tokenSet, idx, PRECEDENCE.BETWEEN_20);
  return [idx, new exprs.Between(expr, negated, low, high)];
};
const parseIn = (tokenSet: TokenSet, start: number, expr: Expr, negated: boolean): ParseResult<exprs.InList> => {
  let idx=start;
  idx = expectToken(tokenSet, idx, tk.LPAREN);
  const [,found] = [idx] = parseKeywords(tokenSet, idx, ['SELECT','WITH']);
  if (found) { // subquery is not supported
    throw unexpectedToken(tokenSet, idx, 'columns (subquery is not supported)');
  }
  const [,exs] = [idx] = parseCommaSeparated(tokenSet, idx, parseExpr);
  return [expectToken(tokenSet, idx, tk.RPAREN), new exprs.InList(expr, exs, negated)];
};
const parseParenthesizedColumnList = (tokenSet: TokenSet, start: number, isOptional: boolean): ParseResult<Ident[]> => {
  const idx=start;
  const optIdx = consumeToken(tokenSet, idx, tk.LPAREN);
  if (optIdx) {
    const [idx, idents] = parseCommaSeparated(tokenSet, optIdx, parseIdentifier);
    return [expectToken(tokenSet, idx, tk.RPAREN), idents];
  } else if (isOptional) {
    return [idx, []];
  } else {
    throw unexpectedToken(tokenSet, idx, 'a list of columns in parentheses');
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
  let idx;
  let optIdx: number|undefined =start;
  do {
    const [,ident] = [idx] = parseIdentifier(tokenSet, optIdx);
    idents.push(ident);
    optIdx = consumeToken(tokenSet, idx, tk.PERIOD);
  } while (optIdx);
  return [idx, new ObjectName(idents)];
};
const parseIdentifier = (tokenSet: TokenSet, start: number): ParseResult<Ident> => {
  let idx=start;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if(token instanceof tk.WordIdent) return [idx, toIdent(token)];
  throw unexpectedToken(tokenSet, idx, 'identifier');
};
const expectKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): number => {
  const idx=start;
  const token = peekToken(tokenSet, idx);
  if (equalToKeyword(token, keyword)) return nextMeaningfulToken(tokenSet, idx)[0];
  throw unexpectedToken(tokenSet, idx, keyword);
};
const expectKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): number => {
  let idx = start;
  keywords.forEach(keyword => idx = expectKeyword(tokenSet, idx, keyword));
  return idx;
};
const parseKeyword = (tokenSet: TokenSet, start: number, keyword: Keyword): ParseResult<boolean> => {
  const idx=start;
  const token = peekToken(tokenSet, idx);
  return equalToKeyword(token, keyword) ? [nextMeaningfulToken(tokenSet, idx)[0], true] : [idx, false];
};
const parseKeywords = (tokenSet: TokenSet, start: number, keywords: Keyword[]): ParseResult<boolean> => {
  let idx=start;
  for (let j=0; idx<tokenSet.length && j<keywords.length; j++) {
    const [,found] = [idx] = parseKeyword(tokenSet, idx, keywords[j]);
    if(!found) return [idx, false];
  }
  return [idx, true];
};
const expectToken = (tokenSet: TokenSet, start: number, expected: Token): number => {
  let idx=start;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if (token instanceof Token && token.value === expected.value) return idx;
  throw unexpectedToken(tokenSet, idx, expected.value);
};
const consumeToken = (tokenSet: TokenSet, start: number, consumedToken: Token): number|undefined => {
  let idx=start;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  if(token instanceof Token && token.value === consumedToken.value) return idx;
};
type NextTokenResult = Token|Eof
const nextMeaningfulToken = (tokenSet: TokenSet, start: number): ParseResult<NextTokenResult> => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof tk.Whitespace) i++;
  return tokenSet.length<=i ? [Infinity, EOF] : [i+1, tokenSet[i]]; // the returning idx is 1 ahead from the returning token.
};
const peekToken = (tokenSet: TokenSet, start: number): Token|Eof => {
  let i=start;
  while(i<tokenSet.length && tokenSet[i] instanceof tk.Whitespace) i++;
//  logger.trace(9);
  return tokenSet.length<=i ? EOF : tokenSet[i];
};
const equalToKeyword = (token: Token|Eof, keyword: Keyword): boolean => token instanceof tk.Word && token.value.toLocaleUpperCase() === keyword;
const inKeywords = (token: Token|Eof, keywords: Keyword[]): boolean => keywords.some(keyword => equalToKeyword(token, keyword)); // TODO delete
const unexpectedToken = (tokenSet: TokenSet, start: number, expected: string): ParseError => {
  let idx=start;
  const [,token] = [idx] = nextMeaningfulToken(tokenSet, idx);
  const [row,col] = tk.getTokenLocation(tokenSet, idx--);
  return new ParseError(`Expected: ${expected}, but found: ${token.value}. (row: ${row}, column: ${col})`);
};
const unsupportedExprssion = (token: Token|Eof): ParseError => new ParseError(`An unsupported expression found: ${token.value}.`);
const toIdent = (wordOrIdent: tk.WordIdent): Ident => // TODO exclude keyword
  wordOrIdent instanceof tk.DelimitedIdent ? new Ident(wordOrIdent.content,wordOrIdent.delimiter) : new Ident(wordOrIdent.value);

export * from './parser';
export * as parser from './parser';

