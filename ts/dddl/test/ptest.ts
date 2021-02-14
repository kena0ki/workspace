globalThis.dddlEnableLog=true;
import { parse } from '../src/parser';
import { logger } from '../src/util';
const stmts = parse(`create table "ITEM" (
    id char(10),
    price decimal(10,3), /* yen */
    -- qty int,
  );
`);
logger.log(stmts);

