<script>
  import { useNavigate, useLocation } from "svelte-navigator";
  import { TextField, Input, Button, Icon } from "svelte-materialify/src";
  import { user } from "../store";
  import { fetchWithTimeout } from "../utils/fetchWithTimeout";

  const navigate = useNavigate();
  const location = useLocation();

  let isShowPassword = false;

  let username;
  let password;

  const handleSumbit = async () => {
    fetchWithTimeout("api/auth/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username: username, password: password }),
    })
      .then((req) => req.json())
      .then((data) => {
        // user.set({ id: data.id, username: data.username });
        navigate("/login", { replace: true });
      })
      .catch(() => {
        // user.set(null);
        navigate("/register", {
          state: { from: $location.pathname },
          replace: true,
        });
      });
  };
</script>

<form action="" on:submit|preventDefault={handleSumbit}>
  <TextField bind:value={username} name="username" type="text" filled
    >Username</TextField
  >
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
  <Button class="primary-color" type="submit">Register</Button>
</form>
