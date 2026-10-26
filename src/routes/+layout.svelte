<script>
  import "../app.css";
  let { children } = $props();
</script>

<a class="skip" href="#main">Skip to content</a>
<main id="main">{@render children()}</main>
