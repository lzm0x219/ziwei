import type { Horoscope, HoroscopeProps } from "../typings";

export function createHoroscope(props: HoroscopeProps): Horoscope {
  return {
    ...props,
  };
}
