export interface GlobalConfigs {
  /**
   * 年分界点参数，默认为正月初一分界。
   *
   * normal：正月初一分界
   * spring：立春分界
   */
  _yearDivision: "normal" | "spring";

  /**
   * 月分界点参数，默认为月中分界。
   *
   * normal：正月中分界
   * last：视为上月
   * next：视为下月
   */
  _monthDivision: "normal" | "last" | "next";

  /**
   * 日分界点参数，默认为子时为次日。
   *
   * normal：子时为次日
   * current：当前子时为当日
   */
  _dayDivision: "normal" | "current";
}

export const GlobalConfigs: GlobalConfigs = {
  _yearDivision: "normal",
  _monthDivision: "normal",
  _dayDivision: "normal",
};

export function getGlobalConfigs(): GlobalConfigs {
  return {
    ...GlobalConfigs,
  };
}

export function setGlobalConfigs(config: Partial<GlobalConfigs>) {
  const _validDivision = {
    _yearDivision: ["normal", "spring"],
    _monthDivision: ["normal", "last", "next"],
    _dayDivision: ["normal", "current"],
  };
  const vaildConfig = Object.keys(config).reduce<Record<string, string>>((result, key) => {
    const _key = key as keyof GlobalConfigs;
    if (config[_key] && _validDivision[_key].includes(config[_key])) {
      result[_key] = config[_key];
    } else {
      result[_key] = GlobalConfigs[_key];
    }
    return result;
  }, {});

  Object.assign(GlobalConfigs, vaildConfig);
}
