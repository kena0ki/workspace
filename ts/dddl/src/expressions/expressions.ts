import { types } from '../data-types';
import { BinaryOperator } from './operators';

export interface Expr {}
export class BinaryOp implements Expr {
  private: undefined
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
    public dataType: types.DataType,
    public value: string,
  ) {}
}

