import { CreateTableStatement, TableConstraint, parser, ParseError } from './parser';
import { dataTypes as types } from './data-types';
import { columnOptions as co } from './column-options';
import { logger,add,subtract } from './util';

type ColumnOptionUnion = NumericColumnOption|StringColumnOption|DatetimeColumnOption|BooleanColumnOption;
type GenColOptType = { [columnName: string]: ColumnOptionUnion|FixedValue|undefined };
type Prefix = string|((col: number, colName: string) => string);
// type Structual<T> = T extends Function | Array<any> ? T : T extends object ? { [K in keyof T]: Structual<T[K]> } : T;
type OmitTag<T> = Omit<T, '_tag'>;
type Param<T> = Partial<OmitTag<T>>;

/** Options for data to be generated. */
export interface GeneratorOption {
  readonly _tag: 'GeneratorOption'
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
   * This callback function is called each time when rows are generated to modify generated rows.
   * @example
   * // Adds line number to the second column
   * columns.num[1] = columns.num[1] + (process.row + 1);
   * // Adds column name to begining of the fourth column
   * columns.str[3] = process.names[3] + columns.str[3];
   * // Returns modified columns
   * return [columns,tempPrev];
   * @param {ColumnsType} columns Columns to be modified.
   * @param {Readonly<RowProcess>} process Information used for data generation process.
   * @param {object} tempPrev Temporary information taken over from previous time function call.
   * @returns {[columns: ColumnsType, tempNext: typeof tempPrev]} Modified columns and the temporary information taken over to next time function call.
   * @default undefined
   */
  eachRow?: (columns: ColumnsType, process: Readonly<RowProcess>, tempPrev: object) => [columns: ColumnsType, tempNext: typeof tempPrev]
  /**
   * The number of rows to be generated.
   * @default 10
   */
  size: number
}
export const newGeneratorOption = (obj?: Param<GeneratorOption>): GeneratorOption => ({
  outputFormat: newCsvFormat(),
  columnOptions: {},
  columnOptionsDefault: { num: newNumericColumnOption(), str: newStringColumnOption(), date: newDatetimeColumnOption(), bool: newBooleanColumnOption() },
  size: 10,
  ...obj,
  _tag: 'GeneratorOption',
});
type ColumnOptionDefaultType = { num: NumericColumnOption, str: StringColumnOption, date: DatetimeColumnOption, bool: BooleanColumnOption };
/** CsvFormat is used for GneratorOption.outputFormat */
export interface CsvFormat {
  readonly _tag: 'CsvFormat'
  /** Delimiter of each column. @default ',' */
  delimiter: string
  /** Quote for each value. @default '"' */
  quote: string
  /** Escape sequence. @default '"' */
  escapeSequence: string
  /** Whether output header or not. @default false */
  header: boolean
}
export const newCsvFormat = (obj?: Partial<CsvFormat>): CsvFormat => ({
  delimiter: `,`,
  quote: `"`,
  escapeSequence: `"`,
  header: false,
  ...obj,
  _tag: 'CsvFormat',
});
/** InsertStatementFormat is used for GneratorOption.outputFormat */
export interface InsertStatementFormat {
  readonly _tag: 'InsertStatementFormat'
}
export const newInsertStatementFormat = (obj?: Param<InsertStatementFormat>): InsertStatementFormat => ({
  ...obj,
  _tag: 'InsertStatementFormat',
});
export const NUM_LOOP_OPTS = ['loop','negate','keep'] as const;
/** NumericColumnOptions is used for GeneratorOption.columnOptions */
export interface NumericColumnOption {
  readonly _tag: 'NumericColumnOption'
  /** How much advance per each row. @default 1 for integers, 0.1 for decimals */
  stepBy?: number
  /** Value of the first row. @default 1 */
  initialValue: number|((col:number)=>number)
  /** Limit of incrementation. @default depend on the corresponding table data type. */
  limit?: number
  /**
   * How to behave when incrementation hits the limit. @default loop.
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
  _tag: 'NumericColumnOption',
});
type IntegerColumnOption = Omit<OmitTag<NumericColumnOption>, 'stepBy'> & {
  readonly _tag: 'IntegerColumnOption'
  stepBy: number
  /** @internal */
  __initialValueNum: number
  /** @internal */
  __maxScale: number,
}
export const newIntegerColumnOption = (obj?: Param<IntegerColumnOption>): IntegerColumnOption => {
  const ret: IntegerColumnOption = {
    ...newNumericColumnOption(obj),
    stepBy: obj?.stepBy || 1,
    __initialValueNum: obj?.__initialValueNum || 1,
    __maxScale: 0,
    _tag: 'IntegerColumnOption',
  };
  const scale1 = (ret.__initialValueNum.toString().split('.')[1] || '').length;
  const scale2 = (ret.stepBy.toString().split('.')[1] || '').length;
  const __maxScale = Math.max(scale1,scale2);
  return { ...ret, __maxScale };
};
type DecimalColumnOption = Omit<OmitTag<NumericColumnOption>, 'stepBy'> & {
  readonly _tag: 'DecimalColumnOption'
  stepBy: number
  precision:number,
  scale:number,
  /** @internal */
  __initialValueNum: number
  /** @internal */
  __maxScale: number,
}
export const newDecimalColumnOption = (obj?: Param<DecimalColumnOption>): DecimalColumnOption => {
  const ret: DecimalColumnOption = {
    ...newNumericColumnOption(obj),
    stepBy: obj?.stepBy || 0.1,
    precision: Infinity,
    scale: 0,
    __initialValueNum: obj?.__initialValueNum || 1,
    __maxScale: 0,
    _tag: 'DecimalColumnOption',
  };
  const scale1 = (ret.__initialValueNum.toString().split('.')[1] || '').length;
  const scale2 = (ret.stepBy.toString().split('.')[1] || '').length;
  const __maxScale = Math.max(scale1,scale2);
  return { ...ret, __maxScale };
};
export const STR_LOOP_OPTS = ['loop','keep'] as const;
export const LENGTH_IN_OPTS = ['char','byte'] as const;
/** StringColumnOption is used for GeneratorOption.columnOptions */
export interface StringColumnOption {
  readonly _tag: 'StringColumnOption',
  /** Limit of incrementation. @default depend on the corresponding table data type. */
  maxLength: number,
  /**
   * Which measurement unit to use, either char or byte.
   * @default char for character string type column, byte for byte string type column
   */
  lengthIn?: typeof LENGTH_IN_OPTS[number],
  /** Prefix. @default a character in A-Z, a-z, depending on the column position */
  prefix: Prefix,
  /**
   * How to behave when incrementation hits the limit. @default loop.
   *   loop: back to the initial value and continue to increment
   *   keep: stop incrementation and keep the limit value
   */
  loop: typeof STR_LOOP_OPTS[number],
}
export const newStringColumnOption = (obj?: Omit<Param<StringColumnOption>, '__prefixStr'>): StringColumnOption => ({
  maxLength: 0,
  lengthIn: undefined,
  prefix: '',
  loop: 'loop',
  ...obj,
  _tag: 'StringColumnOption',
});
type CharColumnOption = OmitTag<StringColumnOption> & {
  readonly _tag: 'CharColumnOption'
  lengthIn: typeof LENGTH_IN_OPTS[number],
  /** @internal */
  __prefixStr: string,
  /** @internal */
  __prefixStrLength: number,
}
export const newCharColumnOption = (obj?: Param<CharColumnOption>): CharColumnOption => {
  const temp: CharColumnOption = {
    ...newStringColumnOption(obj),
    lengthIn: obj?.lengthIn || 'char',
    __prefixStr: obj?.__prefixStr || '',
    __prefixStrLength: obj?.__prefixStrLength || 0,
    _tag: 'CharColumnOption',
  };
  const [__prefixStr, __prefixStrLength] = calcPrefix(temp.lengthIn, temp.__prefixStr, temp.maxLength);
  return { ...temp, __prefixStr, __prefixStrLength };
};
type BinaryColumnOption = OmitTag<StringColumnOption> & {
  readonly _tag: 'BinaryColumnOption'
  lengthIn: typeof LENGTH_IN_OPTS[number],
  /** @internal */
  __prefixStr: string,
  /** @internal */
  __prefixStrLength: number,
}
export const newBinaryColumnOption = (obj?: Param<BinaryColumnOption>): BinaryColumnOption => {
  const temp: BinaryColumnOption = {
    ...newStringColumnOption(obj),
    lengthIn: obj?.lengthIn || 'byte',
    __prefixStr: obj?.__prefixStr || '',
    __prefixStrLength: obj?.__prefixStrLength || 0,
    _tag: 'BinaryColumnOption',
  };
  const [__prefixStr, __prefixStrLength] = calcPrefix(temp.lengthIn, temp.__prefixStr, temp.maxLength);
  return { ...temp, __prefixStr, __prefixStrLength };
};
const calcPrefix = (lengthIn: typeof LENGTH_IN_OPTS[number], basePrefix: string, maxLength: number): [string, number] => {
  if (lengthIn === 'char') {
    // https://dev.to/coolgoose/quick-and-easy-way-of-counting-utf-8-characters-in-javascript-23ce
    const destructed = basePrefix.split(/(\P{Mark}\p{Mark}*)/u).filter(chr => chr).filter((_,index) => index < maxLength-1);
    const __prefixStr = destructed.join('');
    const __prefixStrLength = destructed.length;
    return [__prefixStr, __prefixStrLength];
  } else {
    const encoder = new TextEncoder();
    const prefixU8: Readonly<Uint8Array> = encoder.encode(basePrefix);
    const [__prefixStr, __prefixStrLength] = ((): [string, number] => {
      const decoder = new TextDecoder();
      let endIdx;
      for (endIdx=maxLength-1; endIdx>0; endIdx--){
        if ((prefixU8[endIdx] & 0xC0) !== 0x80) break;
      }
      const slice = prefixU8.slice(0, endIdx);
      const decoded = decoder.decode(slice);
      const decodedLength = decoded.split(/(\P{Mark}\p{Mark}*)/u).filter(chr => chr).length;
      const destructed = basePrefix.split(/(\P{Mark}\p{Mark}*)/u).filter(chr => chr);
      const slicedDestructed = destructed.filter((_,index) => index < decodedLength);
      const prefixStr = decoded === slicedDestructed.join('') ? decoded : destructed.filter((_,index) => index<slicedDestructed.length-1).join('');
      return [prefixStr, encoder.encode(prefixStr).length];
    })();
    return [__prefixStr, __prefixStrLength];
  }
};
/** DatetimeColumnOption is used for GeneratorOption.columnOptions */
export interface DatetimeColumnOption {
  readonly _tag: 'DatetimeColumnOption',
  /**
   * Value of the first row.
   * @default 1970-01-01T00:00:00.000Z. internal values are same but output format depends on their column types, as shown below.
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
   *  @internal
   */
  stepBy: number,
}
export const newDatetimeColumnOption = (obj?: Param<Omit<DatetimeColumnOption, 'stepBy'>>): DatetimeColumnOption => ({
  initialValue: new Date(0),
  stepBy: 1,
  ...obj,
  _tag: 'DatetimeColumnOption',
});
type DateColumnOption = OmitTag<DatetimeColumnOption> & {
  readonly _tag: 'DateColumnOption'
}
export const newDateColumnOption = (obj?: Param<Omit<DateColumnOption, 'stepBy'>>): DateColumnOption => ({
  ...newDatetimeColumnOption(obj),
  _tag: 'DateColumnOption',
});
type TimeColumnOption = OmitTag<DatetimeColumnOption> & {
  /** @internal */
  readonly _tag: 'TimeColumnOption'
}
export const newTimeColumnOption = (obj?: Param<Omit<TimeColumnOption, 'stepBy'>>): TimeColumnOption => ({
  ...newDatetimeColumnOption(obj),
  _tag: 'TimeColumnOption',
});
type TimestampColumnOption = OmitTag<DatetimeColumnOption> & {
  /** @internal */
  readonly _tag: 'TimestampColumnOption'
}
export const newTimestampColumnOption = (obj?: Param<Omit<TimestampColumnOption, 'stepBy'>>): TimestampColumnOption => ({
  ...newDatetimeColumnOption(obj),
  _tag: 'TimestampColumnOption',
});
/** BooleanColumnOption is used for GeneratorOption.columnOptions */
export interface BooleanColumnOption {
  readonly _tag: 'BooleanColumnOption',
  /** Value of the first row. @default false */
  initialValue: boolean,
  /** Whether randomly generate data or not. @default false */
  random: boolean,
  /** Whether use null value or not. If table column has not-null constraint, this option is ignored. @default false */
  useNull: boolean,
}
export const newBooleanColumnOption = (obj?: Param<BooleanColumnOption>): BooleanColumnOption => ({
  initialValue: false,
  random: false,
  useNull: false,
  ...obj,
  _tag: 'BooleanColumnOption',
});
/** FixedValue is use for GneratorOption.columnOptions */
export interface FixedValue {
  /** @internal */
  readonly _tag: 'FixedValue'
  /** The value to be set to a column */
  value: string;
}
export const newFixedValue = (obj: Omit<FixedValue, '_tag'>): FixedValue => ({
  ...obj,
  _tag: 'FixedValue',
});
type ColumnsType = {num:number[], str:string[], date:Date[], bool:(boolean|undefined)[], fixed: (string|undefined)[]};
type NullableColumnsType = {num:(number|undefined)[], str:(string|undefined)[], date:(Date|undefined)[], bool:(boolean|undefined)[], fixed: (string|undefined)[]};
type KeysInUseType = { [keyName: string]: string[] };
type NameToIdx={[key:string]:number};
type Keys={keyName: string, keys: string[]};
type RowProcess = {
  row: number
  prevColumns: ColumnsType
  keysInUse: KeysInUseType
  constraints: TableConstraint[]
  primaryKeys?: Keys
  uniqueKeysSet: Keys[]
  nameToIdx: NameToIdx
  names: string[]
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
  const result = parser.parse(src);
  if (result instanceof ParseError) throw result;
  const stmts = result;
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
  const result = parser.parse(src);
  if (result instanceof ParseError) throw result;
  const stmts = result;
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
  logger.log('CreateTableStatement:', statement);
  logger.log('GeneratorOption:', option);
  const columnDefs = statement.columns;
  let prevColumns: ColumnsType = {num:[],str:[],date:[],bool:[],fixed:[]};
  let strPrefixCodePoint:number=65;
  type AvailableOptionUnion = IntegerColumnOption|DecimalColumnOption|CharColumnOption|BinaryColumnOption|DateColumnOption|TimeColumnOption|TimestampColumnOption|BooleanColumnOption|FixedValue;
  const opts: AvailableOptionUnion[] = columnDefs.map((def,i) => {
    const colOption = option.columnOptions[def.name.value];
    if (colOption && colOption._tag === 'FixedValue') return colOption;
    const dataType = def.dataType;
    let opt;
    if (dataType instanceof types.NumericType) {
      if (colOption && !(colOption._tag === 'NumericColumnOption')) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: NumericColumnOption = colOption || option.columnOptionsDefault.num;
      const __initialValueNum = typeof nonNullOpt.initialValue === 'function' ? nonNullOpt.initialValue(i) : nonNullOpt.initialValue;
      if (dataType instanceof types.Float) {
        let limit = dataType.precision ? +('9'.repeat(dataType.precision)) : Infinity;
        limit = Math.min(limit, nonNullOpt.limit||Infinity);
        opt = newDecimalColumnOption({ precision: dataType.precision, ...nonNullOpt, __initialValueNum, limit });
        prevColumns.num[i]=subtract(opt.__initialValueNum, opt.stepBy, opt.__maxScale);
      } else if (dataType instanceof types.DecimalType) {
        let limit = dataType.precision ? +('9'.repeat(dataType.precision - (dataType.scale || 0))) : Infinity;
        limit = Math.min(limit, nonNullOpt.limit||Infinity);
        logger.log('stepBy', nonNullOpt.stepBy);
        opt = newDecimalColumnOption({ precision: dataType.precision, scale: dataType.scale, ...nonNullOpt, __initialValueNum, limit });
        logger.log('stepBy', opt.stepBy);
      } else {
        opt = newIntegerColumnOption({...nonNullOpt, __initialValueNum});
        prevColumns.num[i]=opt.__initialValueNum - opt.stepBy;
      }
      prevColumns.num[i]=subtract(opt.__initialValueNum, opt.stepBy, opt.__maxScale);
    } else if (dataType instanceof types.StringType) {
      if (colOption && !(colOption._tag === 'StringColumnOption')) throw new GeneratorFatalError('invalid column option');
      const isChar = dataType instanceof types.CharacterStringType;
      const nonNullOpt: StringColumnOption = colOption || option.columnOptionsDefault.str;
      const maxLength = nonNullOpt.maxLength || dataType.length || 10; // There is no reason for 10 but we need a concrete integer value.
      logger.log('maxLength', dataType.length);
      let __prefixStr:string = typeof nonNullOpt.prefix === 'function' ? nonNullOpt.prefix(i,def.name.value) : nonNullOpt.prefix;
      __prefixStr = __prefixStr || String.fromCodePoint(strPrefixCodePoint++).slice(0,maxLength-1);
      const __prefixStrLength = __prefixStr.length;
      opt = isChar ? newCharColumnOption({...nonNullOpt, maxLength, __prefixStr, __prefixStrLength}) :
                     newBinaryColumnOption({...nonNullOpt, maxLength, __prefixStr, __prefixStrLength});
      const initialValue:string = '0'.repeat(maxLength - opt.__prefixStrLength);
      logger.log('initialValue', initialValue);
      logger.log('maxLength', maxLength);
      prevColumns.str[i]=initialValue;
    } else if (dataType instanceof types.DatetimeType) {
      if (colOption && !(colOption._tag === 'DatetimeColumnOption')) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: DatetimeColumnOption = colOption || option.columnOptionsDefault.date;
      opt = dataType instanceof types.Date ? newDateColumnOption({ ...nonNullOpt }) :
            dataType instanceof types.Time ? newTimeColumnOption({ ...nonNullOpt }) :
                                             newTimestampColumnOption({ ...nonNullOpt });
      prevColumns.date[i]= dataType instanceof types.Time ? opt.initialValue : new Date(nonNullOpt.initialValue.getTime() - 24*60*60*1000);
    } else if (dataType instanceof types.BooleanType) {
      if (colOption && !(colOption._tag === 'BooleanColumnOption')) throw new GeneratorFatalError('invalid column option');
      const nonNullOpt: BooleanColumnOption = colOption || option.columnOptionsDefault.bool;
      opt = newBooleanColumnOption({ ...nonNullOpt });
      prevColumns.bool[i]=!opt.initialValue;
    } else {
      throw new GeneratorFatalError('invalid data type');
    }
    return opt;
  });
  logger.log('opts:', opts);
  const tableColOpts: co.ColumnOption[][] = columnDefs.map(def => def.options.map(opt => opt.option));
  const constraints= statement.constraints;
  const nameToIdx = columnDefs.reduce<NameToIdx>((prev,crr,i) => {
    const next = prev;
    next[crr.name.value]=i;
    return prev;
  }, {});
  const names = columnDefs.map(colDef => colDef.name.value);
  let keysInUse;
  const [primaryKeys, uniqueKeysSet] = [,,keysInUse] = ((): [primaryKeys: Keys|undefined, uniqueKeysSet: Keys[], keysInUse: KeysInUseType] => {
    const pksTmp: string[] = [];
    const uks: Keys[] = [];
    const keysInUse: KeysInUseType = {};
    for(const constraint of constraints) {
      if (constraint instanceof parser.Unique) {
        const keys = constraint.columns.map(col => col.value);
        if (constraint.isPrimary) {
          pksTmp.push(...constraint.columns.map(col => col.value));
        } else {
          const keyName = keys.join();
          uks.push({ keyName, keys });
          keysInUse[keyName] = [];
        }
      }
    }
    for(const defs of columnDefs) {
      for(const { option } of defs.options) {
        if (option instanceof co.Unique) {
          const keyName = defs.name.value;
          if (option.isPrimary) {
            pksTmp.push(keyName);
          } else {
            uks.push({ keyName, keys: [keyName] });
            keysInUse[keyName] = [];
          }
        }
      }
    }
    const pks = pksTmp.length > 0 ? { keyName: pksTmp.join(), keys: pksTmp } : undefined;
    if (pks) keysInUse[pks.keyName] = [];
    return [pks, uks, keysInUse];
  })();
  const tableName = statement.name.value.map(ident=>ident.value).join('.');
  if (option.outputFormat._tag === 'CsvFormat' && option.outputFormat.header) {
    const colNames = columnDefs.map(col => col.name.value);
    const header = colNames.join(',');
    yield [{ columns: colNames, row: header }, [] ];
  }
  for(let i=0; i<option.size; i++) {
    const errors: GeneratorValidationError[] = [];
    let temp={};
    const columns: ColumnsType = {num:[],str:[],date:[],bool:[],fixed:[]};
    const rowProcess: RowProcess = {
      row: i,
      keysInUse,
      prevColumns,
      constraints,
      primaryKeys,
      uniqueKeysSet,
      nameToIdx,
      names,
    };
    // generate data
    for(let j=0; j<opts.length; j++) {
      const colProcess: ColumnProcess = {
        ...rowProcess,
        columnName: columnDefs[j].name.value,
        col: j,
      };
      const opt = opts[j];
      if (opt._tag === 'IntegerColumnOption' || opt._tag === 'DecimalColumnOption') columns.num[j]=(generateNumeric(colProcess, opt));
      if (opt._tag === 'CharColumnOption' || opt._tag === 'BinaryColumnOption') columns.str[j]=(generateString(colProcess, opt));
      if (opt._tag === 'DateColumnOption' || opt._tag === 'TimeColumnOption' || opt._tag === 'TimestampColumnOption') columns.date[j]=(generateDatetime(colProcess, opt));
      if (opt._tag === 'BooleanColumnOption') columns.bool[j]=(generateBoolean(colProcess, opt));
      if (opt._tag === 'FixedValue') columns.fixed[j]=opt.value;
    }
    // execute callback function
    let modifiedColumns: NullableColumnsType = columns;
    if (rowProcess && option.eachRow) {
      [modifiedColumns,temp] = option.eachRow(cloneColumns(columns), rowProcess, temp);
    }
    // validate columns
    for(let j=0; j<opts.length; j++) {
      const colProcess: ColumnProcess = {
        ...rowProcess,
        columnName: columnDefs[j].name.value,
        col: j,
      };
      let error: GeneratorValidationError|undefined;
      const opt = opts[j];
      if (opt._tag === 'IntegerColumnOption' || opt._tag === 'DecimalColumnOption') error = validateNumeric(colProcess, modifiedColumns.num[j], tableColOpts[j]);
      if (opt._tag === 'CharColumnOption' || opt._tag === 'BinaryColumnOption') error = validateString(colProcess, modifiedColumns.str[j], tableColOpts[j]);
      if (opt._tag === 'DateColumnOption' || opt._tag === 'TimeColumnOption' || opt._tag === 'TimestampColumnOption') error = validateDatetime(colProcess, modifiedColumns.date[j], tableColOpts[j]);
      if (opt._tag === 'BooleanColumnOption') error = validateBoolean(colProcess, modifiedColumns.bool[j], tableColOpts[j]);
      if (opt._tag === 'FixedValue') error = validateFixedValue(colProcess, modifiedColumns.fixed[j], tableColOpts[j]);
      if (error) errors.push(error);
    }
    // convert to string
    const columnsStr: (string|undefined)[] = [];
    for(let j=0; j<opts.length; j++) {
      const opt = opts[j];
      columnsStr[j] = (() => {
        if (opt._tag === 'IntegerColumnOption' || opt._tag === 'DecimalColumnOption') return modifiedColumns.num[j]?.toFixed(opt.__maxScale);
        if (opt._tag === 'CharColumnOption' || opt._tag === 'BinaryColumnOption') return modifiedColumns.str[j];
        const date=modifiedColumns.date[j];
        if (opt._tag === 'DateColumnOption') return date?.toISOString().slice(0,10);
        if (opt._tag === 'TimeColumnOption') return date && `${Math.floor(date.getTime() / (60*60*1000)) + date.getUTCHours()}:${date.getUTCMinutes()}:${date.getUTCSeconds()}`;
        if (opt._tag === 'TimestampColumnOption') return date?.toISOString().slice(0,19).replace('T',' ');
        const bool=modifiedColumns.bool[j];
        if (opt._tag === 'BooleanColumnOption') return bool === undefined ? bool : bool.toString();
        const fixed=modifiedColumns.fixed[j];
        return fixed ? fixed : 'null';
      })();
    }
    const [,errs] = [keysInUse] = validateRow(rowProcess, columnsStr);
    if (errs) errors.push(...errs);
    const row = option.outputFormat._tag === 'CsvFormat' ? toCsv(columnsStr, option.outputFormat) : toInsert(columnsStr, tableName);
    prevColumns=columns;
    yield [{ columns: columnsStr, row }, errors];
  }
}

