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
  ...types.DATA_TYPE_NAMES,
] as const;
export type Keyword = typeof KEYWORDS[number];
export function isKeyword<T extends Keyword>(src: string, keyword: T): src is T {
  return src.toUpperCase() === keyword;
}
export function isOneOfKeywords<T extends Keyword>(src: string, keywords: T[]): src is T {
  return keywords.some(keyword => src.toUpperCase() === keyword);
}

