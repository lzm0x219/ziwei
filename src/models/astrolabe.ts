import type { Astrolabe, AstrolabeProps } from "./typing";

export function createAstrolabe(props: AstrolabeProps): Astrolabe {
  return {
    ...props,
  };
}
