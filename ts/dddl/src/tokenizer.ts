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

import { logger } from './util';
import { Keyword } from './keywords';

export class Token {
  constructor(private _value: string) { }
  get value(): string { return this._value; }
  get length(): number { return this._value.length; }
  get class(): string { return this.constructor.name; }
}
const equalToKeyword = (token: Token, keyword: Keyword): boolean => token instanceof Word && token.value === keyword;
export const tokenUtil = {
  equalToKeyword,
};
export class Whitespace extends Token {}
export class NewLine extends Whitespace {}
export class WordIdent extends Token {}
export class Word extends WordIdent { // keyword or (delimited) identifier
//  constructor(private _content: string, private _delimiter?: string) { super(_delimiter + _content + _delimiter); }
//  get content(): string { return this._content; }
//  get delimiter(): string|undefined { return this._delimiter; }
}
export class DelimitedIdent extends WordIdent {
  constructor(private _content: string, private _delimiter: string) { super(_delimiter + _content + _delimiter); }
  get content(): string { return this._content; }
  get delimiter(): string { return this._delimiter; }
}
export class StringLiteral extends Token {}
export class SingleQuotedString extends StringLiteral {
  constructor(private _content: string) { super(`'` + _content + `'`); }
  get content(): string { return this._content; }
}
export class Number extends Token {}
export class LParen extends Token {}
export class RParen extends Token {}
export class Comma extends Token {}
export class Cmmnt extends Whitespace {}
export class Operator extends Token {}
export class BinaryOperator extends Operator {}
export class Other extends Token {}

export class NonCharcterStringLiteral extends StringLiteral {
  constructor(private _content: string, private _prefix: string) { super(_prefix + `'` + _content + `'`); }
  get content(): string { return this._content; }
  get prefix(): string { return this._prefix; }
}
export class NationalStringLiteral extends NonCharcterStringLiteral {}
export class HexStringLiteral extends NonCharcterStringLiteral {}

export const SPACE = new Whitespace(' ');
export const TAB = new Whitespace('\t');
export const LF = new NewLine('\n');
export const CRLF = new NewLine('\r\n');
export const CR = new NewLine('\r');
export const LPAREN = new LParen('(');
export const RPAREN = new RParen(')');
export const COMMA = new Comma(',');
export const CONCAT = new Operator('||');
export const EQ = new Operator('=');
export const RARROW = new Operator('=>');
export const PERIOD = new Operator('.');
export const PLUS = new Operator('+');
export const MINUS = new Operator('-');
export const DIV = new Operator('/');
export const MULT = new Operator('*');
export const MOD = new Operator('%');
export const NEQ = new Operator('!=');
export const DOUBLE_EXCLAMATION_MARK = new Operator('!!');
export const EXCLAMATION_MARK = new Operator('!');
export const LT = new Operator('<');
export const LTEQ = new Operator('<=');
export const NEQ2 = new Operator('<>');
export const SHIFT_LEFT = new Operator('<<');
export const GT = new Operator('>');
export const GTEQ = new Operator('>=');
export const SHIFT_RIGHT = new Operator('>>');
export const COLON = new Operator(':');
export const DOUBLE_COLON = new Operator('::');
export const SEMICOLON = new Operator(';');
export const BACKSLASH = new Operator('\\');
export const LBRACKET = new Operator('[');
export const RBRACKET = new Operator(']');
export const AMPERSAND = new Operator('&');
export const CARET = new Operator('^');
export const LBRACE = new Operator('{');
export const RBRACE = new Operator('}');
export const TILDE = new Operator('~');
export const HASH = new Operator('#');
export const ATSIGN = new Operator('@');
export const PIPE = new Operator('|');

const TOKENIZE_SINGLE_QUOTED_STRING_ERROR = 'Unterminated string literal.';
const TOKENIZE_DELIMITED_STRING_ERROR = (delimiter => `Expected close delimiter ${delimiter} before EOF.`)('"');
const TOKENIZE_MULTI_LINE_COMMENT_ERROR = 'Unexpected EOF while in a multi-line comment.';


