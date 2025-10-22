import { describe, expect, test } from "@rstest/core";
import { Gender } from "../enums";
import { byLunisolar, bySolar } from "../ziwei";

describe("ziwei", () => {
  test("bySolar", () => {});
  test("byLunisolar", () => {
    console.log(
      byLunisolar({
        name: "xxx",
        date: "1998-1-23-1",
        gender: Gender.MALE,
      }).palaces[3],
    );
  });
});
