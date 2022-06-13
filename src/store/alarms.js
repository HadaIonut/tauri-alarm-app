import {writable} from "svelte/store";

function createAlarmsStore() {
  const {subscribe, set, update} = writable([]);

  return {
    subscribe,
    set,
    update,
    append: (alarm) => update(current => [...current, alarm]),
    remove: (alarmId) => update(current => current.filter((test) => test.id !== alarmId))
  }
}

export const alarms = createAlarmsStore();