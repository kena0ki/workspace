import { types } from '../data-types';

export const KEYWORDS = [ // only contains keywords that is necessary for create statements
  'CREATE',
  'TABLE',
  'OR',
  'REPLACE',
  'IF',
  'NOT',
  'EXISTS',
  'CONSTRAINT',
  'PRIMARY',
  'UNIQUE',
  'KEY',
  'FOREIGN',
  'REFERENCES',
  'CHECK',
  'AND',
  'OR',
  'LIKE',
  'NOT',
  'IN',
  'IS',
  'NULL',
  'BETWEEN',
  'SELECT',
  'WITH',
  'WITHOUT',
  'PRECISION',
  'ZONE',
  'TRUE',
  'FALSE',
  'CASE',
  'WHEN',
  'ELSE',
  'END',
  'CAST',
  'AS',
  'EXTRACT',
  'INTERVAL', // should be defined as a data type name, but we are commenting it out there since we are not supporting it now
  'LISTAGG',
  ...types.DATA_TYPE_NAMES,
] as const;
export type Keyword = typeof KEYWORDS[number];
export function isKeyword<T extends Keyword>(src: string, keyword: T): src is T {
  return src.toUpperCase() === keyword;
}
export function isOneOfKeywords<T extends Keyword>(src: string, keywords: T[]): src is T {
  return keywords.some(keyword => src.toUpperCase() === keyword);
}

