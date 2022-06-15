<script>
  import {alerts} from "./store/alerts.js";
  import AlertNotification from "./AlertNotification.svelte";

  let localAlerts;
  alerts.subscribe(val => {
    localAlerts = val;
  })
</script>

<div class="alerts-container">
  {#each Object.keys(localAlerts) as alert}
    <AlertNotification key={alert}/>
  {/each}
</div>

<style lang="scss">
  .alerts-container {
    width: 100vw;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>