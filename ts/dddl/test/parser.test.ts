globalThis.dddlEnableLog=true;
import { parse } from '../src/parser';
import { logger } from '../src/util';

describe('parse', () => {
  test('parse create statement', () => {
    //const stmts = parse(`
    //  create table "ITEM" (
    const stmts = parse(`create table "ITEM" (
        id char(10),
        price decimal(10,3), /* yen */
        -- qty int,
      );
    `);
    logger.log(stmts);
    expect(stmts).toMatchSnapshot('Create table');
  });
});

