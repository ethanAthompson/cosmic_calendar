
// NOTE: themes are dark or light
try {
  var theme = localStorage.getItem('theme');
  document.getElementById('html-theme').className = theme;

  // WARNING: without this, the app will break 100% for the user
  if (theme == null) {
    check_html_theme();
  }

} catch (e) {
  console.log('Operation: Failed', e);
}


function check_html_theme() {
  // WARNING if no theme is present; but causes bug, maybe if localstorage is not there then do this
  if (document.getElementById("html-theme").className == "null" || "") {

    // Check if the user prefers a dark color scheme
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {

      // User prefers dark color scheme
      console.log('Dark mode is preferred by the user.');
      localStorage.setItem("theme", "dark");
      document.getElementById('html-theme').className = theme;

    } else {
      // User prefers light color scheme
      console.log('Light mode is preferred by the user.');
      localStorage.setItem("theme", "light");
      document.getElementById('html-theme').className = theme;
    }
  }

}