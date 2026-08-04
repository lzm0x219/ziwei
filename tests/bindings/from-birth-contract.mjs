import assert from "node:assert/strict";
import test from "node:test";

const validBirth = Object.freeze({
  gender: "Yang",
  year: 1984,
  month: 2,
  day: 1,
  hour: 4,
});

export function defineFromBirthContract({ Ziwei, ZiweiInputError }) {
  test("fromBirth returns an immutable Ziwei instance", () => {
    const chart = Ziwei.fromBirth(validBirth);

    assert.ok(chart instanceof Ziwei);
    assert.ok(Object.isFrozen(chart));
    assert.deepEqual(validBirth, {
      gender: "Yang",
      year: 1984,
      month: 2,
      day: 1,
      hour: 4,
    });
  });

  test("Ziwei cannot be constructed directly", () => {
    assert.throws(() => new Ziwei(), /use Ziwei\.fromBirth\(\)/);
  });

  test("fromBirth exposes the same typed input error", () => {
    const invalidBirths = [
      null,
      { ...validBirth, year: "1984" },
      { ...validBirth, gender: "Male" },
      { ...validBirth, year: Number.NaN },
      { ...validBirth, year: 2_147_483_648 },
      { ...validBirth, month: 12 },
      { ...validBirth, day: 0 },
      { ...validBirth, hour: 12 },
    ];

    for (const birth of invalidBirths) {
      assert.throws(
        () => Ziwei.fromBirth(birth),
        (error) => {
          assert.ok(error instanceof ZiweiInputError);
          assert.equal(error.name, "ZiweiInputError");
          assert.equal(error.code, "INVALID_INPUT");
          assert.ok(error.message.length > 0);
          return true;
        },
      );
    }
  });
}
