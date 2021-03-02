import { Expr } from './expressions';

export class Value<T> extends Expr {
  constructor(public value: T) { super(); }
}
export class Number extends Value<string> { // Use string instead of number, in case of big number
  constructor(_value: string) { super(_value); }
}
export class SingleQuotedString extends Value<string> {
  constructor(public content: string) { super(`'` + content + `'`); }
}
export class NationalStringLiteral extends Value<string> {
  constructor(public content: string, public prefix: string) { super(`'` + content + `'`); }
}
export class HexStringLiteral extends Value<string> {
  constructor(public content: string, public prefix: string) { super(`'` + content + `'`); }
}
export class Boolean extends Value<boolean> {
  constructor(_value: 'TRUE'|'FALSE') { super(_value.toUpperCase() === 'TRUE'); }
}
export class Null extends Value<undefined> {
  constructor() { super(undefined); }
}

export * as values from './values';
export * from './values';
