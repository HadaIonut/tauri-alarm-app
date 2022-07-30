import {writable} from "svelte/store";

function createAlarmsStore() {
  const {subscribe, set, update} = writable([]);

  return {
    subscribe,
    set,
    update,
    append: (alarm) => update(current => [...current, alarm]),
    sort: (alarms) =>  set([...alarms].sort((a,b) => new Date(a.execute_time_stamp) - new Date(b.execute_time_stamp))),
    remove: (alarmId) => update(current => current.filter((test) => test.id !== alarmId))
  }
}

export const alarms = createAlarmsStore();