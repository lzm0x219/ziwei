import type { Palace, PalaceProps } from "./typing";

export function createPalace(props: PalaceProps): Palace {
  return {
    ...props,
  };
}
