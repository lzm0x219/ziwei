import { calculateHoroscope } from "../core/algorithms";
import type { Astrolabe, AstrolabeProps } from "./typing";

export function createAstrolabe(props: AstrolabeProps): Astrolabe {
  return {
    ...props,
    getHoroscope(index) {
      return calculateHoroscope(
        props.palaces,
        props.birthYearBranchKey,
        props.lunisolarYear,
        index,
      );
    },
  };
}
