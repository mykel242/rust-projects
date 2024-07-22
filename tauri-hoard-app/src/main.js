const { invoke } = window.__TAURI__.tauri;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

async function greet(name) {
  let foo = await invoke("greet", { 
      name: name 
  });

  return foo;
}

async function get_accounts_stub() {
  let data = await invoke("get_accounts_stub");
  console.log(data);
}

async function get_accounts_rust() {
  let data = await invoke("get_accounts_rust");
  console.log(data);
}

window.addEventListener("DOMContentLoaded", () => {

  greet("velociraptors").then((greeting) => {
    console.log(greeting);
  });
  
  get_accounts_rust();
 
});
