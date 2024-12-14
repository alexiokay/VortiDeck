type store = any;

export const saveCookiesSettings = (store: any) => {
  store.acceptedCookies = {
    necessary: true,
    functional: functional.value,
    analytics: analytics.value,
    performance: performance.value,
    advertisement: advertisement.value,
    others: others.value,
  };

  store.isCookiesSet = true;
};

export const acceptAllCookies = (store: any) => {
  store.acceptedCookies = {
    necessary: true,
    functional: true,
    analytics: true,
    performance: true,
    advertisement: true,
    others: true,
  };
  store.isCookiesSet = true;
};

export const rejectAllCookies = (store: any) => {
  store.acceptedCookies = {
    necessary: true,
    functional: false,
    analytics: false,
    performance: false,
    advertisement: false,
    others: false,
  };
  store.isCookiesSet = true;
};
