class Token {
  constructor(private _value: string) { }
  get value(): string { return this.value; }
  get length(): number { return this.value.length; }
}
class Whitespace extends Token {};
class Word extends Token {};
class SingleQuotedString extends Token {};
class NationalStringLiteral extends Token {};
class HexStringLiteral extends Token {};
class Num extends Token {};
class LParen extends Token {};
class RParen extends Token {};
class Comma extends Token {};
class Cmmnt extends Token {};
class Operator extends Token {};
class Other extends Token {};

const SPACE = new Whitespace(' ');
const TAB = new Whitespace('\t');
const LF = new Whitespace('\n');
const CRLF = new Whitespace('\r\n');
const CR = new Whitespace('\r');
const LPAREN = new LParen('(');
const RPAREN = new RParen(')');
const COMMA = new Comma(',');
const CONCAT = new Operator('||');
const EQ = new Operator('=');
const RARROW = new Operator('=>');
const PERIOD = new Operator('.');
const PLUS = new Operator('+');
const MINUS = new Operator('-');
const DIV = new Operator('/');
const MULT = new Operator('*');
const MOD = new Operator('%');
const NEQ = new Operator('!=');
const DOUBLE_EXCLAMATION_MARK = new Operator('!!');
const EXCLAMATION_MARK = new Operator('!');
const LT = new Operator('<');
const LTEQ = new Operator('<=');
const NEQ2 = new Operator('<>');
const SHIFT_LEFT = new Operator('<<');
const GT = new Operator('>');
const GTEQ = new Operator('>=');
const SHIFT_RIGHT = new Operator('>>');
const COLON = new Operator(':');
const DOUBLE_COLON = new Operator('::');
const SEMICOLON = new Operator(';');
const BACKSLASH = new Operator('\\');
const LBRACKET = new Operator('[');
const RBRACKET = new Operator(']');
const AMPERSAND = new Operator('&');
const CARET = new Operator('^');
const LBRACE = new Operator('{');
const RBRACE = new Operator('}');
const TILDE = new Operator('~');
const HASH = new Operator('#');
const ATSIGN = new Operator('@');


const TOKENIZE_SINGLE_QUOTED_STRING_ERROR = 'Unterminated string literal';
const TOKENIZE_DELIMITED_STRING_ERROR = (delimiter => `Expected close delimiter ${delimiter} before EOF.`)('"');
const TOKENIZE_MULTI_LINE_COMMENT_ERROR = 'Unexpected EOF while in a multi-line comment';

type Rule = { [ch: string]: (chars: string[]) => Token };
const rule: Rule = {
  // whitespaces
  ' ': () => SPACE,
  '\t': () => TAB,
  '\n': () => LF,
  '\r': chars => chars[1] == '\n' ? CRLF : CR,
  // national string
  'N': chars => chars[1] === '\'' ? new NationalStringLiteral(tokenizeSingleQuotedString(chars)) : new Word(tokenizeIdentifier(chars)),
  // hex string
  'X': chars => chars[1] === '\'' ? new HexStringLiteral(tokenizeSingleQuotedString(chars)) : new Word(tokenizeIdentifier(chars)),
  'x': chars => chars[1] === '\'' ? new HexStringLiteral(tokenizeSingleQuotedString(chars)) : new Word(tokenizeIdentifier(chars)),
  // identifier or keyword
  ident: chars => new Word(tokenizeIdentifier(chars)),
  // string
  '\'': chars => new SingleQuotedString(tokenizeSingleQuotedString(chars)),
  // delimited (quoted) identifier
  delimitedIdent: chars => new Word(takeWhileOrError(chars, 1, c => !isDelimiteddIdentifierStart(c), TOKENIZE_DELIMITED_STRING_ERROR)),
  // number
  num: chars => new Num(takeWhile(chars, 1, c => (isDigit(c) || c== '.'))),
  // puctuations
  '(': () => LPAREN,
  ')': () => RPAREN,
  ',': () => COMMA,
  // operators
  '-': chars => chars[1] === '-' ? new Cmmnt(takeWhile(chars, 2, c => c === '\n'  /* TODO only LF? */ )) : DIV,
  '/': chars => chars[1] === '*' ? new Cmmnt(takeWhileOrError(chars, 2, (c,i,chars) => c === '*' && chars[i+1] === '/', TOKENIZE_MULTI_LINE_COMMENT_ERROR)) : DIV,
  '|': chars => chars[1] === '|' ? CONCAT : new Other('|'),
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
  '#':  () => HASH,
  '@':  () => ATSIGN,
};

function tokenize(src: string) {
  const chars = Array.from(src);
  let i=0;
  while(i < chars.length) {
    const c = chars[i];
    const key = isIdentifierStart(c) ? 'ident' :
                isDigit(c) ? 'num' :
                isDigit(c) ? 'num' :
                c;
    const token = rule[key]?.(chars);
  }
}
const min = (...args: number[]): number => args.sort((a,b) => a-b)[0];
const takeWhile = (chars: string[], testStart: number, predicate: (ch: string, idx: number, chars: string[]) => boolean): string => {
  let i=testStart;
  while(i < chars.length && predicate(chars[i], i, chars)) i++;
  return chars.slice(0, min(i+1, chars.length)).join();
}
const takeWhileOrError = (chars: string[], testStart: number, predicate: (ch: string, idx: number, chars: string[]) => boolean, msg: string): string => {
  let i=testStart;
  while(i < chars.length && predicate(chars[i], i, chars)) i++;
  if (i = chars.length) throw new Error(msg);
  return chars.slice(0, i+1).join();
}
const isDigit = (ch: string): boolean => '0' < ch && ch < '9';
const isDelimiteddIdentifierStart = (ch: string): boolean => ['"'].includes(ch);
const isIdentifierStart = (ch: string): boolean => ['@', '#', '_'].includes(ch) ||  ('A' < ch && ch < 'Z' ) || ('a' < ch && ch < 'z');
const isIdentifierPart = (ch: string): boolean => ['@', '$', '#', '_'].includes(ch) || ('0' < ch && ch < '9') || ('A' < ch && ch < 'Z' ) || ('a' < ch && ch < 'z');
const tokenizeSingleQuotedString = (chars: string[]): string => takeWhileOrError(chars, 1, (c,i,chars) => c === '\'' && chars[i+1] !== '\'', TOKENIZE_SINGLE_QUOTED_STRING_ERROR);
const tokenizeIdentifier = (chars: string[]): string => takeWhile(chars, 1, c => isIdentifierPart(c));
// function tokenizeNumber(chars: string[]): string {
//   let i=1;
//   while((isDigit(chars[i]) || chars[i] == '.') && i<chars.length) i++;
//   return chars.slice(0,i).join();
// }
// function tokenizeWord(chars: string[]): string {
//   let i=1;
//   while(isIdentifierPart(chars[i]) && i<chars.length) i++;
//   return chars.slice(0,i).join();
// }
//     fn is_identifier_start(&self, ch: ch) -> bool {
//         ('a'..='z').contains(&ch)
//             || ('A'..='Z').contains(&ch)
//             || ch == '_'
//             || ch == '#'
//             || ch == '@'
//     }
// 
//     fn is_identifier_part(&self, ch: ch) -> bool {
//         ('a'..='z').contains(&ch)
//             || ('A'..='Z').contains(&ch)
//             || ('0'..='9').contains(&ch)
//             || ch == '@'
//             || ch == '$'
//             || ch == '#'
//             || ch == '_'
//     }
