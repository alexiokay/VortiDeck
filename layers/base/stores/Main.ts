import { useStorage } from "@vueuse/core";
import { useUserStore } from "./User";

import { Timeslot } from "@/types/timeslot";
export const useMainStore = defineStore("mainStore", {
  state: () => {
    return {
      // all these properties will have their type inferred automatically
      initialized: useStorage("initialized", false),
      isMobileNavbarOpen: useStorage("isMobileNavbarOpen", false),
      isLocaleSet: useStorage("isLocaleSet", false),
      isOverlaying: useStorage("isOverlaying", false),
      active_menu: useStorage("active_menu", "pending"),

      dates: useStorage("dates", [] as any[]),
      reservations: [] as any[],
    };
  },
  getters: {
    getActiveMenu(state) {
      return state.active_menu;
    },

    getReservations(state) {
      return state.reservations;
    },
    getNewReservations(state) {
      return state.reservations.filter((reservation) => {
        return reservation.status === "New";
      });
    },
    getReservation: (state) => (id: number) =>
      state.reservations.find((reservation) => reservation.id === id),

    getIsLocaleSet(state) {
      return state.isLocaleSet;
    },
    getIsMobileNavbarOpen(state) {
      return state.isMobileNavbarOpen;
    },

    isInitiated(state) {
      return state.initialized;
    },
    getDates(state) {
      return state.dates;
    },
    getDatesByDate: (state) => {
      return (date: string) => {
        console.log(state.dates);
        return state.dates.find((_date) => _date.date === date);
      };
    },

    getTimeslotsByDate: async (state) => {
      return (date: string) => {
        console.log("getting timeslots by date: " + date);
        const new_date = new Date(date).setHours(0, 0, 0, 0);

        return state.dates.find((_date) => {
          const date = new Date(_date.date).setHours(0, 0, 0, 0);

          return date === new_date;
        })?.timeslots;
      };
    },
  },
  actions: {
    initialize() {
      // this is called when the store is initialized
      this.initialized = true;
    },
    setActiveMenu(menu: string) {
      this.active_menu = menu;
    },
    hideMobileNavbar() {
      this.isMobileNavbarOpen = false;
    },
    showMobileNavbar() {
      this.isMobileNavbarOpen = true;
    },

    setLocaleSet(value: boolean = true) {
      this.isLocaleSet = value;
    },
    setOverlaying(bool: boolean) {
      this.isOverlaying = bool;
    },
    setTimeslot(dateId: number, timeslotId: number) {
      // find specified date and timeslot
      const dateIndex = this.dates.findIndex((date) => date.id === dateId);

      const timeslotIndex = this.dates[dateIndex].timeslots.findIndex(
        (timeslot: Timeslot) => timeslot.id === timeslotId,
      );
      const timeslot = this.dates[dateIndex].timeslots[timeslotIndex];

      // search for timeslot in date and sets ...
      if (timeslot) {
        timeslot.amount_of_trucks = timeslot.amount_of_trucks + 1;
        console.log(timeslot.amount_of_trucks);
      }

      // TODO! call composable or function to save to database
    },
    setDates(dates: any) {
      this.dates = dates;
      console.log(this.dates);
    },
    setReservations(reservations: any) {
      this.reservations = reservations;
    },
    addReservation(reservation: any) {
      console.log("adding reservation ");
      this.reservations.push(reservation);
    },
    removeReservation(reservationId: any) {
      try {
        const index = this.reservations.findIndex(
          (reservation) => reservation.id === reservationId,
        );
        this.reservations.splice(index, 1);
        console.log("removed reservation ");
      } catch (error) {
        console.log(error);
        console.log("couldn't remove reservation ");
      }
    },
    updateReservation(reservation: any) {
      console.log("updating reservation ");
      try {
        const index = this.reservations.findIndex(
          (reservation) => reservation.id === reservation.id,
        );
        this.reservations[index] = reservation;
        console.log(this.reservations[index]);
        console.log("updated reservation ");
      } catch (error) {
        console.log(error);
        console.log("couldn't update reservation ");
      }
    },
    openDay(id: number) {
      const index = this.dates.findIndex((date) => date.id === id);
      this.dates[index].is_open = true;
    },
    closeDay(id: number) {
      const index = this.dates.findIndex((date) => date.id === id);
      this.dates[index].is_open = false;
    },
  },
  persist: {
    storage: piniaPluginPersistedstate.sessionStorage,
  },
  // other options...
});