type Rule = { [ch: string]: (chars: string[]) => Token };
const rule: Rule = {
  // whitespaces
  ' ': () => SPACE,
  '\t': () => TAB,
  '\n': () => LF,
  '\r': chars => chars[1] === '\n' ? CRLF : CR,
  // national string: N'...'
  'N': chars => chars[1] === '\'' ? new NationalStringLiteral(tokenizeSingleQuotedString(chars.slice(1)), chars[0]) : new Word(tokenizeIdentifier(chars)),
  // hex string: X'...' or x'...'. the latter (small x) is PostgreSQL diarect.
  'X': chars => chars[1] === '\'' ? new HexStringLiteral(tokenizeSingleQuotedString(chars.slice(1)), chars[0]) : new Word(tokenizeIdentifier(chars)),
  'x': chars => chars[1] === '\'' ? new HexStringLiteral(tokenizeSingleQuotedString(chars.slice(1)), chars[0]) : new Word(tokenizeIdentifier(chars)),
  // identifier or keyword
  ident: chars => new Word(tokenizeIdentifier(chars)),
  // string
  '\'': chars => new SingleQuotedString(tokenizeSingleQuotedString(chars)),
  // delimited (quoted) identifier
  delimitedIdent: chars => new DelimitedIdent(takeWhileOrError(chars.slice(1), 1, c => ~~(c !== chars[0]), TOKENIZE_DELIMITED_STRING_ERROR), chars[0]),
  // number
  num: chars => new Number(takeWhile(chars, 1, c => ~~(isDigit(c) || c === '.')) ),
  // puctuations
  '(': () => LPAREN,
  ')': () => RPAREN,
  ',': () => COMMA,
  // operators
  '-': chars => chars[1] === '-' ? new Cmmnt(takeWhile(chars, 2, c => ~~(c !== '\n'))) : MINUS,
  '/': chars => chars[1] === '*' ? new Cmmnt(takeWhileOrError(chars, 4, (c,i,chars) => ~~!(chars[i-2] === '*' && chars[i-1] === '/'), TOKENIZE_MULTI_LINE_COMMENT_ERROR)) : DIV,
  '+': () => PLUS,
  '*': () => MULT,
  '%': () => MOD,
  '|': chars => chars[1] === '|' ? CONCAT : PIPE,
  '=': chars => chars[1] === '>' ? RARROW : EQ,
  '.': () => PERIOD,
  '!': chars => chars[1] === '=' ? NEQ : chars[1] === '!' ? DOUBLE_EXCLAMATION_MARK : EXCLAMATION_MARK,
  '<': chars => chars[1] === '=' ? LTEQ : chars[1] === '>' ? NEQ2 : chars[1] === '<' ? SHIFT_LEFT : LT,
  '>': chars => chars[1] === '=' ? GTEQ : chars[1] === '>' ? SHIFT_RIGHT : GT,
  ':': chars => chars[1] === ':' ? DOUBLE_COLON : COLON,
  ';':  () => SEMICOLON,
  '\\': () => BACKSLASH,
  '[':  () => LBRACKET,
  ']':  () => RBRACKET,
  '&':  () => AMPERSAND,
  '^':  () => CARET,
  '{':  () => LBRACE,
  '}':  () => RBRACE,
  '~':  () => TILDE,
  '#':  () => HASH, // TODO dead code? this char is in isIdentifierStart
  '@':  () => ATSIGN, // TODO dead code? this char is in isIdentifierStart
};

class InnerError extends Error {
  public readonly innerError = 'nominal typing';
  constructor(_message:string) { super(_message); }
}
export class TokenizeError extends Error {
  constructor(public readonly cause: InnerError, public readonly row: number, public readonly  col: number){
    super(`Tokenize error at row: ${row}, column: ${col}. ${cause.message}`);
    this.stack = cause.stack;
  }
}

