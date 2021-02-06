globalThis.dddlEnableLog = true;
import { tokenize } from '../src/tokenizer';

const tokenSet = tokenize(`
  create table "ITEM" (
    id char(10),
    price decimal(10,3), /* yen */
    -- qty int,
    'hoge''fuga'
  );
`);
console.log(tokenSet);
console.log(tokenSet.joinValues(''));

// console.log(1);
// test('tokenize', () => {
//   console.log(2);
//   const tokenSet = tokenize(`
//     create table "ITEM" (
//       id char(10)
//       price decimal(10,3)
//     );
//   `);
//   console.log(tokenSet);
//   console.log(tokenSet.toString());
// });

