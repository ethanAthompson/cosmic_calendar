// Does quick operations with the theme

try {
  var theme = localStorage.getItem('theme');
  document.getElementById('html-theme').className = theme;
} catch (e) {
  console.log('Operation: Failed', e);
}



