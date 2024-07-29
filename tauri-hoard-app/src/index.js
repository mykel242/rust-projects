const { invoke } = window.__TAURI__.tauri;

let btnNewAccount;
let btnNewAccountOk;
let btnNewAccountCancel;

window.addEventListener("DOMContentLoaded", () => {
  // Show the form when the + account button is clicked
  btnNewAccount = document.querySelector("#btn-new-account");
  btnNewAccount.addEventListener("click", (e) => {
    btnNewAccount.setAttribute("disabled", "");
    e.preventDefault();
    $("#new-account-form").fadeIn(400);
  });

  // Check the form, save the data, hide the form
  btnNewAccountOk = document.querySelector("#new-account-btn-ok");
  btnNewAccountOk.addEventListener("click", (e) => {
    //TODO: I should rewrite this - the validation shouldnt be returning the data
    let account = validateNewAccountForm();

    if (account) {
      // Send the data to the backend
      //let account = { name: "placeholder", balance: -6.66, type: 123 };
      console.log("Ship it!");
      save_new_account(account).then((result) => {
        console.log("save account result:", result);
      });

      // hide the form
      $("#new-account-form").fadeToggle(250, (d) => {
        //clear the form
        resetAccountForm();
        //re-enable the + account button
        btnNewAccount.removeAttribute("disabled");
      });
    } else {
      console.log("Something invalid in the form data.");
    }
  });

  btnNewAccountCancel = document.querySelector("#new-account-btn-cancel");
  btnNewAccountCancel.addEventListener("click", (e) => {
    console.log("cancelled");
    // hide the form
    $("#new-account-form").fadeToggle(250, (d) => {
      // clear the form
      resetAccountForm();
      //re-enable the + account button
      btnNewAccount.removeAttribute("disabled");
    });
  });
});

async function save_new_account(account) {
  let success = await invoke("save_new_account", {
    serialized: JSON.stringify(account),
  });
  console.log(success ? "Account Saved" : "Account Problem!");
  return success;
}

function validateNewAccountForm() {
  let formValid = true;
  let newAccount = {};
  // check label is a string
  let label = $("#new-account-form-label").val().trim();
  if (!isAlphanumeric(label)) {
    formValid = false;
    console.log("Invalid Label");
    $("#new-account-form-label").addClass("invalid");
  } else {
    newAccount["name"] = label;
    $("#new-account-form-label").removeClass("invalid");
  }

  // check that balance is a number
  let balance = $("#new-account-form-balance").val().trim();
  let result = Intl.NumberFormat("us-EN").format(balance);
  if (result == "NaN") {
    formValid = false;
    console.log("Invalid Balance");
    $("#new-account-form-balance").addClass("invalid");
  } else {
    newAccount["balance"] = parseFloat(balance);
    $("#new-account-form-balance").removeClass("invalid");
  }

  // check that type is not 0
  let type = $("#new-account-form-type").val();
  if (type == 0 || type == null) {
    formValid = false;
    console.log("Invalid Type");
  } else {
    newAccount["kind"] = type;
  }

  return formValid ? newAccount : null;
}

function resetAccountForm() {
  $("#new-account-form-label").val("");
  $("#new-account-form-balance").val("");
  $("#new-account-form-type").val(0);
}

function isAlphanumeric(str) {
  return str.match(/^[a-zA-Z0-9 ]+$/) !== null;
}

function isNumeric(str) {
  return str.match(/^[0-9]+$/) !== null;
}
