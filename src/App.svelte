<svelte:head>
  <script src="https://kit.fontawesome.com/dacbc752b2.js" crossorigin="anonymous"></script>
</svelte:head>

<script lang="ts">
  import Browse from "./panels/Browse.svelte";
  import Downloads from "./panels/Downloads.svelte";
  import Home from "./panels/Home.svelte";
  import Library from "./panels/Library.svelte";
  import Preferences from "./panels/Preferences.svelte";

  var currentPage: number;

  function SwitchPage(Option: number) {
    currentPage = Option
  }
</script>

<main class="container">
  <div class="sidenav">
    <button class="menu_button" on:click={() => SwitchPage(0)}>      
      Home
    </button>
    <button class="menu_button" on:click={() => SwitchPage(1)}>      
      Library
    </button>
    <button class="menu_button" on:click={() => SwitchPage(2)}>      
      Browse
    </button>
    <button class="menu_button" on:click={() => SwitchPage(3)}>      
      Downloads
    </button>
    <button class="menu_button" on:click={() => SwitchPage(4)}>      
      Preferences
    </button>
  </div>

  {#if currentPage == 0}
    <Home/>
  {:else if currentPage == 1}
    <Library/>
  {:else if currentPage == 2}
    <Browse/>
  {:else if currentPage == 3}
    <Downloads/>
  {:else if currentPage == 4}
    <Preferences/>
  {:else}
    <Home/>
  {/if}
  
</main>