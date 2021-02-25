import { CreateTableStatement, TableConstraint, parser } from './parser';
import { types } from './data-types';
import { columnOptions as co } from './column-options';
import { logger,max,min,add,subtract } from './util';

type ColumnOptionUnion = NumericColumnOption|StringColumnOption|DatetimeColumnOption|BooleanColumnOption;
type GenColOptType = { [columnName: string]: ColumnOptionUnion|FixedValue|undefined };
type Prefix = string|((col: number, colName: string) => string);

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
  columnOptions: GenColOptType = {}
  /**
   * Fall back column option for each column types.
   */
  columnOptionsDefault: ColumnOptionDefaultType = { num: new NumericColumnOption, str: new StringColumnOption, date: new DatetimeColumnOption, bool: new BooleanColumnOption }
  /**
   * A function to manipulate row data after it's generated.
   * The column you can modify is only the column which is set FixValue option.
   */
  eachRow?<T extends {} = {}>(columns: ColumnsType, process: RowProcess, prev: T): [columns: ColumnsType, next: T]
  /** The number of rows to be generated. Default: 10 */
  size: number = 10
  public constructor(obj?: GeneratorOption) { // TODO Partial
    if (!obj) return;
    Object.assign(this,obj);
  }
}
type ColumnOptionDefaultType = { num: NumericColumnOption, str: StringColumnOption, date: DatetimeColumnOption, bool: BooleanColumnOption };
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
  public readonly stepBy: number = 0
  /** Value of the first row. Default: 1 */
  public readonly initialValue: number|((col:number)=>number) = 1
  /** An inner property */
  public readonly __initialValueNum: number = 1
  /** Limit of incrementation. Default: depend on the corresponding table data type. */
  public readonly limit: number = Infinity
  /**
   * How to behave when incrementation hits the limit. Default: loop.
   *   loop: back to the initial value and continue to increment
   *   negate: negate the value and continue to increment
   *   keep: stop incrementation and keep the limit value
   */
  public readonly loop: 'loop'|'negate'|'keep' = 'loop'
  public constructor(obj?: Omit<Partial<NumericColumnOption>, '__initialValueNum'>) {
    if (!obj) return;
    Object.assign(this,obj);
  }
}
class IntegerColumnOption extends NumericColumnOption {
  private _integerColumnOption='nominal'
  constructor(obj?: Partial<IntegerColumnOption>) {
    super({ ...obj, stepBy: (obj && obj.stepBy) || 1});
  }
}
class DecimalColumnOption extends NumericColumnOption {
  private _decimalColumnOption='nominal'
  public readonly precision:number=Infinity
  public readonly scale:number=0
  /** An inner property */
  public readonly __maxScale: number = 0
  constructor(obj?: Partial<DecimalColumnOption>) {
    super({ ...obj, stepBy: (obj && obj.stepBy) || 0.1 });
    const scale1 = (this.__initialValueNum.toString().split('.')[1] || '').length;
    const scale2 = (this.stepBy.toString().split('.')[1] || '').length;
    this.__maxScale = max(scale1,scale2);
  }
}
/** StringColumnOption is used for GeneratorOption.columnOptions */
export class StringColumnOption {
  /** Limit of incrementation. Default: depend on the corresponding table data type. */
  public readonly maxLength: number = 0
  /** Which measurement unit to use, either char or byte. Default: char */
  public readonly lengthIn: 'char'|'byte' = 'char'
  /** Prefix. Default: a character in A-Z, a-z, depending on the column position */
  public readonly prefix: Prefix = ''
  /** An inner property */
  public readonly __prefixStr: string = ''
  /**
   * How to behave when incrementation hits the limit. Default: loop.
   *   loop: back to the initial value and continue to increment
   *   keep: stop incrementation and keep the limit value
   */
  public readonly loop?: 'loop'|'keep' = 'loop'
  public constructor(obj?: Omit<Partial<StringColumnOption>, '__prefixStr'>) {
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
  /**
   * Value of the first row.
   * Default: 1970-01-01T00:00:00.000Z. Output depends on their column types, as shown below.
   *   Date      : '1970-01-01'
   *   Time      : '00:00:00'
   *   Timestamp : '1970-01-01 00:00:00'
   */
  public readonly initialValue: Date = new Date(0)
  /**
   *  DatetimeColumnOption is used for GeneratorOption.columnOptions
   *   Currently this option is not supported yet, so the increment step can't be changed.
   *   The value depends on the table column type.
   *     Date type      : 1 day.
   *     Time           : 1 second.
   *     Timestamp type : 1 day.
   */
  public readonly stepBy: number = 1
  public constructor(obj?: Omit<Partial<DatetimeColumnOption>, 'stepBy'>) {
    if (!obj) return;
    Object.assign(this,obj);
  }
}
class DateColumnOption extends DatetimeColumnOption {
  private _dateColumnOption='nominal'
  constructor(obj: Omit<Partial<DatetimeColumnOption>, 'stepBy'>) {
    super(obj);
  }
}
class TimeColumnOption extends DatetimeColumnOption {
  private _timeColumnOption='nominal'
  constructor(obj: Omit<Partial<DatetimeColumnOption>, 'stepBy'>) {
    super(obj);
  }
}
class TimestampColumnOption extends DatetimeColumnOption {
  private _timestampColumnOption='nominal'
  constructor(obj: Omit<Partial<DatetimeColumnOption>, 'stepBy'>) {
    super(obj);
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
  public constructor(obj?: Partial<BooleanColumnOption>) {
    if (!obj) return;
    Object.assign(this,obj);
  }
}
/** FixedValue is use for GneratorOption.columnOptions */
export class FixedValue {
  private _fixedValue='nominal'
  constructor(
    /** The value to be set to a column */
    public readonly value: string = '',
  ) {}
}

type ColumnsType = {num:number[], str:string[], date:Date[], bool:(boolean|undefined)[], fixed: (string|undefined)[]};
type KeysInUseType = { [keyName: string]: string[] };
type NameIdx={[key:string]:number};
type Keys={keyName: string, keys: string[]};
type RowProcess = {
  row: number
  prevColumns: ColumnsType
  keysInUse: KeysInUseType
  constraints: TableConstraint[]
  primaryKeys?: Keys
  uniqueKeysSet: Keys[]
  nameIdx: NameIdx
}
type ColumnProcess = RowProcess & {
  col: number
  columnName: string
}

/** GeneratorFatalError is thrown when data generation stops due to something unexpected, such as invalid options. */
export class GeneratorFatalError extends Error {
  private _generatorFatalError='nominal'
  public errorCode: string = ''
}
/** GeneratorValidationError is returned when a generated row has invalid data. */
export class GeneratorValidationError extends Error {
  private _generatorValidationError='nominal'
  public errorCode: string = ''
}
/** GeneratorResult has result of data generation */
export type GeneratorResult = { columns: (string|undefined)[], row: string };

export async function tryParseAndGenerate(src: string, option?: GeneratorOption): Promise<string[]> {
  const [stmts,error] = parser.parse(src);
  if (!stmts) throw error;
  const rows:string[]=[];
  for (const stmt of stmts) {
    for await (const [result, errors] of generate(stmt, option)) {
      if (errors.length > 0) throw errors;
      rows.push(result.row);
    }
  }
  return rows;
}
export async function* parseAndGenerate(src: string, option?: GeneratorOption): AsyncGenerator<[GeneratorResult, GeneratorValidationError[]], void, undefined> {
  const [stmts,error] = parser.parse(src);
  if (!stmts) throw error;
  for (const stmt of stmts) {
    for await (const result of generate(stmt, option)) {
      yield result;
    }
  }
}

/**
 * Generates data from a create table statement with options.
 * How data is generated depends on types and options but the general idea is simple.
 * Generator adds 1 to previous data row by row so each column would have sequentially incremented number.
 * Given that we have the following statement,
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
export async function* generate(statement: CreateTableStatement, option: GeneratorOption = new GeneratorOption):
  AsyncGenerator<[GeneratorResult, GeneratorValidationError[]], void, undefined> {
  const columnDefs = statement.columns;
  let prevColumns: ColumnsType = {num:[],str:[],date:[],bool:[],fixed:[]};
  let strPrefixCodePoint:number=65;
  type AvailableOptionUnion = IntegerColumnOption|DecimalColumnOption|CharColumnOption|BinaryColumnOption|DatetimeColumnOption|BooleanColumnOption|FixedValue;
  const opts: AvailableOptionUnion[] = columnDefs.map((def,i) => {
    const colOption = option.columnOptions[def.name.value];
    if (colOption instanceof FixedValue) return colOption;
    const dataType = def.dataType;
    let opt;
    if (dataType instanceof types.NumericType) {
      if (colOption && !(colOption instanceof NumericColumnOption)) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: NumericColumnOption = colOption || option.columnOptionsDefault.num;
      const __initialValueNum = typeof nonNullOpt.initialValue === 'function' ? nonNullOpt.initialValue(i) : nonNullOpt.initialValue;
      if (dataType instanceof types.Float) {
        let limit = dataType.precision ? +('9'.repeat(dataType.precision)) : Infinity;
        limit = min(limit, nonNullOpt.limit);
        opt = new DecimalColumnOption({ precision: dataType.precision, ...nonNullOpt, __initialValueNum, limit });
        prevColumns.num[i]=subtract(opt.__initialValueNum, opt.stepBy, opt.__maxScale);
      } else if (dataType instanceof types.DecimalType) {
        let limit = dataType.precision ? +('9'.repeat(dataType.precision - (dataType.scale || 0))) : Infinity;
        limit = min(limit, nonNullOpt.limit);
        logger.log('stepBy', nonNullOpt.stepBy);
        opt = new DecimalColumnOption({ precision: dataType.precision, scale: dataType.scale, ...nonNullOpt, __initialValueNum, limit });
        logger.log('stepBy', opt.stepBy);
      } else {
        opt = new IntegerColumnOption(colOption);
        prevColumns.num[i]=opt.__initialValueNum - opt.stepBy;
      }
      prevColumns.num[i]=subtract(opt.__initialValueNum, opt.stepBy, opt.__maxScale! );
    } else if (dataType instanceof types.StringType) {
      if (colOption && !(colOption instanceof StringColumnOption)) throw new GeneratorFatalError('invalid column option');
      const isChar = dataType instanceof types.CharacterStringType;
      const nonNullOpt: StringColumnOption = colOption || option.columnOptionsDefault.str;
      const maxLength = nonNullOpt.maxLength || dataType.length || 10; // There is no basis for 10 but we need a concrete integer value.
      logger.log('maxLength', dataType.length);
      let __prefixStr = typeof nonNullOpt.prefix === 'function' ? nonNullOpt.prefix(i,def.name.value) : nonNullOpt.prefix;
      if (!__prefixStr) __prefixStr=String.fromCodePoint(strPrefixCodePoint++);
      opt = isChar ? new CharColumnOption({...colOption, maxLength, __prefixStr}) :
                     new BinaryColumnOption({...colOption, maxLength, __prefixStr});
      const initialValue:string = __prefixStr.slice(0,maxLength-1) + '0'.repeat(max(maxLength - __prefixStr.length, 1));
      logger.log('initialValue', initialValue);
      logger.log('maxLength', maxLength);
      prevColumns.str[i]=initialValue;
    } else if (dataType instanceof types.DatetimeType) {
      if (colOption && !(colOption instanceof DatetimeColumnOption)) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: DatetimeColumnOption = colOption || option.columnOptionsDefault.date;
      opt = dataType instanceof types.Date ? new DateColumnOption(nonNullOpt) :
            dataType instanceof types.Time ? new TimeColumnOption(nonNullOpt) :
                                             new TimestampColumnOption(nonNullOpt);
      prevColumns.date[i]=opt.initialValue;
    } else if (dataType instanceof types.BooleanType) {
      if (colOption && !(colOption instanceof BooleanColumnOption)) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: BooleanColumnOption = colOption || option.columnOptionsDefault.bool;
      const useNull = def.options.some(o => o instanceof co.NotNull);
      opt = new BooleanColumnOption({ ...nonNullOpt, useNull });
      prevColumns.bool[i]=opt.initialValue;
    } else {
      throw new GeneratorFatalError('invalid data type');
    }
    return opt;
  });
  logger.log(opts);
  let keysInUse: KeysInUseType = {};
  const tableColOpts: co.ColumnOption[][] = columnDefs.map(def => def.options.map(opt => opt.option));
  const constraints= statement.constraints;
  const nameIdx = columnDefs.reduce<NameIdx>((prev,crr,i) => {
    const next = prev;
    next[crr.name.value]=i;
    return prev;
  }, {});
  const primaryKeys: Keys|undefined = (() => {
    for(const constraint of constraints) {
      if (constraint instanceof parser.Unique && constraint.isPrimary) {
        const keys = constraint.columns.map(col => col.value);
        return { keyName: keys.join(), keys };
      }
    }
    return undefined;
  })();
  const uniqueKeysSet: Keys[] = (() => {
    const keysSet: Keys[]=[];
    for(const constraint of constraints) {
      if (constraint instanceof parser.Unique && !constraint.isPrimary) {
        const keys = constraint.columns.map(col => col.value);
        keysSet.push({ keyName: keys.join(), keys });
      }
    }
    return keysSet;
  })();
  const tableName = statement.name.value.map(ident=>ident.value).join('.');
  for(let i=0; i<option.size; i++) {
    const errors: GeneratorValidationError[] = [];
    let work={};
    const columns: ColumnsType = {num:[],str:[],date:[],bool:[],fixed:[]};
    const rowProcess: RowProcess = {
      row: i,
      keysInUse,
      prevColumns,
      constraints,
      primaryKeys,
      uniqueKeysSet,
      nameIdx
    };
    for(let j=0; j<opts.length; j++) {
      const colProcess: ColumnProcess = {
        ...rowProcess,
        columnName: columnDefs[j].name.value,
        col: j,
      };
      const opt = opts[j];
      if (opt instanceof NumericColumnOption) columns.num[j]=(generateNumeric(colProcess, opt));
      if (opt instanceof StringColumnOption) columns.str[j]=(generateString(colProcess, opt));
      if (opt instanceof DatetimeColumnOption) columns.date[j]=(generateDatetime(colProcess, opt));
      if (opt instanceof BooleanColumnOption) columns.bool[j]=(generateBoolean(colProcess, opt));
      if (opt instanceof FixedValue) columns.fixed[j]=opt.value;
    }
    if (rowProcess && option.eachRow) {
      const [editedColumns] = [,work] = option.eachRow(columns, rowProcess, work);
      columns.fixed = editedColumns.fixed;
    }
    for(let j=0; j<opts.length; j++) {
      const colProcess: ColumnProcess = {
        ...rowProcess,
        columnName: columnDefs[j].name.value,
        col: j,
      };
      const error = validateFixedValue(colProcess, columns.fixed[j], tableColOpts[j]);
      if (error) errors.push(error);
    }
    const columnsStr: (string|undefined)[] = [];
    for(let j=0; j<opts.length; j++) {
      const opt = opts[j];
      columnsStr[j] = (() => {
        if (opt instanceof NumericColumnOption) return columns.num[j].toString();
        if (opt instanceof StringColumnOption) return columns.str[j];
        const date=columns.date[j];
        if (opt instanceof DateColumnOption) return date.toISOString().slice(0,10);
        if (opt instanceof TimeColumnOption) return `${date.getTime() / (60*60*1000) + date.getHours()}:${date.getMinutes}:${date.getSeconds}`;
        if (opt instanceof TimestampColumnOption) return date.toISOString().slice(0,19).replace('T',' ');
        const bool=columns.bool[j];
        if (opt instanceof BooleanColumnOption) return bool === undefined ? bool : bool.toString();
        const fixed=columns.fixed[j];
        return fixed ? fixed : 'null';
      })();
    }
    const [,errs] = [keysInUse] = validateRow(rowProcess, columnsStr);
    if (errs) errors.push(...errs);
    const row = option.outputFormat instanceof CsvFormat ? toCsv(columnsStr, option.outputFormat) : toInsert(columnsStr, tableName);
    prevColumns=columns;
    yield [{ columns: columnsStr, row }, errors];
  }
}

// TODO negative step
const generateNumeric = (process: ColumnProcess, option: NumericColumnOption): number => {
  const prev=process.prevColumns.num[process.col];
  logger.log('add', prev, option.stepBy, option.__maxScale);
  const next = add(prev, option.stepBy, option.__maxScale);
  if (next > option.limit) { // overflow
    return option.loop === 'loop'   ? option.__initialValueNum :
           option.loop === 'negate' ? -prev:
                                      prev;
  }
  return next;
};
const generateString = (process: ColumnProcess, option: StringColumnOption): string => {
  const prev=process.prevColumns.str[process.col];
  let seq=parseInt(prev.slice(option.__prefixStr.length))+1;
  if (option.maxLength !== Infinity && seq > +('9'.repeat(option.maxLength))) { // overflow
    seq = option.loop === 'loop' ? 1 : seq-1;
  }
  return option.__prefixStr + '0'.repeat(option.maxLength - option.__prefixStr.length - seq.toString().length) + seq;
};
const generateDatetime = (process: ColumnProcess, option: DatetimeColumnOption): Date => {
  const prev=process.prevColumns.date[process.col];
  if (option instanceof TimeColumnOption) {
    return new Date(prev.getTime()+(option.stepBy*1000)); // advance by seconds
  } else {
    return new Date(prev.getTime()+(option.stepBy*24*60*60*1000)); // advance by days
  }
};
const generateBoolean = (process: ColumnProcess, option: BooleanColumnOption): boolean|undefined => {
  const prev=process.prevColumns.bool[process.col];
  if (option.random) {
    if (option.useNull) return !!Math.floor(Math.random()*2);
    const random = Math.floor(Math.random()*3);
    return random === 0 ? false :
           random === 1 ? true :
                          undefined;
  }
  if (option.useNull) {
    return prev === null  ? false :
           prev === false ? true :
                            undefined;
  }
  return !prev;
};
const validateFixedValue = (process: ColumnProcess, column: string|undefined, colOpts: co.ColumnOption[]): GeneratorValidationError|undefined => {
  const notNull = colOpts.some(o => o instanceof co.NotNull);
  const isPK = process.primaryKeys && process.primaryKeys.keys.includes(process.columnName);
  if (!column && (notNull || isPK)) return new GeneratorValidationError('Violated not-null constraint');
};
const validateRow = (process: RowProcess, columns: (string|undefined)[]): [keysInuse: KeysInUseType, errors?: GeneratorValidationError[]] => {
  const errors:GeneratorValidationError[]=[];
  if (process.primaryKeys) {
    const value = process.primaryKeys.keys.reduce((prv, crr) => prv+(columns[process.nameIdx[crr]]||''), '');
    const keyInUse = process.keysInUse[process.primaryKeys.keyName];
    if (keyInUse.includes(value)) errors.push(new GeneratorValidationError('Violated unique-key constraint'));
    keyInUse.push(value);
  }
  process.uniqueKeysSet.forEach(uKeys => {
    const value = uKeys.keys.reduce((prv, crr) => prv+(columns[process.nameIdx[crr]]||''), '');
    const keyInUse = process.keysInUse[uKeys.keyName];
    if (keyInUse.includes(value)) errors.push(new GeneratorValidationError('Violated unique-key constraint'));
    keyInUse.push(value);
  });
  return [process.keysInUse, errors];
};
const toCsv = (columns: (string|undefined)[], option: CsvFormat): string => {
  const replaced = new RegExp(option.quote, 'g');
  return columns.map((col) => {
    if (!col) return 'null';
    const escaped = col.replace(replaced, option.escapeSequence+option.quote);
    return option.quote+escaped+option.quote;
  }).join(option.delimiter);
};
const toInsert = (columns: (string|undefined)[], table: string): string => {
  const cols = columns.map((col) => {
    if (!col) return 'null';
    const escaped = col.replace(/'/g, `'`);
    return `'`+escaped+`'`;
  }).join(`,`);
  return `insert into ${table} values (${cols});`;
};

