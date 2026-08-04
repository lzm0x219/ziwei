import initNative, { NativeZiwei } from "./generated/native.js";

const construct = Symbol("Ziwei.construct");
const nativeHandles = new WeakMap();
let initialized = false;

export class ZiweiInputError extends Error {
  constructor(message) {
    super(message);
    this.name = "ZiweiInputError";
    this.code = "INVALID_INPUT";
  }
}

export class Ziwei {
  constructor(token, native) {
    if (token !== construct) {
      throw new TypeError("Ziwei cannot be constructed directly; use Ziwei.fromBirth()");
    }

    nativeHandles.set(this, native);
    Object.freeze(this);
  }

  static fromBirth(birth) {
    if (!initialized) {
      throw new Error("@ziweijs/core-wasm must be initialized before Ziwei.fromBirth()");
    }

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

export default async function init(input) {
  await initNative(input === undefined ? undefined : { module_or_path: input });
  initialized = true;
}

function errorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}
