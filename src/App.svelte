<script lang="ts">
  
  function switch_panels(panel: number) {

    
    
    switch (panel) {
      case 0:
        console.log("Home");
        break;

      case 1:
        console.log("Library");
        break;

      case 2:
        console.log("Browse");
        break;

      case 3:
        console.log("Downloads");
        break;

      case 4:
        console.log("Preferences");
        break;
    }

    return 0;
  }

</script>

<main class="container">
  
  <div class="sidenav">
    <button id="home" class="menu_button" on:click={() => switch_panels(0)}>Home</button>
    <button id="library" class="menu_button" on:click={() => switch_panels(1)}>Library</button>
    <button id="browse" class="menu_button" on:click={() => switch_panels(2)}>Browse</button>
    <button id="downloads" class="menu_button" on:click={() => switch_panels(3)}>Downloads</button>
    <button id="preferences" class="menu_button" on:click={() => switch_panels(4)}>Preferences</button>
  </div>

</main>