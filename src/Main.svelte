<!-- Only touch this file if adding a new page -->
<!-- Or styling a Modal -->
<!-- Otherwise, ignore it -->

<svelte:head>
  <script src="https://kit.fontawesome.com/dacbc752b2.js" crossorigin="anonymous"></script>
</svelte:head>

<script lang="ts">
  import Browse from "./components/Browse.svelte";
  import Downloads from "./components/Downloads.svelte";
  import Home from "./components/Home.svelte";
  import Library from "./components/Library.svelte";
  import Preferences from "./components/Preferences.svelte";
  import { Modal } from "svelte-simple-modal";

  var currentPage: number;

  // TS Function
  // - Changes the currentPage variable to the number on args
  function SwitchPage(Option: number) {
    currentPage = Option
  }
</script>

<main class="container">
  <div class="sidenav">
    <img src="../src-tauri/icons/icon.png" width="100" class="branding" alt="branding"/>
    <button class="menu-button" on:click={() => SwitchPage(0)}>      
      <i class="fa-solid fa-house" /> Home
    </button>
    <button class="menu-button" on:click={() => SwitchPage(1)}>      
      <i class="fa-solid fa-gamepad" /> Library
    </button>
    <button class="menu-button" on:click={() => SwitchPage(2)}>      
      <i class="fa-solid fa-magnifying-glass" /> Browse
    </button>
    <button class="menu-button" on:click={() => SwitchPage(3)}>      
      <i class="fa-solid fa-download" /> Downloads
    </button>
    <button class="menu-button" on:click={() => SwitchPage(4)}>      
      <i class="fa-solid fa-gear" /> Preferences
    </button>
  </div>

  {#if currentPage == 0}
    <Home/>
  {:else if currentPage == 1}
    <Modal
      styleBg={{ backgroundColor: "rgba(0, 0, 0, 0.5)" }}
      styleWindow={{ 
        backgroundColor: "#000",
        border: "2px dashed #00ff66",
        borderRadius: "0px",
        float: "center"
      }}
      closeButton={false}
    >
      <Library/>
    </Modal>
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