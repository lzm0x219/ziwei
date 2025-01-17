/**
 * pipe 函数用于创建函数管道，将多个函数组合成一个新的函数
 *
 * @template T - 泛型参数，表示函数输入和输出的类型
 * @param {...Array<(arg: T) => T>} fns - 接收任意数量的函数作为参数，这些函数都接收类型 T 的参数并返回相同类型
 * @returns {(value: T) => T} - 返回一个新的函数，该函数接收一个初始值，并按顺序执行所有传入的函数
 *
 * @example
 * // 示例用法：
 * const addOne = (x: number) => x + 1;
 * const double = (x: number) => x * 2;
 * const addOneAndDouble = pipe(addOne, double);
 * console.log(addOneAndDouble(3)); // 输出: 8 (先加1得4，再乘2得8)
 *
 * @description
 * 函数实现了函数式编程中的管道（pipeline）模式：
 * 1. 接收多个函数作为参数
 * 2. 返回一个新函数，这个新函数接收一个初始值
 * 3. 使用 reduce 方法依次执行所有函数，前一个函数的输出作为后一个函数的输入
 * 4. 所有函数必须保持类型一致性，即输入和输出类型相同
 */
export function pipe<T>(...fns: Array<(arg: T) => T>) {
  return (value: T) => fns.reduce((acc, fn) => fn(acc), value);
}
