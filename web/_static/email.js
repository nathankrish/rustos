function fill_email() {
  var emails = {
    "staff": "!@#3!@#210!@#-s!@#t!@#aff@c!@#c.ga!@#t!@#ech.e!@#du",
    "yechan": "!@#ye!@#ch!@#an@ga!@#t!@#ech.!@#edu",
    "sujin": "su!@#j!@#in.p!@#ark@ga!@#t!@#ec!@#h.edu",
    "mansour": "ma!@#ns!@#ou!@#ra!@#h@ga!@#t!@#ec!@#h.edu",
  };

  for (var recv in emails) {
    var email = (emails[recv]).replace(/!@#/g, "");
    var alls = document.getElementsByClassName("reference external");
    var forms = Array.prototype.filter.call(alls, function (e) {
      return e.href === 'mailto:' + recv;
    });
    for (var i = 0; i < forms.length; i++) {
      forms[i].href = "mailto:" + email;
      forms[i].textContent = email;
    }
  }
}

fill_email();
