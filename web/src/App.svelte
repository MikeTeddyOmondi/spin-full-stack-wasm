<script>
  import { onMount } from "svelte";
  import { todoStore } from "./stores/store";
  import { getTodos } from "./lib/todo";

  let input = {
    title: "",
  };
  let newTodo;
  let allTodos;
  let isChecked;
  // let pending = "pending";

  onMount(async () => {
    await getTodos();
  });

  async function handleSubmit() {
    if (input.title === "") return;
    console.log("Submit...");
    // newTodo = await invoke("add_todo", { input });
    // console.log(newTodo);
    // todoStore.update((currentData) => [...currentData, newTodo]);
    input.title = "";
  }

  // async function getTodos() {
  //   console.log("Get all todos...");
  //   // allTodos = await invoke("show_all");
  //   // console.log(allTodos);
  //   // $todoStore = [...allTodos];
  // }

  async function toggleComplete(e) {
    console.log("Complete: ", e.target.id);
    let id = e.target.id;
    // let result = await invoke("complete_todo", { id });
    // console.log(`${result} row(s) completed`);
    // if (result) {
    //   todoStore.update((currentTodos) => {
    //     return currentTodos.map((todo) => {
    //       if (todo.id === id) {
    //         return { ...todo, completed: true };
    //       }
    //       return todo;
    //     });
    //   });
    //   console.log($todoStore);
    // }
  }

  async function toggleParentComplete(e) {
    console.log("Complete: ", e.target.parentElement.id);
    let id = e.target.parentElement.id;
    // let result = await invoke("complete_todo", { id });
    // console.log(`${result} row(s) completed`);
    // if (result) {
    //   todoStore.update((currentTodos) => {
    //     return currentTodos.map((todo) => {
    //       if (todo.id === id) {
    //         return { ...todo, completed: true };
    //       }
    //       return todo;
    //     });
    //   });
    //   console.log($todoStore);
    // }
  }

  async function handleDelete(e) {
    console.log("Delete: ", e.target.parentElement.id);
    let id = e.target.parentElement.id;
    // let result = await invoke("delete_todo", { id });
    // console.log(`${result} row(s) deleted`);
    // if (result) {
    //   $todoStore = $todoStore.filter((todo) => todo.id != id);
    // }
  }

  async function handleClearAll() {
    console.log("Delete all...");
    // let result = await invoke("delete_all");
    // console.log(`${result} row(s) deleted`);
    // if (result) {
    //   $todoStore = [];
    // }
  }
</script>

<div class="container">
  <div class="input-field">
    <form on:submit|preventDefault={handleSubmit}>
      <input
        name="todo"
        placeholder="Enter your new todo"
        bind:value={input.title}
      />
      <i class="uil uil-notes note-icon" />
    </form>
  </div>

  <ul class="todoLists">
    {#each $todoStore as todo}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <li id={todo.id} class="list {todo.completed ? '' : 'pending'}" on:click={toggleComplete}>
        <!-- <input type="checkbox" bind:checked={isChecked} /> -->
        <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
        <p class="task {todo.completed ? 'task-complete' : ''} " on:click={toggleParentComplete}>
          {#if todo.completed}
            <s>{todo.title}</s>
          {:else}
            {todo.title}
          {/if}
        </p>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        {#if todo.completed}
          <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
          <img src="/trash_24px.png" alt="trash-icon" on:click={handleDelete} />
        {/if}
      </li>
    {/each}
  </ul>

  <div class="pending-tasks">
    <span>You have <span class="pending-num"> {$todoStore.length > 1 ? $todoStore.length : "no" } </span> task{#if $todoStore.length > 1}s{/if} pending.</span>
    {#if $todoStore.length} 
     <button class="clear-button" on:click={handleClearAll}>Clear All</button>
    {/if} 
  </div>
</div>
