import { tokenize  } from '../src/tokenizer';
import { logger } from '../src/util';
import { promises as fs } from 'fs';
import * as path from 'path';

class A extends Object {
  get hoge() { return 1;}
}
console.log('hogeeeee', (new A).hoge);

describe('tokenize', () => {
  test(`001. tokenize create statement`, async () => {
    const file = path.join(__dirname, `sql/tokenizer_001.sql`);
    const sql = await fs.readFile(file, 'utf8');
    const tokenSet = tokenize(sql);
    logger.log(tokenSet.tokens);
    expect(tokenSet).toMatchSnapshot('Create table');
  });
  test(`002. tokenize select statement`, async () => {
    const start = Date.now();
    const file = path.join(__dirname, `sql/tokenizer_002.sql`);
    const sql = await fs.readFile(file, 'utf8');
    const tokenSet = tokenize(sql);
    logger.log('time:', Date.now() - start);
    logger.log(tokenSet.toString());
    logger.log(tokenSet.joinValues(''));
    expect(tokenSet).toMatchSnapshot('query');
  });
  test(`003. tokenize operators`, async () => {
    const tokenSet = tokenize(`\\[]&^{}~#`);
    logger.log(tokenSet);
    expect(tokenSet).toMatchSnapshot('operators');
  });
  test(`004. tokenize error`, async () => {
    try {
      tokenize(`select 'error`);
    } catch (err) {
      logger.log(err);
      expect(err).toMatchSnapshot('error');
    }
  });
  test(`005. tokenize error`, async () => {
    try {
      tokenize(`select * from "error`);
    } catch (err) {
      logger.log(err);
      expect(err).toMatchSnapshot('error');
    }
  });
  test(`006. tokenize error`, async () => {
    try {
      tokenize(`
    select /* from A`);
    } catch (err) {
      logger.log(err);
      expect(err).toMatchSnapshot('error');
    }
  });
});

