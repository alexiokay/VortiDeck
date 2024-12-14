import en from "../locales/en.yml";
import nl from "../locales/nl.yml";

export default defineI18nConfig(() => ({
  defaultSFCLang: "yml",
  globalSFCScope: "true",
  legacy: false,
  locale: "en",
  messages: {
    en,
    nl,
  },
}));
