
export const global = Function('return this')();

export const logger = global.dddlEnableLog || process.env.DDDL_ENABLE_LOG ? console : {
  log(){},
  trace(){},
};

export function wrapError<T extends Error>(wrapper: T, cause: Error): T {
  wrapper.message = wrapper.message + '\n' + cause.message;
  if (wrapper.stack) {
    wrapper.stack = wrapper.stack.split('\n').slice(0,2).join('\n') + '\n' + cause.stack;
  }
  return wrapper;
}
