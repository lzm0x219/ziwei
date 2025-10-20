import type { Star, StarProps } from "./typing";

export function createStar(props: StarProps): Star {
  return {
    ...props,
  };
}