// TODO negative step
const generateNumeric = (process: ColumnProcess, option: IntegerColumnOption|DecimalColumnOption): number => {
  const prev=process.prevColumns.num[process.col];
  // logger.log('add', prev, option.stepBy, maxScale);
  const next = add(prev, option.stepBy, option.__maxScale);
  if (next > (option.limit||Infinity)) { // overflow
    return option.loop === 'loop'   ? option.__initialValueNum :
           option.loop === 'negate' ? -prev:
                                      prev;
  }
  return next;
};
const generateString = (process: ColumnProcess, option: CharColumnOption|BinaryColumnOption): string => {
  const prev=process.prevColumns.str[process.col];
  const suffixLength = option.maxLength - option.__prefixStrLength;
  let seq=parseInt(prev.slice(prev.length - suffixLength))+1;
  if (option.maxLength !== Infinity && seq > +('9'.repeat(suffixLength))) { // overflow
    seq = option.loop === 'loop' ? 1 : seq-1;
  }
  return option.__prefixStr + '0'.repeat(suffixLength - seq.toString().length) + seq;
};
const generateDatetime = (process: ColumnProcess, option: DateColumnOption|TimeColumnOption|TimestampColumnOption): Date => {
  const prev=process.prevColumns.date[process.col];
  if (option._tag === 'TimeColumnOption') {
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
// TODO improve validation
const validateNumeric = (process: ColumnProcess, column: number|undefined, colOpts: co.ColumnOption[]): GeneratorValidationError|undefined => {
  return checkNullConstraint(process, column?.toString(), colOpts);
};
const validateString = (process: ColumnProcess, column: string|undefined, colOpts: co.ColumnOption[]): GeneratorValidationError|undefined => {
  return checkNullConstraint(process, column?.toString(), colOpts);
};
const validateDatetime = (process: ColumnProcess, column: Date|undefined, colOpts: co.ColumnOption[]): GeneratorValidationError|undefined => {
  return checkNullConstraint(process, column?.toString(), colOpts);
};
const validateBoolean = (process: ColumnProcess, column: boolean|undefined, colOpts: co.ColumnOption[]): GeneratorValidationError|undefined => {
  return checkNullConstraint(process, column?.toString(), colOpts);
};
const validateFixedValue = (process: ColumnProcess, column: string|undefined, colOpts: co.ColumnOption[]): GeneratorValidationError|undefined => {
  return checkNullConstraint(process, column, colOpts);
};
const checkNullConstraint = (process: ColumnProcess, column: string|undefined, colOpts: co.ColumnOption[]): GeneratorValidationError|undefined => {
  const notNull = colOpts.some(o => o instanceof co.NotNull);
  const isPK = process.primaryKeys && process.primaryKeys.keys.includes(process.columnName);
  if (!column && (notNull || isPK)) return new GeneratorValidationError('Violated not-null constraint at column: ' + process.columnName + ', row: ' + (process.row+1));
};
const validateRow = (process: RowProcess, columns: (string|undefined)[]): [keysInUse: KeysInUseType, errors?: GeneratorValidationError[]] => {
  const errors:GeneratorValidationError[]=[];
  if (process.primaryKeys) {
    const value = process.primaryKeys.keys.reduce((prv, crr) => prv+(columns[process.nameToIdx[crr]]||''), '');
    const keyInUse: string[] = process.keysInUse[process.primaryKeys.keyName] || [];
    if (keyInUse.includes(value)) errors.push(new GeneratorValidationError('Violated unique-key constraint at row: ' + (process.row+1)));
    keyInUse.push(value);
  }
  process.uniqueKeysSet.forEach(uKeys => {
    const value = uKeys.keys.reduce((prv, crr) => prv+(columns[process.nameToIdx[crr]]||''), '');
    const keyInUse: string[] = process.keysInUse[uKeys.keyName] || [];
    if (keyInUse.includes(value)) errors.push(new GeneratorValidationError('Violated unique-key constraint at row: ' + (process.row+1)));
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
const cloneColumns = (columns: ColumnsType): ColumnsType => {
  return {
    num: [...columns.num],
    str: [...columns.str],
    date: columns.date.map(d => new Date(d.getTime())),
    bool: [...columns.bool],
    fixed: [...columns.fixed],
  };
};

