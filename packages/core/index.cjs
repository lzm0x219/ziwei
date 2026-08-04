"use strict";

const { NativeZiwei } = require("./native.cjs");

const construct = Symbol("Ziwei.construct");
const nativeHandles = new WeakMap();

class ZiweiInputError extends Error {
  constructor(message) {
    super(message);
    this.name = "ZiweiInputError";
    this.code = "INVALID_INPUT";
  }
}

class Ziwei {
  constructor(token, native) {
    if (token !== construct) {
      throw new TypeError("Ziwei cannot be constructed directly; use Ziwei.fromBirth()");
    }

    nativeHandles.set(this, native);
    Object.freeze(this);
  }

  static fromBirth(birth) {
    try {
      validateBirthShape(birth);
      return new Ziwei(construct, NativeZiwei.fromBirth(birth));
    } catch (error) {
      if (error instanceof ZiweiInputError) {
        throw error;
      }

      throw new ZiweiInputError(errorMessage(error));
    }
  }
}

function validateBirthShape(birth) {
  if (
    birth === null ||
    typeof birth !== "object" ||
    typeof birth.gender !== "string" ||
    typeof birth.year !== "number" ||
    typeof birth.month !== "number" ||
    typeof birth.day !== "number" ||
    typeof birth.hour !== "number"
  ) {
    throw new ZiweiInputError(
      "ZiweiBirth must contain string gender and numeric year, month, day, hour fields",
    );
  }
}

function errorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}

module.exports = { Ziwei, ZiweiInputError };
