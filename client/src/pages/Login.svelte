<script>
  import { useNavigate, useLocation } from "svelte-navigator";
  import { TextField, Input, Button } from "svelte-materialify/src";
  import { user } from "../store";
  import { fetchWithTimeout } from "../utils/fetchWithTimeout";

  const navigate = useNavigate();
  const location = useLocation();

  let isShowPassword = false;

  let username;
  let password;

  const handleSumbit = async () => {
    fetchWithTimeout("api/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username: username, password: password }),
    })
      .then((req) => req.json())
      .then((data) => {
        user.set({ id: data.id, username: data.username });
        localStorage.setItem("id", data.id);
        localStorage.setItem("username", data.username);
        const from = ($location.state && $location.state.from) || "/";
        navigate(from, { replace: true });
      })
      .catch(() => {
        user.set(null);
        localStorage.setItem("id", null);
        localStorage.setItem("username", null);
        navigate("/login", {
          state: { from: $location.pathname },
          replace: true,
        });
      });
  };

  const handleSignOut = async () => {
    await fetch("api/auth/logout");
    localStorage.removeItem("id");
    localStorage.removeItem("username");
    user.set(null);
  };
</script>

{#if !$user}
  <form action="" on:submit|preventDefault={handleSumbit}>
    <TextField bind:value={username} name="username" type="text" filled>
      Username
    </TextField>
    <TextField
      bind:value={password}
      name="password"
      type={isShowPassword ? "text" : "password"}
      filled
    >
      Password
      <div
        slot="append"
        on:click={() => {
          isShowPassword = !isShowPassword;
        }}
      >
        <span class="material-icons primary-text">
          {isShowPassword ? "visibility_off" : "visibility"}
        </span>
      </div>
    </TextField>
    <Button class="primary-color" type="submit">LogIn</Button>
  </form>
{:else}
  <h1>{$user.username} has logged in!</h1>
  <Button class="primary-color" on:click={handleSignOut}>SignOut</Button>
{/if}
