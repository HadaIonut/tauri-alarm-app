<script>
  import {invoke} from '@tauri-apps/api/tauri';
  import SveltyPicker from 'svelty-picker'
  import {
    Alert,
    Button,
    Card,
    CardActions,
    CardText,
    CardTitle,
    Col,
    MaterialApp,
    Row,
    TextField
  } from "svelte-materialify";
  import {addHours, addMinutes, formatDate, readSaveFile, updateSaveFile} from "./utils/utils.js";
  import {listen} from "@tauri-apps/api/event";
  import {alarms} from "./store/alarms.js";
  import {onMount} from "svelte";
  import AlertManager from "./AlertManager.svelte";
  import {alerts} from "./store/alerts.js";
  import AlarmCard from "./AlarmCard.svelte";

  let myDate = formatDate(new Date(), 'y-M-d h:m');
  let alarmMessage = '';
  let alarmIn = '';

  const addAlarm = async () => {
    let alarmMatch = alarmIn.match(/^(\d+h)?(\d+m)?$/);
    let alarmInDate = new Date();
    if (alarmIn && alarmMatch) {
      if (alarmMatch[1]) {
        let hours = [...alarmMatch[1]];
        hours.pop();
        hours = hours.join('');

        alarmInDate = addHours(alarmInDate, Number(hours))
      }
      if (alarmMatch[2]) {
        let minute = [...alarmMatch[2]];
        minute.pop();
        minute = minute.join('');

        alarmInDate = addMinutes(alarmInDate, Number(minute))
      }

      alarmInDate = formatDate(alarmInDate, 'y-M-d h:m');
      return await invoke('create_new_alarm', {message: alarmMessage, timestamp: alarmInDate})
    }

    return await invoke('create_new_alarm', {message: alarmMessage, timestamp: myDate})
  };

  const initAndStartListeners = () => {
    invoke('start_schedule_thread');
    invoke('init_file_save');
    listen('alarm-added', async event => {
      alarms.append(event.payload.alarm);
      await updateSaveFile($alarms);
    });
    listen('alarm-removed', async event => {
      alarms.remove(event.payload.alarm.id);
      alerts.addAlert({message: event.payload.alarm.message});
      await updateSaveFile($alarms);
    });
  };

  initAndStartListeners();

  onMount(async () => {
    alarms.set(await readSaveFile());
  })
</script>

<main>
  <MaterialApp class="full-height">
    <Row>
      <Col>
        <Card class="alarm-card">
          <CardTitle>
            <span class="card-title">
              List of alarms go here
            </span>
          </CardTitle>
          <CardText class="alarm-container">
            {#each $alarms as alarm }
              <AlarmCard alarm="{alarm}"></AlarmCard>
            {/each}
          </CardText>
        </Card>
      </Col>

      <Col class="add-alarm-col">
        <Card>
          <CardTitle class="justify-center">
            <span class="card-title">Add an alert here:</span>
          </CardTitle>

          <CardText>
            <TextField outlined bind:value={alarmMessage}>Alarm message</TextField>
            <Row>
              <Col class="time-picker-col">
                <SveltyPicker inputClasses="form-control" format="yyyy-mm-dd hh:ii" bind:value={myDate}></SveltyPicker>
              </Col>

              <Col>
                <TextField outlined bind:value={alarmIn}>Set alarm in</TextField>
              </Col>
            </Row>
          </CardText>

          <CardActions>
            <Button outlined on:click={addAlarm}>Add Alarm</Button>
          </CardActions>
        </Card>

      </Col>
    </Row>
    <AlertManager/>
  </MaterialApp>
</main>

<style lang="scss">
  @import 'svelte-materialify/src/styles/tools/colors';

  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
    Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  :global(.alarm-container) {
    height: 65%;
    overflow: auto;
  }

  :global(.alarm-card) {
    height: 50vh;
    min-height: 311px;
  }

  :global(.card-title) {
    border-bottom: dashed 1px darkgray;
    padding-bottom: 10px;
    width: 100%;
  }

  :global(.form-control) {
    width: 100%;
    border: 1px solid material-color('grey', 'darken-2');
    padding: 0 1px 0 12px;
    border-radius: 4px;
    min-height: 56px;
    color: material-color('grey', 'darken-2');
  }

  :global(.add-alarm-col) {
    flex-grow: 3;

    & > :global(*) {
      margin-bottom: 5px;
    }
  }

  :global(.time-picker-col) {
    & > :global(*) {
      margin-bottom: 5px;
    }
  }

  :global(.full-height) {
    height: 100%;
  }

  .justify-center {
    justify-content: center
  }

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
    height: 100vh;
  }

</style>
