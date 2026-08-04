import init, { Ziwei, type ZiweiBirth } from "@ziweijs/core-wasm";

await init();

const birth: ZiweiBirth = {
  gender: "Yang",
  year: 1984,
  month: 2,
  day: 1,
  hour: 4,
};

const chart: Ziwei = Ziwei.fromBirth(birth);
void chart;

// @ts-expect-error Ziwei instances can only be created through fromBirth().
new Ziwei();
