<script>
  export let message;
  let direction = 'left';
  let backgroundColor = 'blue-grey';

  if (!message.username) {
    direction = 'right';
    backgroundColor = 'blue';
  }
</script>

{#if message.message}
  <div class="d-flex flex-column mb-1">
    <span
      class="p{direction === 'right'
        ? 'r'
        : 'l'}-2 rounded-pill {backgroundColor} lighten-e pt-1 pb-1 pl-2 pr-2 float-{direction} text-body-1"
       style="width: min-content;"
    >
      {message.message}
    </span>
    {#if message.username}
      <span class="text-body-2 p{direction === 'right' ? 'r' : 'l'}-1">
        {message.username}
      </span>
    {/if}
  </div>
{/if}
