// Data type definitions do not strictly follow the ANSI SQL Standards
// We only support major data types.
// Data type definition might not strictly follow the spec. E.g., `length` of CHAR is optional according to the spec but we enforce to define `length`.

const DATA_TYPE_NAMES_L = [
  'CHAR',
  'CHARACTER',
//  'CHAR VARYING', // not supported
//  'CHARACTER VARYING', // not supported
  'VARCHAR',
  'CLOB',
  'BINARY',
  'VARBINARY',
  'BLOB',
] as const;
type DataTypeNameL = typeof DATA_TYPE_NAMES_L[number];
const DATA_TYPE_NAMES_OPT_P_S = [
  'DECIMAL',
  'DEC',
  'NUMBER',
  'NUMERIC',
] as const;
type DataTypeNameOptPS = typeof DATA_TYPE_NAMES_OPT_P_S[number];
const DATA_TYPE_NAMES_OPT_P = [
  'FLOAT',
] as const;
type DataTypeNameOptP = typeof DATA_TYPE_NAMES_OPT_P[number];
const DATA_TYPE_NAMES_NO_ARGS = [
//  'UUID', // not supported
  'SMALLINT',
  'INT',
  'INTEGER',
  'BIGINT',
  'REAL',
  'DOUBLE PRECISION',
  'DOUBLE',
  'BOOLEAN',
  'DATE',
  'TIME',
  'TIMESTAMP',
  'INTERVAL',
// 'REGCLASS', // not supported
  'TEXT',
  'BYTEA',
// 'CUSTOM', // not supported
// 'ARRAY', // not supported
] as const;
type DataTypeNameNoArgs = typeof DATA_TYPE_NAMES_NO_ARGS[number];
const DATA_TYPE_NAMES = [
  ...DATA_TYPE_NAMES_L,
  ...DATA_TYPE_NAMES_OPT_P_S,
  ...DATA_TYPE_NAMES_OPT_P,
  ...DATA_TYPE_NAMES_NO_ARGS,
] as const;
type DataTypeName = typeof DATA_TYPE_NAMES[number];

export class DataType {
  constructor(public name: DataTypeName) {}
}
export class Char extends DataType { // Fixed-length character type e.g. CHAR(10)
  constructor(public length: number) { super('CHAR'); }
}
export class Character extends DataType { // Alias for Char
  constructor(public length: number) { super('CHARACTER'); }
}
export class Varchar extends DataType { // Variable-length character type e.g. VARCHAR(10)
  constructor(public length: number) { super('VARCHAR'); }
}
export class Clob extends DataType { // Large character object e.g. CLOB(1000)
  constructor(public length: number) { super('CLOB'); }
}
export class Binary extends DataType { // Fixed-length binary type e.g. BINARY(10)
  constructor(public length: number) { super('BINARY'); }
}
export class Varbinary extends DataType { // Variable-length binary type e.g. VARBINARY(10)
  constructor(public length: number) { super('VARBINARY'); }
}
export class Blob extends DataType { // Large binary object e.g. BLOB(1000)
  constructor(public length: number) { super('BLOB'); }
}
export class Decimal extends DataType { // Decimal type with optional precision and scale e.g. DECIMAL(10,2)
  constructor( public precision?: number, public scale?: number) { super('DECIMAL'); }
}
export class Dec extends DataType { // Alias for Decimal type
  constructor( public precision?: number, public scale?: number) { super('DEC'); }
}
export class Number extends DataType { // Alias for Decimal type
  constructor( public precision?: number, public scale?: number) { super('NUMBER'); }
}
export class Numeric extends DataType { // Alias for Decimal type
  constructor( public precision?: number, public scale?: number) { super('NUMERIC'); }
}
export class Float extends DataType { // Floating point with optional precision e.g. FLOAT(8)
  constructor( public precision?: number) { super('FLOAT'); }
}
export class SmallInt extends DataType { // Small integer
  constructor() { super('SMALLINT'); }
}
export class Int extends DataType { // Integer
  constructor() { super('INT'); }
}
export class Integer extends DataType { // Integer
  constructor() { super('INTEGER'); }
}
export class BigInt extends DataType { // Big integer
  constructor() { super('BIGINT'); }
}
export class Real extends DataType { // Floating point e.g. REAL
  constructor() { super('REAL'); }
}
export class DoublePrecision extends DataType { // Double
  constructor() { super('DOUBLE PRECISION'); }
}
export class Double extends DataType { // alias for DoublePrecision
  constructor() { super('DOUBLE'); }
}
export class Boolean extends DataType { // Boolean
  constructor() { super('BOOLEAN'); }
}
export class Date extends DataType { // Date
  constructor() { super('DATE'); }
}
export class Time extends DataType { // Time
  constructor() { super('TIME'); }
}
export class Timestamp extends DataType { // Timestamp
  constructor() { super('TIMESTAMP'); }
}
export class Interval extends DataType { // Interval
  constructor() { super('INTERVAL'); }
}
export class Text extends DataType { // Text
  constructor() { super('TEXT'); }
}
export class Bytea extends DataType { // Bytea
  constructor() { super('BYTEA'); }
}

