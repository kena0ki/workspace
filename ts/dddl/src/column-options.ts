import { ObjectName, Ident, ReferencialAction } from './parser';
import { Expr } from './expressions';

export class ColumnOption {
  private columnoption: never
  constructor() {}
}
export class NotNull extends ColumnOption {}
export class Null extends ColumnOption {}
export class Default extends ColumnOption {
  constructor(public expr: Expr) { super(); }
}
export class Unique extends ColumnOption {
  constructor(public isPrimary: boolean) { super(); }
}
export class Foreign extends ColumnOption {
  constructor(
    public foreignTable: ObjectName,
    public referredColumns: Ident[],
    public onDelete?: ReferencialAction,
    public onUpdate?: ReferencialAction,
  ) { super(); }
}
export class Check extends ColumnOption {
  constructor(public expr: Expr) { super(); }
}
export class DiarectSpecific extends ColumnOption {
  constructor(public name: 'AUTO_INCREMENT'|'AUTOINCREMENT') { super(); }
}

export * from './column-options';
export * as columnOptions from './column-options';

