// globalThis.dddlEnableLog=true;
import { parse } from '../src/parser';
import { logger } from '../src/util';
import { promises as fs } from 'fs';
import * as path from 'path';

describe('parse', () => {
  test(`001. parse create statement`, async () => {
    const file = path.join(__dirname, `sql/parser_001.sql`);
    const sql = await fs.readFile(file, 'utf8');
    console.time();
    const stmts = parse(sql);
    console.timeEnd();
    logger.log(stmts);
    expect(stmts).toMatchSnapshot('Create table');
  });
  test(`002. parse create statement`, async () => {
    const file = path.join(__dirname, `sql/parser_002.sql`);
    const sql = await fs.readFile(file, 'utf8');
    const stmts = parse(sql);
    logger.log(stmts);
    expect(stmts).toMatchSnapshot('Create table');
  });
  test(`003. parse create statement`, async () => {
    const file = path.join(__dirname, `sql/parser_003.sql`);
    const sql = await fs.readFile(file, 'utf8');
    const stmts = parse(sql);
    logger.log(stmts);
    expect(stmts).toMatchSnapshot('Create table');
  });
});

