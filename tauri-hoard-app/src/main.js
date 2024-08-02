const { invoke } = window.__TAURI__.tauri;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

async function greet(name) {
  let foo = await invoke("greet", {
    name: name,
  });

  return foo;
}

async function get_accounts() {
  let data = await invoke("get_accounts");
  console.log(data);
}

window.addEventListener("DOMContentLoaded", () => {
  greet("velociraptors").then((greeting) => {
    console.log(greeting);
  });

  get_accounts();
});
