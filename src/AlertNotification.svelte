<script>
  import {fly} from 'svelte/transition';
  import {alerts} from "./store/alerts.js";
  import {Card, CardText, CardTitle} from "svelte-materialify";

  export let key;

  let notificationData;
  alerts.subscribe((value) => {
    notificationData = value[key];
  })
</script>

{#if notificationData}
  <div class="notification" transition:fly="{{ y: 100, duration: 2000 }}">
    <Card>
      <CardTitle class="justify-center">
        <span>{notificationData.message}</span>
      </CardTitle>
    </Card>
  </div>
{/if}

<style lang="scss">
  .notification {
    position: absolute;
    bottom: 10px;
    border: 1px solid transparent;
    border-radius: 0.25rem;
    width: 50vw;
    padding-top: 10px;
    padding-bottom: 10px;
    padding-left: 5px;
  }
</style>