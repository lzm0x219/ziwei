import { _stemStarTransformations } from "../constants";
import type { Palace, PalaceProps } from "./typing";

export function createPalace(props: PalaceProps): Palace {
  return {
    ...props,
    $starKeysByFlying() {
      return _stemStarTransformations[props.stemKey];
    },
  };
}
