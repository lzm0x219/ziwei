import { expect, test } from "vitest";
import { $index, relativeIndex, oppositeIndex } from "../../src/utils/math.ts";

test("$index()", () => {
  expect($index(0)).toBe(0);
  expect($index(11)).toBe(11);
  expect($index(12)).toBe(0);
});

test("relativeIndex()", () => {
  expect(relativeIndex(0)).toBe(0);
  expect(relativeIndex(1)).toBe(11);
  expect(relativeIndex(2)).toBe(10);
});

test("oppositeIndex()", () => {
  expect(oppositeIndex(0)).toBe(6);
  expect(oppositeIndex(1)).toBe(7);
  expect(oppositeIndex(2)).toBe(8);
});
