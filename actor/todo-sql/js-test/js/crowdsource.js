(function(){

var SERVICES_BASE = "https://services.todobackend.com/";
var TEST_RUN_CREATE_URL = new URL('/test-runs', SERVICES_BASE);

mocha.setup('bdd');
mocha.slow("5s");
mocha.timeout("30s"); //so that tests don't fail with a false positive while waiting for e.g a heroku dyno to spin up
window.expect = chai.expect;


var targetRootUrl = window.location.search.substr(1);
  
if( targetRootUrl ){
  defineSpecsFor(targetRootUrl);
  runAndRecordTests();
}else{
  console.warn('no target specified for tests');
}

function runAndRecordTests(){
  mocha.checkLeaks();
  var runner = mocha.run();

  var testResultsUrl = null;
  startRecordingTestRun().then( function(testRunResource){
    testResultsUrl = new URL(testRunResource._links.results.href, SERVICES_BASE);
  });

  track('Test Start',{targetRootUrl:targetRootUrl});

  runner.on('suite end', function(suite){
    if( suite.root ){
      var suitePayload = serializeSuite(suite);

      track('Test Suite End', suitePayload);
      if( testResultsUrl ){
        recordTestResults(testResultsUrl,suitePayload);
      }else{
        console.log('no test run url available to record results');
      }
    }
  });
};

function startRecordingTestRun(){
  return $.post(TEST_RUN_CREATE_URL);
}

function recordTestResults(testResultsUrl,results){
  return $.ajax({
    url: testResultsUrl,
    method: 'POST',
    contentType: 'application/json',
    data: JSON.stringify(results)
  });
}

function serializeSuite(suite){
  var childSuites = _.map( suite.suites, serializeSuite );
  var tests = _.map( suite.tests, function(test){
    return _.pick(test,'duration','title','state','pending','speed','sync','timedOut','type');
  });

  return {
    suites: childSuites,
    tests: tests,
    root: suite.root,
    pending: suite.pending,
    title: suite.title
  };
}

function track(eventName,eventPayload){
  analytics.track(eventName, eventPayload, { context: { ip: "0.0.0.0" }});
  
}

})();
