import {writable} from "svelte/store";

const createAlertStore = () => {
  const {subscribe, set, update} = writable({});

  const remove = (key) => {
    update((prev) => {
      const {[key]: parentValue, ...noChild } = prev;
      return noChild;
    })
  }

  return {
    subscribe,
    update,
    clear: () => set({}),
    remove,
    addAlert: ({message, duration = 1000}) => {
      const getRandomId = () => '_' + Math.random().toString(36).substr(2, 9);

      const notificationKey = getRandomId();

      update((prev) => ({
        ...prev,
        [notificationKey]: {
          key: notificationKey,
          message,
        }
      }))

      setTimeout(() => remove(notificationKey), duration + 2000);
    }
  }
}

export const alerts = createAlertStore();