import { CreateTableStatement, TableConstraint, parser } from './parser';
import { DataType, types } from './data-types';

/** Options for data to be generated. */
export class GeneratorOption {
  /** Output format. Either csv or insert statement. */
  outputFormat: CsvFormat|InsertStatementFormat = new CsvFormat
  /**
   *  Options for each column to determine how they are generated.
   *  The options need to be valid types for corresponding columns.
   *    - Column -             - Option -
   *    Numeric types          -> NumericColumnOption
   *    String types           -> StringColumnOption
   *    Datetime types         -> DatetimeColumnOption
   *  If you want a fixed value for some columns throughout entire data,
   *  you can specify the value using FixedValue.
   */
  columnOptions: { [columnName: string]: NumericColumnOption|StringColumnOption|DatetimeColumnOption|FixedValue } = {}
  /**
   *  
   */
  columnOptionsDefault: NumericColumnOption|StringColumnOption|DatetimeColumnOption[] = []
  /** A function to manipulate row data after it's generated. */
  eachRow?: (rowCount: number, columns: string[], prev: object) => string
  /** The number of rows to be generated. */
  rows: number = 0
  /**
   * How to behave when invalid data is detected. Default: log.
   *   log: continue, but output log.
   *   abort: abort data generation.
   *   ignore: ignore error data and going on.
   */
  errorAction: 'log'|'abort'|'ignore' = 'log'
  public constructor(obj?: GeneratorOption) { // TODO Partial
    if (!obj) return;
    Object.assign(this,obj);
  }
}
/** CsvFormat is used for GneratorOption.outputFormat */
export class CsvFormat {
  /** Delimiter of each column. Default: ',' */
  public readonly delimiter: string = `,`
  /** Quote for each value. Default: '"' */
  public readonly quote: string = `"`
  /** Escape sequence. Default: '"' */
  public readonly escapeSequence: string = `"`
  /** Define options */
  public constructor(obj?: Partial<CsvFormat>) {
    if (!obj) return;
    Object.assign(this,obj);
  }
}
/** InsertStatementFormat is used for GneratorOption.outputFormat */
export class InsertStatementFormat {
  private _insertStatementFormat='nominal'
}
/** NumericColumnOptions is used for GeneratorOption.columnOptions */
export class NumericColumnOption {
  /** How much advance per each row. Default: 1 for integers, 0.1 for decimals */
  public readonly stepBy?: number
  /** Value of the first row. Default: 1 */
  public readonly initialValue: number = 1
  /** Prefix. Default: nothing */
  public readonly prefix: Prefix = ''
  /** Limit of incrementation. Default: depend on the corresponding table data type. */
  public readonly limit: number = Infinity
  /**
   * How to behave when incrementation hits the limit. Default: loop.
   *   loop: back to the initial value and continue to increment
   *   negate: negate the value and continue to increment
   *   keep: stop incrementation and keep the limit value
   */
  public readonly loop: 'loop'|'negate'|'keep' = 'loop'
  public constructor(obj?: Partial<NumericColumnOption>) {
    if (!obj) return;
    Object.assign(this,obj);
  }
}
class IntegerColumnOption extends NumericColumnOption {
  private _integerColumnOption='nominal'
  constructor(option: Partial<NumericColumnOption>) {
    super({ stepBy: 1, ...option });
  }
}
class FloatColumnOption extends NumericColumnOption {
  private _floatColumnOption='nominal'
  constructor(option: Partial<NumericColumnOption>) {
    super({ stepBy: 0.1, ...option });
  }
}
/** StringColumnOption is used for GeneratorOption.columnOptions */
export class StringColumnOption {
  /** Limit of incrementation. Default: depend on the corresponding table data type. */
  public readonly maxLength: number = Infinity
  /** Which measurement unit to use, either char or byte. Default: char */
  public readonly lengthIn: 'char'|'byte' = 'char'
  /** Prefix. Default: a character in a-z, A-Z, depending on the column position */
  public readonly prefix: Prefix = ''
  /** Suffix. Default: nothing */
  public readonly suffix: Prefix = ''
  /**
   * How to behave when incrementation hits the limit. Default: loop.
   *   loop: back to the initial value and continue to increment
   *   keep: stop incrementation and keep the limit value
   */
  public readonly loop?: 'loop'|'keep' = 'loop'
  public constructor(obj?: Partial<StringColumnOption>) {
    if (!obj) return;
    Object.assign(this,obj);
  }
}
class CharColumnOption extends StringColumnOption {
  private _charColumnOption='nominal'
  constructor(option: Partial<StringColumnOption>) {
    super({ lengthIn: 'char', ...option });
  }
}
class BinaryColumnOption extends StringColumnOption {
  private _binaryColumnOption='nominal'
  constructor(option: Partial<StringColumnOption>) {
    super({ lengthIn: 'byte', ...option });
  }
}
/** DatetimeColumnOption is used for GeneratorOption.columnOptions */
export class DatetimeColumnOption {
  private _datetimeColumnOption='nominal'
  /** Value of the first row. Default: the time data generation is started. */
  public readonly initialValue?: string
  /**
   *  DatetimeColumnOption is used for GeneratorOption.columnOptions
   *   Currently this option is not supported yet, so the increment step can't be changed.
   *   The value depends on the table column type.
   *     Date type      : 1 day.
   *     Timestamp type : 1 day.
   *     Time           : 1 second.
   */
  protected stepBy?: number = 1
}
/** FixedValue is use for GneratorOption.columnOptions */
export class FixedValue {
  private _fixedValue='nominal'
  constructor(
    /** The value to be set to a column */
    public readonly value: string = '',
  ) {}
}
type AvailableOptionUnion = IntegerColumnOption|FloatColumnOption|CharColumnOption|BinaryColumnOption|DatetimeColumnOption;
type OnlyKeysOfType<T,O> = Extract<keyof O, {[K in keyof O]: O[K] extends T|undefined ? K : never}[keyof O]>
type OnlyOfType<T,O> = {[K in OnlyKeysOfType<T,O>]?: O[K]};
function isOfTypeNumber<K>(key: keyof K): key is OnlyKeysOfType<number,K> { return typeof key === 'number'; }
function isOfTypeString<K>(key: keyof K): key is OnlyKeysOfType<string,K> { return typeof key === 'string'; }
type Prefix = string|((row: number, col: number, colName: string) => string);

