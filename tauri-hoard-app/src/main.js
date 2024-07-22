const { invoke } = window.__TAURI__.tauri;

// let greetInputEl;
// let greetMsgEl;
let btnNewAccount;
let divNewAccountFormContainer;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // greetMsgEl.textContent = await invoke("greet", { 
  //     name: greetInputEl.value 
  // });
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

  // get_accounts_stub();
  get_accounts_rust();
  // greetInputEl = document.querySelector("#greet-input");
  // greetMsgEl = document.querySelector("#greet-msg");
  // document.querySelector("#greet-form").addEventListener("submit", (e) => {
  //   e.preventDefault();
  //   greet();
  // });

  btnNewAccount = document.querySelector("#btn-new-account");
  divNewAccountFormContainer = document.querySelector("#new-account-form-container");

  btnNewAccount.addEventListener("click", (e) =>{
    e.preventDefault();
    console.log("New Account Button Clicked");
    // divNewAccountFormContainer.removeAttribute("hidden");
    // btnNewAccount.setAttribute("disabled", "");
    // btnNewAccount.classList.remove("uk-button-primary");
    // btnNewAccount.classList.add("uk-button-disabled");

  });
  //document.querySelector

});
