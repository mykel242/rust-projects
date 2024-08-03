const { invoke } = window.__TAURI__.tauri;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

async function greet(name) {
  let foo = await invoke("greet", {
    name: name,
  });

  return foo;
}

window.addEventListener("DOMContentLoaded", () => {
  greet("velociraptors").then((greeting) => {
    console.log(greeting);
  });
});