export class GenerateError extends Error {
}
/**
 * Generates data from a create table statement with options.
 * How data is generated depends on types and options but the general idea is simple.
 * Generator adds 1 to previous data row by row so each column would have sequentially incremented number.
 * Given that we have the following ddl,
 *    create table a (
 *      c1 char(5),
 *      c2 integer,
 *      c3 float,
 *      c4 binary(8)
 *    );
 * then we would get the following.
 *       c1      c2  c3    c4
 *    L1 "a0001","1","0.1","b0000001"
 *    L2 "a0002","2","0.2","b0000002"
 *    L3 "a0003","3","0.3","b0000003"
 *    L4 "a0004","4","0.4","b0000004"
 */
export const generate = (statement: CreateTableStatement, options: GeneratorOption) => {
  const columns = statement.columns;
  const nameToColIdx = {};
  type OptsType = { [key:string]: AvailableOptionUnion };
  const opts: OptsType = columns.reduce<OptsType>((prev, curr) => {
    const colOption = options.columnOptions[curr.name.value];
    if (curr.dataType instanceof types.NumericType) {
      if (!(colOption instanceof NumericColumnOption)) throw new GenerateError;
      const isFloat = curr.dataType instanceof types.Float || (curr.dataType instanceof types.DecimalType && curr.dataType.scale && curr.dataType.scale > 0);
      prev[curr.name.value] = isFloat ? new FloatColumnOption(colOption) : new IntegerColumnOption(colOption);
      return prev;
    } else if (curr.dataType instanceof types.StringType) {
      if (!(colOption instanceof StringColumnOption)) throw new GenerateError;
      const isFloat = curr.dataType instanceof types.Float || (curr.dataType instanceof types.DecimalType && curr.dataType.scale && curr.dataType.scale > 0);
      prev[curr.name.value] = isFloat ? new FloatColumnOption(colOption) : new IntegerColumnOption(colOption);
      return prev;
    }
    return prev;
  }, {});
};

type Process = {
  columnName: string
  row: number,
  col: number,
  keysInUse: { [keyName: string]: string[] }
}
const generateInteger = (process: Process, option: IntegerColumnOption, constraint: TableConstraint) => {
}
