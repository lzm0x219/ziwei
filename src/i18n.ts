import ZH_CN from "./locales/zh-CN";
import ZH_HANT from "./locales/zh-Hant";
import { createI18n } from "./tools/i18n";

const i18n = createI18n({
  lang: "zh-CN",
  resources: {
    "zh-CN": ZH_CN,
    "zh-Hant": ZH_HANT as unknown as typeof ZH_CN,
  },
});

export default i18n;
