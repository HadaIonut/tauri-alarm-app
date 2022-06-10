<script>
  import {invoke} from '@tauri-apps/api/tauri';
  import SveltyPicker from 'svelty-picker'
  import {Col, MaterialApp, Row, TextField} from "svelte-materialify";

  let input = '8';
  let result = '';
  let myDate = '2021-11-11';

  const handleClick = async () => {
    result = await invoke('generate_password', {
      length: +input,
    });
  };
</script>

<main>
  <button on:click={handleClick}>{result}</button>

  <div>
    <SveltyPicker inputClasses="form-control" format="yyyy-mm-dd hh:ii" bind:value={myDate}></SveltyPicker>
    <div class="text-3xl font-bold underline">caca</div>
  </div>


  <MaterialApp>
    <Row>
      <Col>
        <TextField>Regular</TextField>
        <TextField filled>Filled</TextField>
        <TextField outlined>Outlined</TextField>
        <TextField solo placeholder="Solo" />
      </Col>
      <Col>
        <TextField placeholder="Placeholder">Regular</TextField>
        <TextField placeholder="Placeholder" filled>Filled</TextField>
        <TextField placeholder="Placeholder" outlined>Outlined</TextField>
        <TextField placeholder="Placeholder" solo />
      </Col>
    </Row>
  </MaterialApp>

</main>

<style lang="scss">
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
    Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  .a {
    background: red;
    & > .b {
      background: blue;
    }
  }

  :global(.form-control) {
    background: pink;
  }

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }

</style>
