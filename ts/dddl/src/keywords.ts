import { types } from './data-types';

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
  ...types.DATA_TYPE_NAMES,
] as const;
export type Keyword = typeof KEYWORDS[number];