const SMALLINT = new SmallInt;
const INT = new Int;
const INTEGER = new Integer;
const BIGINT = new BigInt;
const REAL = new Real;
const DOUBLE_PRECISION = new DoublePrecision;
const DOUBLE = new Double;
const BOOLEAN = new Boolean;
const DATE = new Date;
const TIME = new Time;
const TIMESTAMP = new Timestamp;
const INTERVAL = new Interval;
const TEXT = new Text;
const BYTEA = new Bytea;

export const mapperL: { [key in DataTypeNameL]: (length: number) => DataType } = {
  'CHAR':      (length: number) => new Char(length),
  'CHARACTER': (length: number) => new Character(length),
  'VARCHAR':   (length: number) => new Varchar(length),
  'CLOB':      (length: number) => new Clob(length),
  'BINARY':    (length: number) => new Binary(length),
  'VARBINARY': (length: number) => new Varbinary(length),
  'BLOB':      (length: number) => new Blob(length),
};
export const mapperOptPS: { [key in DataTypeNameOptPS]: (precision?: number, scale?: number) => DataType } = {
  'DECIMAL':   (precision?: number, scale?: number) => new Decimal(precision, scale),
  'DEC':       (precision?: number, scale?: number) => new Dec(precision, scale),
  'NUMBER':    (precision?: number, scale?: number) => new Number(precision, scale),
  'NUMERIC':   (precision?: number, scale?: number) => new Numeric(precision, scale),
};
export const mapperOptP: { [key in DataTypeNameOptP]: (precision?: number) => DataType } = {
  'FLOAT':     (precision?: number) => new Float(precision),
};
export const mapperNoArgs: { [key in DataTypeNameNoArgs]: (lenOrPr: number, scale: number) => DataType } = {
  'SMALLINT':         () => SMALLINT,
  'INT':              () => INT,
  'INTEGER':          () => INTEGER,
  'BIGINT':           () => BIGINT,
  'REAL':             () => REAL,
  'DOUBLE PRECISION': () => DOUBLE_PRECISION,
  'DOUBLE':           () => DOUBLE,
  'BOOLEAN':          () => BOOLEAN,
  'DATE':             () => DATE,
  'TIME':             () => TIME,
  'TIMESTAMP':        () => TIMESTAMP,
  'INTERVAL':         () => INTERVAL,
  'TEXT':             () => TEXT,
  'BYTEA':            () => BYTEA,
};

export const isDataTypeName = (name: string|DataTypeName): name is DataTypeName => DATA_TYPE_NAMES.includes(name as DataTypeName); // narrow down string to DataTypeName
export const isDataTypeNameL = (name: string|DataTypeNameL): name is DataTypeNameL => DATA_TYPE_NAMES_L.includes(name as DataTypeNameL);
export const isDataTypeNameOptPS = (name: string|DataTypeNameOptPS): name is DataTypeNameOptPS => DATA_TYPE_NAMES_OPT_P_S.includes(name as DataTypeNameOptPS);
export const isDataTypeNameOptP = (name: string|DataTypeNameOptP): name is DataTypeNameOptP => DATA_TYPE_NAMES_OPT_P.includes(name as DataTypeNameOptP);
export const isDataTypeNameNoArgs = (name: string|DataTypeNameNoArgs): name is DataTypeNameNoArgs => DATA_TYPE_NAMES_NO_ARGS.includes(name as DataTypeNameNoArgs);

