import { Expr } from './expressions';

export interface Value<T> extends Expr {
  value: T
}
export class Number implements Value<string> { // Use string instead of number, in case of big number
  constructor(public value: string) {}
}
export class SingleQuotedString implements Value<string> {
  public value: string
  constructor(public content: string) { this.value = `'` + content + `'`; }
}
export class NationalStringLiteral implements Value<string> {
  constructor(public value: string) {}
}
export class HexStringLiteral implements Value<string> {
  constructor(public value: string) {}
}
export class Boolean implements Value<boolean> {
  public value: boolean
  constructor(_value: 'TRUE'|'FALSE') { this.value = _value.toUpperCase() === 'TRUE'; }
}
export class Null implements Value<undefined> {
  public value: undefined
}

