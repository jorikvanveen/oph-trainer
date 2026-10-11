<script lang="ts">
  import { onMount } from 'svelte';

  let { title, description, confirm, cancel, onconfirm, oncancel }: {
    title: string;
    description: string;
    confirm: string;
    cancel: string;
    onconfirm: () => void;
    oncancel: () => void;
  } = $props();
  let dialog: HTMLDialogElement;

  onMount(() => dialog.showModal());

  function close(action: () => void) {
    dialog.close();
    action();
  }
</script>

<dialog bind:this={dialog} role="alertdialog" aria-labelledby="confirmation-title"
  aria-describedby="confirmation-description" oncancel={(event) => {
    event.preventDefault();
    close(oncancel);
  }}>
  <h2 id="confirmation-title">{title}</h2>
  <p id="confirmation-description">{description}</p>
  <div class="dialog-actions">
    <button class="secondary" onclick={() => close(oncancel)}>{cancel}</button>
    <button onclick={() => close(onconfirm)}>{confirm}</button>
  </div>
</dialog>

<style>
  button {
    border: 1px solid transparent;
    border-radius: 12px;
    background: #335c49;
    color: white;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    min-height: 36px;
    padding: 8px 12px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
  }

  button:hover {
    background: #284a3a;
  }

  button.secondary {
    background: #fff;
    color: #335c49;
    border-color: #d7dcd1;
  }

  button.secondary:hover {
    background: #edf1e9;
  }

  dialog {
    max-width: 512px;
    width: calc(100% - 32px);
    border: 1px solid #e3e6dd;
    border-radius: 14px;
    padding: 24px;
    box-shadow: 0 10px 25px #242a2720;
    color: #242a27;
    background: white;
  }

  dialog::backdrop {
    background: #00000080;
  }

  dialog h2 {
    font-size: 20px;
    font-weight: 600;
    line-height: 1.3;
    margin: 0 0 12px;
  }

  dialog p {
    font-size: 13px;
    line-height: 1.8;
    color: #6c716a;
  }

  @media (max-width: 520px) {
    .dialog-actions { flex-direction: column-reverse; }
  }

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 24px;
  }
</style>
