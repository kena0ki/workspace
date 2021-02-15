globalThis.dddlEnableLog=true;
import { parse } from '../src/parser';
import { logger } from '../src/util';
import { promises as fs } from 'fs';
import * as path from 'path';

(async () => {
  const file = path.join(__dirname, `sql/parser_002.sql`);
  const sql = await fs.readFile(file, 'utf8');
  const stmts = parse(sql);
  logger.log(stmts);
})();

