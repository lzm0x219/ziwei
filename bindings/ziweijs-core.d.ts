export type Gender = "Yin" | "Yang";

export interface ZiweiBirth {
  readonly gender: Gender;
  readonly year: number;
  readonly month: number;
  readonly day: number;
  readonly hour: number;
}

export declare class ZiweiInputError extends Error {
  private constructor();
  readonly code: "INVALID_INPUT";
}

export declare class Ziwei {
  private constructor();
  static fromBirth(birth: ZiweiBirth): Ziwei;
}
