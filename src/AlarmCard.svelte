<script>
  import {onDestroy} from "svelte";
  import {invoke} from '@tauri-apps/api/tauri';

  export let alarm;
  let alarmIn = ''

  let timeElapsedFormatter = (date) => {
    let dayOfMonth = date.getDate();
    let month = date.getMonth() + 1;
    let year = date.getFullYear();
    let hour = date.getHours();
    let minutes = date.getMinutes();
    let diffMs = date - new Date();
    let diffSec = Math.round(diffMs / 1000)
    let diffMin = diffSec / 60;
    let diffHour = diffMin / 60;

    year = year.toString().slice(-2);
    month = month < 10 ? '0' + month : month;
    dayOfMonth = dayOfMonth < 10 ? '0' + dayOfMonth : dayOfMonth;
    hour = hour < 10 ? '0' + hour : hour;
    minutes = minutes < 10 ? '0' + minutes : minutes;

    if (diffSec < 1) {
      return 'right now';
    } else if (diffMin < 1) {
      return `in ${Math.floor(diffSec)} sec`
    } else if (diffHour < 1) {
      return `in ${Math.floor(diffMin)} min`
    } else if (diffHour < 5) {
      return `in ${Math.floor(diffHour)} h`
    } else {
      return `${dayOfMonth}.${month}.${year} ${hour}:${minutes}`
    }
  }

  let updateTimer = 1000;
  let timeoutRef;

  let updateIntervalTimer = () => {
    let diffMs = new Date(alarm.execute_time_stamp) - new Date()
    let diffSec = Math.round(diffMs / 1000)
    let diffMin = diffSec / 60

    if (alarmIn.includes("h")) updateTimer = 600000
    else if (alarmIn.includes("min") && diffMin > 30) updateTimer = 600000
    else if (alarmIn.includes("min") && diffMin < 30 && diffMin > 10) updateTimer = 300000
    else if (alarmIn.includes("min") && diffMin > 1) updateTimer = 60000
    else if (alarmIn.includes("sec")) updateTimer = 1000
  }

  let alarmScheduler = () => {
    alarmIn = timeElapsedFormatter(new Date(alarm.execute_time_stamp))
    updateIntervalTimer()
    console.log("updateTimer: ", updateTimer)
    timeoutRef = setTimeout(alarmScheduler, updateTimer)
  }

  alarmScheduler()

  onDestroy(() => {
    clearTimeout(timeoutRef)
  })
</script>

<div class="with-border">
  <div class="message">
    {alarm.message}
  </div>
  <div class="time-stamp">
    {alarmIn}
  </div>
  <div on:click={() => invoke('remove_alarm_by_id', {id: alarm.id})}>X</div>
</div>

<style lang="scss">
  .message {
    text-align: left;
    font-size: 1rem;
    font-weight: 500;
    color: black;
  }

  .time-stamp {
    text-align: right;
    font-size: 0.7rem;
  }

  .with-border {
    box-shadow: 0 3px 1px -2px rgba(0, 0, 0, 0.2), 0 2px 2px 0 rgba(0, 0, 0, 0.14), 0 1px 5px 0 rgba(0, 0, 0, 0.12);
    padding: 8px;
    margin-bottom: 16px;
  }
</style>