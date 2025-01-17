/**
 * 函数性能测量装饰器
 * 用于测量异步或同步函数的执行时间
 *
 * @param operation - 操作名称，用于在日志中标识当前测量的操作
 * @returns 返回一个高阶函数，该函数接收并包装目标函数
 * @template T - 目标函数的返回值类型
 *
 * @example
 * // 使用示例：
 * const measuredFunction = withMeasure('数据处理')(async () => {
 *   // 某些操作
 *   return result;
 * });
 */
export function withMeasure<T>(operation: string) {
  return async (fn: () => T | Promise<T>): Promise<T> => {
    const start = performance.now();
    const result = await Promise.resolve(fn());
    const end = performance.now();
    console.log(`${operation} 耗费 ${end - start}ms`);
    return result;
  };
}
