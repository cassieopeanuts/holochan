<!-- src/components/Navbar.svelte -->
<script>
  export let logo;
</script>

<nav class="navbar">
  <div class="container">
    <img src={logo} alt="HoloChan Logo" class="logo small" />
    <ul class="nav-links">
      <li><a href="#home">Home</a></li>
      <li><a href="#threads">Threads</a></li>
      <li><a href="#profile">Profile</a></li>
      <li><a href="#create">Create</a></li>
    </ul>
  </div>
</nav>

<style>
/* Styles from App.svelte's navbar */
</style>
