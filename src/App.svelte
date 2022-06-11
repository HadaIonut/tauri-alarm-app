<script>
  import {invoke} from '@tauri-apps/api/tauri';
  import SveltyPicker from 'svelty-picker'
  import {
    Button,
    Card,
    CardActions,
    CardText,
    CardTitle,
    Col,
    Input,
    MaterialApp,
    Row,
    TextField
  } from "svelte-materialify";
  import {addHours, addMinutes, formatDate} from "./utils/utils.js";
  let myDate = formatDate(new Date(), 'y-M-d h:m');
  let alarmMessage = '';
  let alarmIn = '';

  const handleClick = async () => {
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
</script>

<main>
  <MaterialApp style="height: 100%">
    <Row style="height: 100%">
      <Col>
        <Card>
          <CardTitle>
            List of alarms go here
          </CardTitle>
          <CardText>
            caca
          </CardText>
        </Card>
      </Col>

      <Col class="add-alarm-col">
        <Card>
          <CardTitle style="justify-content: center">
            <span>Add an alert here:</span>
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
            <Button outlined on:click={handleClick}>Add Alarm</Button>
          </CardActions>
        </Card>

      </Col>
    </Row>
  </MaterialApp>

</main>

<style lang="scss">
  @import 'svelte-materialify/src/styles/tools/colors';

  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
    Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
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

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
    height: 100vh;
  }

</style>
