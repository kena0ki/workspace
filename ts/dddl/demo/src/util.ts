type Option = {label:string, value:string};
export function getOptionByLabel(options: Option[], label: string): Option|undefined {
  return options.find(opt => opt.label === label);
}
export function getOptionByValue(options: Option[], value: string): Option|undefined {
  return options.find(opt => opt.value === value);
}

export * as util from './util';

