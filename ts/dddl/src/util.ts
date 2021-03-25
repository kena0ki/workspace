// import * as util from 'util';

export const global = Function('return this')();

export const logger = global.dddlEnableLog
  // Rollup.js unable to parse the following line.
  // || (global.process && global.process.env.DDDL_ENABLE_LOG)
  ? console : {
  log(){},
  trace(){},
  time(){},
  timeEnd(){},
};

// export function wrapError<T extends Error>(wrapper: T, cause: Error): T {
//   wrapper.message = wrapper.message + '\n' + cause.message;
//   if (wrapper.stack) {
//     wrapper.stack = wrapper.stack.split('\n').slice(0,2).join('\n') + '\n' + cause.stack;
//   }
//   return wrapper;
// }

const surrogatePairs = /[\uD800-\uDBFF][\uDC00-\uDFFF]/g;
export const length = (str: string): number => str.replace(surrogatePairs,' ').length;

// dirty hack for calculation of floating point number.
// TODO overflow
export const add = (left:number, right:number, scale:number): number => {
  if (scale <= 0) return (+left)+(+right);
  const digits=10**scale;
  return ((left*digits)+(right*digits))/digits;
};
export const subtract = (left:number, right:number, scale:number): number => {
  if (scale <= 0) return left-right;
  const digits=10**scale;
  return ((left*digits)-(right*digits))/digits;
};

export * from './util';
export * as util from './util';

