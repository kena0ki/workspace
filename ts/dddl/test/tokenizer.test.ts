import { tokenize } from '../src/tokenizer';
import { logger } from '../src/util';

class A extends Object {
  get hoge() { return 1;}
}
console.log('hogeeeee', (new A).hoge);

describe('tokenize', () => {
  test('tokenize create statement', () => {
    const tokenSet = tokenize(`
      create table "ITEM" (
        id char(10),
        price decimal(10,3), /* yen */
        -- qty int,
      );
    `);
    logger.log(tokenSet.tokens);
    expect(tokenSet).toMatchSnapshot('Create table');
  });
  test('tokenize select statement', () => {
    const start = Date.now();
    const tokenSet = tokenize(`
      \rSELECT\r\n
        X'hex string' || x'hex string2',
        N'national string',
        'character '' string',
        1+2-3/4*5%6
      from A@B.C D, _E E, #F
      where !(D.d = E.e) and D.d != E.e and D.d <> E.e and D.d < E.e and D.d > E.e;
    `);
    logger.log('time:', Date.now() - start);
    logger.log(tokenSet.toString());
    logger.log(tokenSet.joinValues(''));
    expect(tokenSet).toMatchSnapshot('query');
  });
  test('tokenize operators', () => {
    const tokenSet = tokenize(`
      \\[]&^{}~#
    `);
    logger.log(tokenSet);
    expect(tokenSet).toMatchSnapshot('query');
  });
});

