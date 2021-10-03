function defineSpecsFor(apiRoot){

  function get(url, options){
    return getRaw(url,options).then( transformResponseToJson );
  }

  function getRaw(url, options){
    return ajax("GET", url, options);
  }
  function post(url, data, options){
    options = options || {};
    options.data = JSON.stringify(data);
    return ajax("POST", url, options);
  }
  function postJson(url, data, options){
    return post(url,data,options).then( transformResponseToJson );
  }

  function patch(url, data, options){
    options = options || {};
    options.data = JSON.stringify(data);
    return ajax("PATCH", url, options);
  }
  function patchJson(url, data, options){
    return patch(url,data,options).then( transformResponseToJson );
  }

  function delete_(url, options){
    return ajax("DELETE", url, options);
  }

  function postRoot(data){
    return postJson(apiRoot,data);
  }
  function getRoot(){
    return get(apiRoot);
  }

  function urlFromTodo(todo){ return todo.url; }

  describe( "Todo-Backend API residing at "+apiRoot, function(){

    function createFreshTodoAndGetItsUrl(params){
      var postParams = _.defaults( (params||{}), {
        title: "blah"
      });
      return postRoot(postParams)
        .then( urlFromTodo );
    };

    describe( "the pre-requisites", function(){
      specify( "the api root responds to a GET (i.e. the server is up and accessible, CORS headers are set up)", function(){
        var getRoot = getRaw(apiRoot);
        return expect( getRoot ).to.be.fulfilled;
      });

      specify( "the api root responds to a POST with the todo which was posted to it", function(){
        var postRoot = postJson(apiRoot, {title:"a todo"});
        return expect( postRoot ).to.eventually.have.property("title","a todo");
      });

      specify( "the api root responds successfully to a DELETE", function(){
        var deleteRoot = delete_(apiRoot);
        return expect( deleteRoot ).to.be.fulfilled;
      });

      specify( "after a DELETE the api root responds to a GET with a JSON representation of an empty array", function(){
        var deleteThenGet = delete_(apiRoot).then( getRoot );

        return expect( deleteThenGet ).to.become([]);
      });
    });

    describe( "storing new todos by posting to the root url", function(){
      beforeEach(function(){
        return delete_(apiRoot);
      });

      it("adds a new todo to the list of todos at the root url", function(){
        var getAfterPost = postRoot({title:"walk the dog"}).then(getRoot);
        return getAfterPost.then(function(todosFromGet){
          expect(todosFromGet).to.have.length(1);
          expect(todosFromGet[0]).to.have.property("title","walk the dog");
        });
      });

      function createTodoAndVerifyItLooksValidWith( verifyTodoExpectation ){
        return postRoot({title:"blah"})
          .then(verifyTodoExpectation)
          .then(getRoot)
          .then(function(todosFromGet){
            verifyTodoExpectation(todosFromGet[0]);
        });
      }

      it("sets up a new todo as initially not completed", function(){
        return createTodoAndVerifyItLooksValidWith(function(todo){
          expect(todo).to.have.property("completed",false);
          return todo;
        });
      });

      it("each new todo has a url", function(){
        return createTodoAndVerifyItLooksValidWith(function(todo){
          expect(todo).to.have.a.property("url").is.a("string");
          return todo;
        });
      });
      it("each new todo has a url, which returns a todo", function(){
        var fetchedTodo = postRoot({title:"my todo"})
          .then( function(newTodo){
            return get(newTodo.url);
          });
        return expect(fetchedTodo).to.eventually.have.property("title","my todo");
      });
    });


    describe( "working with an existing todo", function(){
      beforeEach(function(){
        return delete_(apiRoot);
      });

      it("can navigate from a list of todos to an individual todo via urls", function(){
        var makeTwoTodos = Q.all( [
          postRoot({title:"todo the first"}),
          postRoot({title:"todo the second"})
          ] );

        var getAgainstUrlOfFirstTodo = makeTwoTodos.then( getRoot ).then( function(todoList){
          expect(todoList).to.have.length(2);
          return get(urlFromTodo(todoList[0]));
        });

        return expect(getAgainstUrlOfFirstTodo).to.eventually.have.property("title");
      });

      it("can change the todo's title by PATCHing to the todo's url", function(){
        return createFreshTodoAndGetItsUrl({title:"initial title"})
          .then( function(urlForNewTodo){
            return patchJson( urlForNewTodo, {title:"bathe the cat"} );
          }).then( function(patchedTodo){
            expect(patchedTodo).to.have.property("title","bathe the cat");
          });
      });

      it("can change the todo's completedness by PATCHing to the todo's url", function(){
        return createFreshTodoAndGetItsUrl()
          .then( function(urlForNewTodo){
            return patchJson( urlForNewTodo, {completed:true} );
          }).then( function(patchedTodo){
            expect(patchedTodo).to.have.property("completed",true);
          });
      });

      it("changes to a todo are persisted and show up when re-fetching the todo", function(){
        var patchedTodo = createFreshTodoAndGetItsUrl()
          .then( function(urlForNewTodo){
            return patchJson( urlForNewTodo, {title:"changed title", completed:true} );
          });

        function verifyTodosProperties(todo){
          expect(todo).to.have.property("completed",true);
          expect(todo).to.have.property("title","changed title");
        }

        var verifyRefetchedTodo = patchedTodo.then(function(todo){
          return get( todo.url );
        }).then( function(refetchedTodo){
          verifyTodosProperties(refetchedTodo);
        });

        var verifyRefetchedTodoList = patchedTodo.then(function(){
          return getRoot();
        }).then( function(todoList){
          expect(todoList).to.have.length(1);
          verifyTodosProperties(todoList[0]);
        });

        return Q.all([
          verifyRefetchedTodo,
          verifyRefetchedTodoList
        ]);
      });

      it("can delete a todo making a DELETE request to the todo's url", function(){
        var todosAfterCreatingAndDeletingTodo = createFreshTodoAndGetItsUrl()
          .then( function(urlForNewTodo){
            return delete_(urlForNewTodo);
          }).then( getRoot );
        return expect(todosAfterCreatingAndDeletingTodo).to.eventually.be.empty;
      });

    });

    describe("tracking todo order", function(){
      it("can create a todo with an order field", function(){
        var postResult = postRoot({title:"blah",order:523});
        return expect(postResult).to.eventually.have.property("order",523);
      });

      it("can PATCH a todo to change its order", function(){
        var patchedTodo = createFreshTodoAndGetItsUrl( {order: 10} )
          .then( function(newTodoUrl){
            return patchJson(newTodoUrl,{order:95});
          });

        return expect(patchedTodo).to.eventually.have.property("order",95);
      });

      it("remembers changes to a todo's order", function(){
        var refetchedTodo = createFreshTodoAndGetItsUrl( {order: 10} )
          .then( function(newTodoUrl){
            return patchJson(newTodoUrl,{order:95});
          }).then( function( patchedTodo ){
            return get(urlFromTodo(patchedTodo));
          });

        return expect(refetchedTodo).to.eventually.have.property("order",95);
      });
    });
  });


  function transformResponseToJson(data){
    try{
      return JSON.parse(data);
    } catch(e) {
      var wrapped = new Error("Could not parse response as JSON.");
      wrapped.stack = e.stack;
      throw wrapped;
    }
  }

  function interpretXhrFail(httpMethod,url,xhr){
    var failureHeader = "\n\n"+httpMethod+" "+url+"\nFAILED\n\n";
    if( xhr.status == 0 ){
      return Error(
        failureHeader
        + "The browser failed entirely when make an AJAX request.\n"
        + "Either there is a network issue in reaching the url, or the\n"
        + "server isn't doing the CORS things it needs to do.\n"
        + "Ensure that you're sending back: \n"
        + "  - an `access-control-allow-origin: *` header for all requests\n"
        + "  - an `access-control-allow-headers` header which lists headers such as \"Content-Type\"\n"
        + "\n"
        + "Also ensure you are able to respond to OPTION requests appropriately. \n"
        + "\n"
      );
    }else{
      return Error(
        failureHeader
        + xhr.status + ": " + xhr.statusText + " (" + xhr.responseText.replace(/\n*$/, "") + ")"
        + "\n\n"
      );
    }
  }

  function ajax(httpMethod, url, options){
    var ajaxOptions = _.defaults( (options||{}), {
      type: httpMethod,
      url: url,
      contentType: "application/json",
      dataType: "text", // so we can explicitly parse JSON and fail with more detail than jQuery usually would give us
      timeout: 30000 // so that we don't fail while waiting on a heroku dyno to spin up
    });

    var xhr = $.ajax( ajaxOptions );

    return Q.promise( function(resolve, reject){
      xhr.success( function(){
        return resolve(xhr);
      });
      xhr.fail( function(){
        reject(interpretXhrFail(httpMethod,url,xhr));
      });
    });
  };
}
