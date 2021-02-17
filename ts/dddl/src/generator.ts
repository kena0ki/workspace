/** Options for data to be generated. */
export type GeneratorOptions = {
  /** Output format. Either csv or insert statement. */
  outputFormat: CsvFormat|InsertStatementFormat,
  /**
   *  Options for each column to determine how they are generated.
   *  If you want fixed values, just specify the values you want to set.
   */
  columnOptions: IntegerColumnOption[],
  /** A function to manipulate row data after it's generated. */
  eachRow: (rowCount: number, row: string[], prev: object) => string
}
/** CsvFormat is used for GeneratorOptions.outputFormat */
export class CsvFormat {
  /** Delimiter of each columns. Default: ',' */
  public delimiter?: string = `,`
  /** Quote for each items. Default: '"' */
  public quote?: string = `"`
  /** Escape sequence. Default: '"' */
  public escapeSequence?: string = `"`
  /** Define options */
  public define(obj: OnlyOfType<string|number,CsvFormat>): this {
    (Object.keys(obj) as [keyof CsvFormat]).forEach(key => {
      if (isOfTypeNumber<CsvFormat>(key)) this[key] = obj[key];
      if (isOfTypeString<CsvFormat>(key)) this[key] = obj[key];
    });
    return this;
  }
}
/** InsertStatementFormat is used for GeneratorOptions.outputFormat */
export class InsertStatementFormat {
}
/** IntegerColumnOptions is used for GeneratorOptions.columnOptions */
export class IntegerColumnOption {
  /** How much advance for each rows. Default: 1 */
  public stepBy: number = 1
  public initialValue: number = 1
  public limit: number = Infinity
  public loop: boolean = true
}
/** FloatColumnOptions is used for GeneratorOptions.columnOptions */
export class FloatColumnOption {
  /** How much advance for each rows. Default: 0.1 */
  public stepBy: number = 0.1
  public initialValue: number = 1
  public limit: number = Infinity
  public loop: boolean = true
}
/** CharColumnOptions is used for GeneratorOptions.columnOptions */
export class CharColumnOption {
  public maxLength: number = Infinity
  public prefix: string = ''
  public suffix: string = ''
  public loop: boolean = true
}
/** BinaryColumnOptions is used for GeneratorOptions.columnOptions */
export class BinaryColumnOptions {
  // TODO
}
/** DateColumnOptions is used for GeneratorOptions.columnOptions */
export class DateColumnOptions {
  // TODO
}
type OnlyKeysOfType<T,O> = Extract<keyof O, {[K in keyof O]: O[K] extends T|undefined ? K : never}[keyof O]>
type OnlyOfType<T,O> = {[K in OnlyKeysOfType<T,O>]?: O[K]};
function isOfTypeNumber<K>(key: keyof K): key is OnlyKeysOfType<number,K> { return typeof key === 'number'; }
function isOfTypeString<K>(key: keyof K): key is OnlyKeysOfType<string,K> { return typeof key === 'string'; }

