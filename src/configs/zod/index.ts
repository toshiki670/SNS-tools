import { z } from "zod";
import { makeZodI18nMap } from "zod-i18n-map";

import translationEnUS from "zod-i18n-map/locales/en/zod.json";
import translationJaJP from "zod-i18n-map/locales/ja/zod.json";

import i18n from "@/configs/i18n";

i18n.addResourceBundle("en-US", "zod", translationEnUS);
i18n.addResourceBundle("ja-JP", "zod", translationJaJP);

z.setErrorMap(makeZodI18nMap({ ns: "zod" }));

export default z;