export class TokenSet extends Array<Token> {
  constructor(...a: any) { super(...a); }
  get tokens(): Token[] { return this.map(v=>v); }
  toString = () => {
    const tokens = this.slice(1).reduce(
      (prev, curr) => `${prev},\n  ${curr.constructor.name}: ${JSON.stringify(curr)}`,
      `  ${this[0].constructor.name}: ${JSON.stringify(this[0].value)}`
    );
    return `[\n${tokens}\n]`;
  }
  joinValues = (delim:string = ','): string => this.slice(1).reduce((prev, curr) => `${prev}${delim}${curr.value}`, this[0].value);
}
export const getTokenLocation = (tokenSet: TokenSet, idx: number=Infinity): [row:number,column:number] => {
  const loopEnd=Math.min(tokenSet.length, idx-1);
  let row=1, col=1;
  for(let i=0; i<loopEnd; i++) {
    const token = tokenSet[i];
    if (token instanceof Cmmnt) {
      const lines = token.value.split(`\n`);
      row += lines.length-1;
      col = lines.length-1 > 0 ? 1 : col + lines[lines.length-1].length;
    } else if (token instanceof NewLine) {
      row++;
      col=1;
    } else {
      col += token.length;
    }
  }
  return [row,col];
};

export const tokenize = (src: string): TokenSet|TokenizeError => {
  const chars = Array.from(src);
  const tokenSet: TokenSet = new TokenSet();
  let i=0;
  while(i < chars.length) {
    const c = chars[i];
    const key = ['N','X','x'].includes(c) ? c :
                isIdentifierStart(c) ? 'ident' :
                isDelimitedIdentifierStart(c) ? 'delimitedIdent' :
                isDigit(c) ? 'num' :
                c;
    try {
      const token = rule[key]?.(chars.slice(i)) || new Other(key);
      // logger.log('c:', `|${c}|`);
      // logger.log('token:', `*${token.value}*`);
      i += token.length;
      tokenSet.push(token);
    } catch (err: unknown) {
      if (err instanceof InnerError
        // @ts-ignore: ts-node(ts-jest) incorrectly return false even if err is an instance of InnerError. so we need an extra route for it.
        || 'innerError' in err
      ) {
        const [row, col] = getTokenLocation(tokenSet);
        const e = new TokenizeError(err as InnerError, row, col); // TODO undesirable type assertion due to the ts-node issue above
        logger.log(e.stack);
        return e;
      }
      throw err;
    }
  }
  logger.log(tokenSet);
  return tokenSet;
};
const takeWhile = (chars: string[], testStart: number, advanceBy: (ch: string, idx: number, chars: string[]) => number): string => {
  let i=testStart;
  let by=0;
  while(i < chars.length && (by = advanceBy(chars[i], i, chars)) ) i+=by;
  return chars.slice(0, Math.min(i, chars.length)).join('');
};
const takeWhileOrError = (chars: string[], testStart: number, advanceBy: (ch: string, idx: number, chars: string[]) => number, msg: string): string => {
  let i=testStart;
  let by=0;
  while(i < chars.length && (by = advanceBy(chars[i], i, chars)) ) i+=by;
  if (i === chars.length) throw new InnerError(msg);
  return chars.slice(0, i).join('');
};
const isDigit = (ch: string): boolean => '0' <= ch && ch <= '9';
const delimiters = ['"']; // TODO dialect
const isDelimitedIdentifierStart = (ch: string): boolean => delimiters.includes(ch);
const isIdentifierStart = (ch: string): boolean => ['@', '#', '_'].includes(ch) ||  ('A' <= ch && ch <= 'Z' ) || ('a' <= ch && ch <= 'z');
const isIdentifierPart = (ch: string): boolean => ['@', '$', '#', '_'].includes(ch) || ('0' <= ch && ch <= '9') || ('A' <= ch && ch <= 'Z' ) || ('a' <= ch && ch <= 'z');
const tokenizeSingleQuotedString = (chars: string[]): string => takeWhileOrError(chars.slice(1), 0, tokenizeSingleQuotedStringAdvanceBy, TOKENIZE_SINGLE_QUOTED_STRING_ERROR);
const tokenizeSingleQuotedStringAdvanceBy = (c:string,i:number,chars:string[]):number => c !== '\'' ? 1 : (c === '\'' && chars[i+1] === '\'') ? 2 : 0;
const tokenizeIdentifier = (chars: string[]): string => takeWhile(chars, 1, c => ~~isIdentifierPart(c));

export * from './tokenizer';
export * as token from './tokenizer';

