export function isNumber(n?: number): n is number {
  return n !== undefined && n !== Infinity && !isNaN(n);
}
