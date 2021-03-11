import { CreateTableStatement, TableConstraint, parser } from './parser';
import { dataTypes as types } from './data-types';
import { columnOptions as co } from './column-options';
import { logger,max,min,add,subtract } from './util';

type ColumnOptionUnion = NumericColumnOption|StringColumnOption|DatetimeColumnOption|BooleanColumnOption;
type GenColOptType = { [columnName: string]: ColumnOptionUnion|FixedValue|undefined };
type Prefix = string|((col: number, colName: string) => string);
// type Structual<T> = T extends Function | Array<any> ? T : T extends object ? { [K in keyof T]: Structual<T[K]> } : T;
type OmitTag<T> = Omit<T, '__tag'>;
type Param<T> = Partial<OmitTag<T>>;

/** Options for data to be generated. */
export interface GeneratorOption<T extends {} = {}> {
  readonly __tag: 'GeneratorOption'
  /** Output format. Either csv or insert statement. */
  outputFormat: CsvFormat|InsertStatementFormat
  /**
   *  Options for each column to determine how they are generated.
   *  The options need to be valid types for corresponding columns.
   *    - Column -             - Option -
   *    Numeric types          -> NumericColumnOption
   *    String types           -> StringColumnOption
   *    Datetime types         -> DatetimeColumnOption
   *    Boolean types          -> BooleanColumnOption
   *  If you want fixed values for some columns throughout entire rows,
   *  you can specify the value using FixedValue.
   */
  columnOptions: GenColOptType
  /**
   * Fall back column option for each column types.
   */
  columnOptionsDefault: ColumnOptionDefaultType
  /**
   * A function to manipulate row data after it's generated.
   * The column you can modify is only the column which is set FixValue option.
   */
  eachRow?: (columns: ColumnsType, process: RowProcess, prev: T) => [columns: ColumnsType, next: T]
  /** The number of rows to be generated. Default: 10 */
  size: number
}
export const newGeneratorOption = (obj?: Param<GeneratorOption>): GeneratorOption => ({
  outputFormat: newCsvFormat(),
  columnOptions: {},
  columnOptionsDefault: { num: newNumericColumnOption(), str: newStringColumnOption(), date: newDatetimeColumnOption(), bool: newBooleanColumnOption() },
  size: 10,
  ...obj,
  __tag: 'GeneratorOption',
});
type ColumnOptionDefaultType = { num: NumericColumnOption, str: StringColumnOption, date: DatetimeColumnOption, bool: BooleanColumnOption };
/** CsvFormat is used for GneratorOption.outputFormat */
export interface CsvFormat {
  readonly __tag: 'CsvFormat'
  /** Delimiter of each column. Default: ',' */
  delimiter: string
  /** Quote for each value. Default: '"' */
  quote: string
  /** Escape sequence. Default: '"' */
  escapeSequence: string
  /** Whether output header or not. Default: false */
  header: boolean
}
export const newCsvFormat = (obj?: Partial<CsvFormat>): CsvFormat => ({
  delimiter: `,`,
  quote: `"`,
  escapeSequence: `"`,
  header: false,
  ...obj,
  __tag: 'CsvFormat',
});
/** InsertStatementFormat is used for GneratorOption.outputFormat */
export interface InsertStatementFormat {
  readonly __tag: 'InsertStatementFormat'
}
export const newInsertStatementFormat = (obj?: Param<InsertStatementFormat>): InsertStatementFormat => ({
  ...obj,
  __tag: 'InsertStatementFormat',
});
export const NUM_LOOP_OPTS = ['loop','negate','keep'] as const;
/** NumericColumnOptions is used for GeneratorOption.columnOptions */
export interface NumericColumnOption {
  readonly __tag: 'NumericColumnOption'
  /** How much advance per each row. Default: 1 for integers, 0.1 for decimals */
  stepBy?: number
  /** Value of the first row. Default: 1 */
  initialValue: number|((col:number)=>number)
  /** Limit of incrementation. Default: depend on the corresponding table data type. */
  limit?: number
  /**
   * How to behave when incrementation hits the limit. Default: loop.
   *   loop: back to the initial value and continue to increment
   *   negate: negate the value and continue to increment
   *   keep: stop incrementation and keep the limit value
   */
  loop: typeof NUM_LOOP_OPTS[number]
}
export const newNumericColumnOption = (obj?: Omit<Param<NumericColumnOption>, '__initialValueNum'>): NumericColumnOption => ({
  initialValue: 1,
  loop: 'loop',
  ...obj,
  __tag: 'NumericColumnOption',
});
type IntegerColumnOption = Omit<OmitTag<NumericColumnOption>, 'stepBy'> & {
  readonly __tag: 'IntegerColumnOption'
  stepBy: number
  /** Internal property */
  __initialValueNum: number
}
export const newIntegerColumnOption = (obj?: Param<IntegerColumnOption>): IntegerColumnOption => ({
  ...newNumericColumnOption(obj),
  stepBy: obj?.stepBy || 1,
  __initialValueNum: obj?.__initialValueNum || 1,
  __tag: 'IntegerColumnOption',
});
type DecimalColumnOption = Omit<OmitTag<NumericColumnOption>, 'stepBy'> & {
  readonly __tag: 'DecimalColumnOption'
  stepBy: number
  precision:number,
  scale:number,
  /** Internal property */
  __maxScale: number,
  /** Internal property */
  readonly __initialValueNum: number
}
export const newDecimalColumnOption = (obj?: Param<DecimalColumnOption>): DecimalColumnOption => {
  const ret: DecimalColumnOption = {
    ...newNumericColumnOption(obj),
    stepBy: obj?.stepBy || 0.1,
    precision: Infinity,
    scale: 0,
    __initialValueNum: obj?.__initialValueNum || 1,
    __maxScale: 0,
    __tag: 'DecimalColumnOption',
  };
  const scale1 = (ret.__initialValueNum.toString().split('.')[1] || '').length;
  const scale2 = (ret.stepBy.toString().split('.')[1] || '').length;
  const __maxScale = max(scale1,scale2);
  return { ...ret, __maxScale };
};
export const STR_LOOP_OPTS = ['loop','keep'] as const;
export const LENGTH_IN_OPTS = ['char','byte'] as const;
/** StringColumnOption is used for GeneratorOption.columnOptions */
export interface StringColumnOption {
  readonly __tag: 'StringColumnOption',
  /** Limit of incrementation. Default: depend on the corresponding table data type. */
  maxLength: number,
  /** Which measurement unit to use, either char or byte. Default: char */
  lengthIn: typeof LENGTH_IN_OPTS[number],
  /** Prefix. Default: a character in A-Z, a-z, depending on the column position */
  prefix: Prefix,
  /**
   * How to behave when incrementation hits the limit. Default: loop.
   *   loop: back to the initial value and continue to increment
   *   keep: stop incrementation and keep the limit value
   */
  loop: typeof STR_LOOP_OPTS[number],
}
export const newStringColumnOption = (obj?: Omit<Param<StringColumnOption>, '__prefixStr'>): StringColumnOption => ({
  maxLength: 0,
  lengthIn: 'char',
  prefix: '',
  loop: 'loop',
  ...obj,
  __tag: 'StringColumnOption',
});
type CharColumnOption = OmitTag<StringColumnOption> & {
  readonly __tag: 'CharColumnOption'
  /** Internal property */
  __prefixStr: string,
}
export const newCharColumnOption = (obj?: Param<CharColumnOption>): CharColumnOption => ({
  ...newStringColumnOption(obj),
  lengthIn: obj?.lengthIn || 'char',
  __prefixStr: obj?.__prefixStr || '',
  __tag: 'CharColumnOption',
});
type BinaryColumnOption = OmitTag<StringColumnOption> & {
  readonly __tag: 'BinaryColumnOption'
  /** Internal property */
  __prefixStr: string,
}
export const newBinaryColumnOption = (obj?: Param<BinaryColumnOption>): BinaryColumnOption => ({
  ...newStringColumnOption(obj),
  lengthIn: obj?.lengthIn || 'byte',
  __prefixStr: obj?.__prefixStr || '',
  __tag: 'BinaryColumnOption',
});
/** DatetimeColumnOption is used for GeneratorOption.columnOptions */
export interface DatetimeColumnOption {
  readonly __tag: 'DatetimeColumnOption',
  /**
   * Value of the first row.
   * Default: 1970-01-01T00:00:00.000Z. Output depends on their column types, as shown below.
   *   Date      : '1970-01-01'
   *   Time      : '00:00:00'
   *   Timestamp : '1970-01-01 00:00:00'
   */
  initialValue: Date,
  /**
   *  DatetimeColumnOption is used for GeneratorOption.columnOptions
   *   Currently this option is not supported yet, so the increment step can't be changed.
   *   The value depends on the table column type.
   *     Date type      : 1 day.
   *     Time           : 1 second.
   *     Timestamp type : 1 day.
   */
  stepBy: number,
}
export const newDatetimeColumnOption = (obj?: Param<Omit<DatetimeColumnOption, 'stepBy'>>): DatetimeColumnOption => ({
  initialValue: new Date(0),
  stepBy: 1,
  ...obj,
  __tag: 'DatetimeColumnOption',
});
type DateColumnOption = OmitTag<DatetimeColumnOption> & {
  /** Internal property */
  readonly __tag: 'DateColumnOption'
}
export const newDateColumnOption = (obj?: Param<Omit<DateColumnOption, 'stepBy'>>): DateColumnOption => ({
  ...newDatetimeColumnOption(obj),
  __tag: 'DateColumnOption',
});
type TimeColumnOption = OmitTag<DatetimeColumnOption> & {
  /** Internal property */
  readonly __tag: 'TimeColumnOption'
}
export const newTimeColumnOption = (obj?: Param<Omit<TimeColumnOption, 'stepBy'>>): TimeColumnOption => ({
  ...newDatetimeColumnOption(obj),
  __tag: 'TimeColumnOption',
});
type TimestampColumnOption = OmitTag<DatetimeColumnOption> & {
  /** Internal property */
  readonly __tag: 'TimestampColumnOption'
}
export const newTimestampColumnOption = (obj?: Param<Omit<TimestampColumnOption, 'stepBy'>>): TimestampColumnOption => ({
  ...newDatetimeColumnOption(obj),
  __tag: 'TimestampColumnOption',
});
/** BooleanColumnOption is used for GeneratorOption.columnOptions */
export interface BooleanColumnOption {
  readonly __tag: 'BooleanColumnOption',
  /** Value of the first row. Default: false */
  initialValue: boolean,
  /** Whether randomly generate data or not. Default: false */
  random: boolean,
  /** Whether use null value or not. If table column has not-null constraint, this option is ignored. Default: false */
  useNull: boolean,
}
export const newBooleanColumnOption = (obj?: Param<BooleanColumnOption>): BooleanColumnOption => ({
  initialValue: false,
  random: false,
  useNull: false,
  ...obj,
  __tag: 'BooleanColumnOption',
});
/** FixedValue is use for GneratorOption.columnOptions */
export interface FixedValue {
  /** Internal property */
  readonly __tag: 'FixedValue'
  /** The value to be set to a column */
  value: string;
}
export const newFixedValue = (obj: Omit<FixedValue, '__tag'>): FixedValue => ({
  ...obj,
  __tag: 'FixedValue',
});
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
export async function* generate(statement: CreateTableStatement, option: GeneratorOption = newGeneratorOption())
  : AsyncGenerator<[GeneratorResult, GeneratorValidationError[]], void, undefined> {
  logger.log(statement, option);
  const columnDefs = statement.columns;
  let prevColumns: ColumnsType = {num:[],str:[],date:[],bool:[],fixed:[]};
  let strPrefixCodePoint:number=65;
  type AvailableOptionUnion = IntegerColumnOption|DecimalColumnOption|CharColumnOption|BinaryColumnOption|DateColumnOption|TimeColumnOption|TimestampColumnOption|BooleanColumnOption|FixedValue;
  const opts: AvailableOptionUnion[] = columnDefs.map((def,i) => {
    const colOption = option.columnOptions[def.name.value];
    if (colOption && colOption.__tag === 'FixedValue') return colOption;
    const dataType = def.dataType;
    let opt;
    if (dataType instanceof types.NumericType) {
      if (colOption && !(colOption.__tag === 'NumericColumnOption')) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: NumericColumnOption = colOption || option.columnOptionsDefault.num;
      const __initialValueNum = typeof nonNullOpt.initialValue === 'function' ? nonNullOpt.initialValue(i) : nonNullOpt.initialValue;
      if (dataType instanceof types.Float) {
        let limit = dataType.precision ? +('9'.repeat(dataType.precision)) : Infinity;
        limit = min(limit, nonNullOpt?.limit||Infinity);
        opt = newDecimalColumnOption({ precision: dataType.precision, ...nonNullOpt, __initialValueNum, limit });
        prevColumns.num[i]=subtract(opt.__initialValueNum, opt.stepBy, opt.__maxScale);
      } else if (dataType instanceof types.DecimalType) {
        let limit = dataType.precision ? +('9'.repeat(dataType.precision - (dataType.scale || 0))) : Infinity;
        limit = min(limit, nonNullOpt?.limit||Infinity);
        logger.log('stepBy', nonNullOpt.stepBy);
        opt = newDecimalColumnOption({ precision: dataType.precision, scale: dataType.scale, ...nonNullOpt, __initialValueNum, limit });
        logger.log('stepBy', opt.stepBy);
      } else {
        opt = newIntegerColumnOption({...nonNullOpt, __initialValueNum});
        prevColumns.num[i]=opt.__initialValueNum - opt.stepBy;
      }
      prevColumns.num[i]=subtract(opt.__initialValueNum, opt.stepBy, opt.__tag === 'DecimalColumnOption' ? opt.__maxScale : 0);
    } else if (dataType instanceof types.StringType) {
      if (colOption && !(colOption.__tag === 'StringColumnOption')) throw new GeneratorFatalError('invalid column option');
      const isChar = dataType instanceof types.CharacterStringType;
      const nonNullOpt: StringColumnOption = colOption || option.columnOptionsDefault.str;
      const maxLength = nonNullOpt.maxLength || dataType.length || 10; // There is no basis for 10 but we need a concrete integer value.
      logger.log('maxLength', dataType.length);
      let __prefixStr = typeof nonNullOpt.prefix === 'function' ? nonNullOpt.prefix(i,def.name.value) : nonNullOpt.prefix;
      if (!__prefixStr) __prefixStr=String.fromCodePoint(strPrefixCodePoint++);
      opt = isChar ? newCharColumnOption({...nonNullOpt, maxLength, __prefixStr}) :
                     newBinaryColumnOption({...nonNullOpt, maxLength, __prefixStr});
      const initialValue:string = __prefixStr.slice(0,maxLength-1) + '0'.repeat(max(maxLength - __prefixStr.length, 1));
      logger.log('initialValue', initialValue);
      logger.log('maxLength', maxLength);
      prevColumns.str[i]=initialValue;
    } else if (dataType instanceof types.DatetimeType) {
      if (colOption && !(colOption.__tag === 'DatetimeColumnOption')) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: DatetimeColumnOption = colOption || option.columnOptionsDefault.date;
      opt = dataType instanceof types.Date ? newDateColumnOption({ ...nonNullOpt }) :
            dataType instanceof types.Time ? newTimeColumnOption({ ...nonNullOpt }) :
                                             newTimestampColumnOption({ ...nonNullOpt });
      prevColumns.date[i]= dataType instanceof types.Time ? opt.initialValue : new Date(nonNullOpt.initialValue.getTime() - 24*60*60*1000);
    } else if (dataType instanceof types.BooleanType) {
      if (colOption && !(colOption.__tag === 'BooleanColumnOption')) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: BooleanColumnOption = colOption || option.columnOptionsDefault.bool;
      opt = newBooleanColumnOption({ ...nonNullOpt });
      prevColumns.bool[i]=!opt.initialValue;
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
  if (option.outputFormat.__tag === 'CsvFormat' && option.outputFormat.header) {
    const colNames = columnDefs.map(col => col.name.value);
    const header = colNames.join(',');
    yield [{ columns: colNames, row: header }, [] ];
  }
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
      if (opt.__tag === 'IntegerColumnOption' || opt.__tag === 'DecimalColumnOption') columns.num[j]=(generateNumeric(colProcess, opt));
      if (opt.__tag === 'CharColumnOption' || opt.__tag === 'BinaryColumnOption') columns.str[j]=(generateString(colProcess, opt));
      if (opt.__tag === 'DateColumnOption' || opt.__tag === 'TimeColumnOption' || opt.__tag === 'TimestampColumnOption') columns.date[j]=(generateDatetime(colProcess, opt));
      if (opt.__tag === 'BooleanColumnOption') columns.bool[j]=(generateBoolean(colProcess, opt));
      if (opt.__tag === 'FixedValue') columns.fixed[j]=opt.value;
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
        if (opt.__tag === 'IntegerColumnOption' || opt.__tag === 'DecimalColumnOption') return columns.num[j].toString();
        if (opt.__tag === 'CharColumnOption' || opt.__tag === 'BinaryColumnOption') return columns.str[j];
        const date=columns.date[j];
        if (opt.__tag === 'DateColumnOption') return date.toISOString().slice(0,10);
        if (opt.__tag === 'TimeColumnOption') return `${Math.floor(date.getTime() / (60*60*1000)) + date.getUTCHours()}:${date.getUTCMinutes()}:${date.getUTCSeconds()}`;
        if (opt.__tag === 'TimestampColumnOption') return date.toISOString().slice(0,19).replace('T',' ');
        const bool=columns.bool[j];
        if (opt.__tag === 'BooleanColumnOption') return bool === undefined ? bool : bool.toString();
        const fixed=columns.fixed[j];
        return fixed ? fixed : 'null';
      })();
    }
    const [,errs] = [keysInUse] = validateRow(rowProcess, columnsStr);
    if (errs) errors.push(...errs);
    const row = option.outputFormat.__tag === 'CsvFormat' ? toCsv(columnsStr, option.outputFormat) : toInsert(columnsStr, tableName);
    prevColumns=columns;
    yield [{ columns: columnsStr, row }, errors];
  }
}

