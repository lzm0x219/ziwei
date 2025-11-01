import { _stemStarTransformations } from "../constants";
import type { Palace, PalaceProps } from "../typings";

export function createPalace(props: PalaceProps): Palace {
  return {
    ...props,
    $starKeysByFlying() {
      return _stemStarTransformations[props.stemKey];
    },
  };
}
