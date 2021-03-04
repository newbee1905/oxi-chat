<script lang="ts">
  import { onMount } from "svelte";
  import { MaterialApp, Row } from "svelte-materialify";
  import { Router, Route, Link } from "svelte-navigator";
  import Login from "./pages/Login.svelte";
  import Register from "./pages/Register.svelte";
  import Chat from "./pages/Chat.svelte";
  import PrivateRoute from "./components/PrivateRoute.svelte";
  import { user } from "./store";
  import { fetchWithTimeout } from "./utils/fetchWithTimeout";

  onMount(async () => {
    try {
      const res = await fetchWithTimeout("/api/auth/login", {
        credentials: "include",
        method: "GET",
      });
      let userInfo = await res.json();
      user.set(userInfo);
    } catch (err) {
      user.set(null);
    }
  });
</script>

<MaterialApp>
  <Router>
    <nav>
      <Link to="/">Home</Link>
      <Link to="login">Login</Link>
      <Link to="register">Register</Link>
      <Link to="chat">Chat</Link>
    </nav>
    <Route path="/">
      <div class="d-flex flex-column align-center">
        <Row>
          {#if !$user}
            <h1 class="text-h1 red-text">You have not logged in!</h1>
          {:else}
            <h1 class="text-h1 blue-text">Hello {$user.username}!</h1>
          {/if}
        </Row>
      </div>
    </Route>

    <Route path="login">
      <Login />
    </Route>

    <Route path="register">
      <Register />
    </Route>

    <PrivateRoute path="chat">
      <Chat />
    </PrivateRoute>
  </Router>
</MaterialApp>
