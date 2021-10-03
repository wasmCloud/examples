mocha.setup('bdd');
mocha.slow("5s");
mocha.timeout("30s"); //so that tests don't fail with a false positive while waiting for e.g a heroku dyno to spin up
window.expect = chai.expect;


function loadTargetRootFromInput(){
  var targetRoot = $('#target-chooser input').val();
  window.location.search = targetRoot;
}

$('#target-chooser button').on('click',loadTargetRootFromInput);
$('#target-chooser input').on('keyup',function(){
  if(event.keyCode == 13){
    loadTargetRootFromInput();
  }
});


targetRootUrl = window.location.search.substr(1);

if( targetRootUrl ){
  $("#target-info .target-url").text(targetRootUrl);
  $("#target-chooser").hide();

  defineSpecsFor(targetRootUrl);

  mocha.checkLeaks();
  var runner = mocha.run();
}else{
  $("#target-info").hide();
}
