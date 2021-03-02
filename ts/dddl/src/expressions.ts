import { DataType } from './data-types';
import { BinaryOperator, UnaryOperator } from './operators';
import { Ident, ObjectName } from './parser';

export class Expr {
  private _expr:void
  constructor() {}
}
export class UnaryOp extends Expr {
  constructor(
    public op: UnaryOperator,
    public expr: Expr,
  ) { super(); }
}
export class BinaryOp extends Expr {
  constructor(
    public left: Expr,
    public op: BinaryOperator,
    public right: Expr,
  ) { super(); }
}
export class IsNull extends Expr {
  constructor(
    public expr: Expr,
  ) { super(); }
}
export class IsNotNull extends Expr {
  constructor(
    public expr: Expr,
  ) { super(); }
}
export class InList extends Expr {
  constructor(
    public expr: Expr,
    public list: Expr[],
    public negated: boolean,
  ) { super(); }
}
export class Between extends Expr {
  constructor(
    public expr: Expr,
    public negated: boolean,
    public low: Expr,
    public high: Expr,
  ) { super(); }
}
export class TypedString extends Expr {
  constructor(
    public dataType: DataType,
    public value: string,
  ) { super(); }
}
export class Case extends Expr {
  constructor(
    public operand: undefined|Expr,
    public conditions: Expr[],
    public results: Expr[],
    public elseResult: undefined|Expr,
  ) { super(); }
}
export class Cast extends Expr {
  constructor(
    public expr: Expr,
    public dataType: DataType,
  ) { super(); }
}
export class QualifiedWildcard extends Expr {
  constructor(
    public idents: Ident[],
  ) { super(); }
}
export class CompoundIdentifier extends Expr {
  constructor(
    public idents: Ident[],
  ) { super(); }
}
export class Identifier extends Expr {
  constructor(
    public ident: Ident,
  ) { super(); }
}
export class Wildcard extends Expr {
  constructor(
  ) { super(); }
}
export class Collate extends Expr {
  constructor(
    public expr: Expr,
    public collate: ObjectName,
  ) { super(); }
}
export class Function extends Expr {
}
export class Exists extends Expr {
}
export class Extract extends Expr {
}
export class ListAgg extends Expr {
}
export class Subquery extends Expr {
}

export * as exprs from './expressions';
export * from './expressions';

