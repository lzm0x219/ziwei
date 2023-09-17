export type Mod = string | { [key: string]: unknown };
export type Mods = Mod | Mod[];

/**
 * bem helper
 * b() // 'button'
 * b('text') // 'button__text'
 * b({ disabled }) // 'button button--disabled'
 * b('text', { disabled }) // 'button__text button__text--disabled'
 * b(['disabled', 'primary']) // 'button button--disabled button--primary'
 */
export function createBEM(name: string) {
  const genBem = (name: string, mods?: Mods): string => {
    if (!mods) {
      return "";
    }

    if (typeof mods === "string") {
      return ` ${name}--${mods}`;
    }

    if (Array.isArray(mods)) {
      return mods.reduce<string>((ret, item) => ret + genBem(name, item), "");
    }

    return Object.keys(mods).reduce(
      (ret, key) => ret + genBem(name, mods[key] ? key : ""),
      "",
    );
  };

  return function (el?: Mods, mods?: Mods) {
    if (typeof el !== "string") {
      mods = el;
      el = "";
    }

    const blockOrElement = el ? `${name}__${el}` : name;
    return `${blockOrElement}${genBem(blockOrElement, mods)}`;
  };
}

export type BEM = ReturnType<typeof createBEM>;