// TODO negative step
const generateNumeric = (process: ColumnProcess, option: IntegerColumnOption|DecimalColumnOption): number => {
  const prev=process.prevColumns.num[process.col];
  // logger.log('add', prev, option.stepBy, maxScale);
  const next = add(prev, option.stepBy, option.__tag === 'DecimalColumnOption' ? option.__maxScale : 0);
  if (next > (option?.limit||Infinity)) { // overflow
    return option.loop === 'loop'   ? option.__initialValueNum :
           option.loop === 'negate' ? -prev:
                                      prev;
  }
  return next;
};
const generateString = (process: ColumnProcess, option: CharColumnOption|BinaryColumnOption): string => {
  const prev=process.prevColumns.str[process.col];
  let seq=parseInt(prev.slice(option.__prefixStr.length))+1;
  if (option.maxLength !== Infinity && seq > +('9'.repeat(option.maxLength - option.__prefixStr.length))) { // overflow
    seq = option.loop === 'loop' ? 1 : seq-1;
  }
  return option.__prefixStr + '0'.repeat(option.maxLength - option.__prefixStr.length - seq.toString().length) + seq;
};
const generateDatetime = (process: ColumnProcess, option: DateColumnOption|TimeColumnOption|TimestampColumnOption): Date => {
  const prev=process.prevColumns.date[process.col];
  if (option.__tag === 'TimeColumnOption') {
    return new Date(prev.getTime()+(option.stepBy*1000)); // advance by seconds
  } else {
    return new Date(prev.getTime()+(option.stepBy*24*60*60*1000)); // advance by days
  }
};
const generateBoolean = (process: ColumnProcess, option: BooleanColumnOption): boolean|undefined => {
  const prev=process.prevColumns.bool[process.col];
  if (option.random) {
    if (!option.useNull) return !!Math.floor(Math.random()*2);
    const random = Math.floor(Math.random()*3);
    return random === 0 ? false :
           random === 1 ? true :
                          undefined;
  }
  if (option.useNull) {
    return prev === undefined  ? false :
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

