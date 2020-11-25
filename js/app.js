$(document).ready(function () {
  $('#language-switcher').change(function () {
    window.location = this.value;
  });

  $('#release-switcher').change(function () {
    window.location = this.value;
  });

  $('.subsection').click(function () {
    $(this).next().slideToggle();
  });
});
