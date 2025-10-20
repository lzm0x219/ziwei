import { Language } from "./enums";
import ZH_CN from "./locales/zh-CN";
import ZH_HANT from "./locales/zh-Hant";
import { createI18n } from "./tools/i18n";

const i18n = createI18n({
  lang: Language.ZH_CN,
  resources: {
    [Language.ZH_CN]: ZH_CN,
    [Language.ZH_HANT]: ZH_HANT as unknown as typeof ZH_CN,
  },
});

export default i18n;
