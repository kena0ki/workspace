
export const global = Function('return this')();

export const logger = global.dddlEnableLog ? console : {
  log(){},
  trace(){},
};

