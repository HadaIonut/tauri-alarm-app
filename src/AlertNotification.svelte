<script>
  import {fly} from 'svelte/transition';
  import {alerts} from "./store/alerts.js";

  export let key;

  let notificationData;
  alerts.subscribe((value) => {
    notificationData = value[key];
  })
</script>

{#if notificationData}
  <div class="notification" transition:fly="{{ y: 100, duration: 2000 }}">
    {notificationData.message}
  </div>
{/if}

<style lang="scss">
  .notification {
    position: absolute;
    bottom: 10px;
    border: 1px solid transparent;
    border-radius: 0.25rem;
    width: 90vw;
    padding-top: 10px;
    padding-bottom: 10px;
    padding-left: 5px;
  }
</style>