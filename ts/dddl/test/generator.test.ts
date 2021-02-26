globalThis.dddlEnableLog=true;
import { tryParseAndGenerate } from '../src/generator';
import { logger } from '../src/util';
import { promises as fs } from 'fs';
import * as path from 'path';

describe('parse', () => {
  test(`001. parse and generate`, async () => {
    const file = path.join(__dirname, `sql/generator_001.sql`);
    const sql = await fs.readFile(file, 'utf8');
    console.time();
    let rows;
    try {
      rows = await tryParseAndGenerate(sql);
    } catch(err) {
      console.log(err);
    }
    console.timeEnd();
    logger.log(rows);
    expect(rows).toMatchSnapshot('parse and generate');
  });
});

