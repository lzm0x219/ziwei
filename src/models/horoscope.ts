import type { Horoscope, HoroscopeProps } from "./typing";

export function createHoroscope(props: HoroscopeProps): Horoscope {
  return {
    ...props,
  };
}
