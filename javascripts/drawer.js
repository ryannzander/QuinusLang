/* Ensure drawer (sidebar) is open on desktop viewport */
(function() {
  function openDrawerOnDesktop() {
    if (window.matchMedia("(min-width: 76.25em)").matches) {
      var drawer = document.getElementById("__drawer");
      if (drawer) drawer.checked = true;
    }
  }
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", openDrawerOnDesktop);
  } else {
    openDrawerOnDesktop();
  }
  window.addEventListener("resize", openDrawerOnDesktop);
})();
