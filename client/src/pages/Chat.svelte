<script>
  import { onMount } from 'svelte';
  import Message from '../components/Message.svelte';
  import { TextField, Input, Button, List, Divider } from 'svelte-materialify/src';
  import store from '../store';

  let message = '';
  let direction = 'right';
  let messages = [];

  onMount(() => {
    store.subscribe((currentMessage) => {
      messages = [...messages, currentMessage];
    });
  });

  const onSendMessage = () => {
    message = message.trim();
    if (message.length > 0) {
      store.sendMessage(message.trim());
      messages = [...messages, { message: message }];
      message = '';
    }
  };
</script>

<form action="" on:submit|preventDefault={onSendMessage}>
  <div style="max-height: 90vh; overflow: scorll;" class="d-flex flex-column">
    {#each messages as message, i}
      <Message {message} {direction} />
      {#if i !== messages.length - 1}
        <!-- <Divider /> -->
      {/if}
    {/each}
  </div>
  <TextField bind:value={message} name="message" type="text" filled />
  <!-- <Button class="primary-color" type="submit">Send</Button> -->
</form>
