import { DataType } from '../data-types';
import { BinaryOperator, UnaryOperator } from './operators';

export interface Expr {}
export class UnaryOp implements Expr {
  constructor(
    public op: UnaryOperator,
    public expr: Expr,
  ) {}
}
export class BinaryOp implements Expr {
  constructor(
    public left: Expr,
    public op: BinaryOperator,
    public right: Expr,
  ) {}
}
export class IsNull implements Expr {
  constructor(
    public expr: Expr,
  ) {}
}
export class IsNotNull implements Expr {
  constructor(
    public expr: Expr,
  ) {}
}
export class InList implements Expr {
  constructor(
    public expr: Expr,
    public list: Expr[],
    public negated: boolean,
  ) {}
}
export class Between implements Expr {
  constructor(
    public expr: Expr,
    public negated: boolean,
    public low: Expr,
    public high: Expr,
  ) {}
}
export class TypedString implements Expr {
  constructor(
    public dataType: DataType,
    public value: string,
  ) {}
}
export class Case implements Expr {
  constructor(
    public operand: undefined|Expr,
    public conditions: Expr[],
    public results: Expr[],
    public elseResult: undefined|Expr,
  ) {}
}
export class Cast implements Expr {
  constructor(
    public expr: Expr,
    public dataType: DataType,
  ) {}
}

