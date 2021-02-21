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
   *    Boolean types          -> BooleanColumnOption
   *  If you want a fixed value for some columns throughout entire data,
   *  you can specify the value using FixedValue.
   */
  columnOptions: { [columnName: string]: ColumnOptionUnion|FixedValue } = {}
  /**
   *  
   */
  columnOptionsDefault: ColumnOptionUnion[] = []
  /** A function to manipulate row data after it's generated. */
  eachRow?<T extends {} = {}>(rowCount: number, columns: string[], prev: T): [columns: string[], next: T]
  /** The number of rows to be generated. */
  size: number = 0
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
type ColumnOptionUnion = NumericColumnOption|StringColumnOption|DatetimeColumnOption|BooleanColumnOption;
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
  public readonly initialValue: number|((col:number)=>number) = 1
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
  constructor(option?: Partial<NumericColumnOption>) {
    super({ stepBy: 1, ...option });
  }
}
class FloatColumnOption extends NumericColumnOption {
  private _floatColumnOption='nominal'
  constructor(option?: Partial<NumericColumnOption>) {
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
  constructor(option?: Partial<StringColumnOption>) {
    super({ lengthIn: 'char', ...option });
  }
}
class BinaryColumnOption extends StringColumnOption {
  private _binaryColumnOption='nominal'
  constructor(option?: Partial<StringColumnOption>) {
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
  public readonly stepBy: number = 1
  public constructor(obj?: {initialValue?: string}) {
    if (!obj) return;
    Object.assign(this,obj);
  }
}
/** BooleanColumnOption is used for GeneratorOption.columnOptions */
export class BooleanColumnOption {
  private _booleanColumnOption='nominal'
  /** Value of the first row. Default: false */
  public readonly initialValue: boolean = false
  /** Whether randomly generate data or not. Default: false */
  public readonly random: boolean = false
  /** Whether use null value or not. If table column has not-null constraint, this option is ignored. Default: false */
  public readonly useNull: boolean = false
}
/** FixedValue is use for GneratorOption.columnOptions */
export class FixedValue {
  private _fixedValue='nominal'
  constructor(
    /** The value to be set to a column */
    public readonly value: string = '',
  ) {}
}
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
export async function* generate(statement: CreateTableStatement, options: GeneratorOption)
  : AsyncGenerator<[columns: string[], error?: GenerateError], void, undefined> {
  const columnDefs = statement.columns;
  const constraints = statement.constraints;
  const nameToColIdx = {};
  let prevColumns: string[] = [];
  type AvailableOptionUnion = IntegerColumnOption|FloatColumnOption|CharColumnOption|BinaryColumnOption|DatetimeColumnOption|BooleanColumnOption|FixedValue;
  const opts: AvailableOptionUnion[] = columnDefs.map(def => {
    const colOption = options.columnOptions[def.name.value];
    let opt;
    if (def.dataType instanceof types.NumericType) {
      if (colOption && !(colOption instanceof NumericColumnOption)) throw new GenerateError('invalid column option');
      const isFloat = def.dataType instanceof types.Float || (def.dataType instanceof types.DecimalType && def.dataType.scale && def.dataType.scale > 0);
      opt = isFloat ? new FloatColumnOption(colOption) : new IntegerColumnOption(colOption);
      prevColumns.push(''+opt.initialValue);
    } else if (def.dataType instanceof types.StringType) {
      if (colOption && !(colOption instanceof StringColumnOption)) throw new GenerateError('invalid column option');
      const isChar = def.dataType instanceof types.CharacterStringType;
      opt = isChar ? new CharColumnOption(colOption) : new BinaryColumnOption(colOption);
      prevColumns.push(''+opt.initialValue);
    } else if (def.dataType instanceof types.DatetimeType) {
      if (colOption && !(colOption instanceof DatetimeColumnOption)) throw new GenerateError('invalid column option');
      opt = colOption || new DatetimeColumnOption;
      prevColumns.push(opt.initialValue);
    } else if (def.dataType instanceof types.BooleanType) {
      if (colOption && !(colOption instanceof BooleanColumnOption)) throw new GenerateError('invalid column option');
      opt = colOption || new BooleanColumnOption;
      prevColumns.push(''+opt.initialValue);
    } else {
      throw new GenerateError('invalid data type');
    }
    return opt;
  }, {});
  for(let i=0; i<options.size; i++) {
    let work={};
    const columns: string[] = [];
    for(let j=0; j<opts.length; j++) {
      const process: Process = {
        columnName: columnDefs[j].name.value,
        row: i,
        col: j,
        keysInUse: {},
        prevColumns,
      };
      const opt = opts[j];
      if (opt instanceof IntegerColumnOption) columns.push(generateInteger(process, opt));
    }
    prevColumns=columns;
  }
  yield [['1','2'],];
}

type Process = {
  row: number
  col: number
  columnName: string
  prevColumns: string[]
  keysInUse: { [keyName: string]: string[] }
}
const generateInteger = (process: Process, option: IntegerColumnOption): string => {
  const prev=process.prevColumns[process.col];
  
  return '';
};
