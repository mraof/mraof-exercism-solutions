import ballerina/http;
import ballerina/log;

// By default, Ballerina exposes a service via HTTP/1.1.
service hello on new http:Listener(9090) {

    // Invoke `sayHello` resource with caller and request.
    resource function sayHello(http:Caller caller, http:Request req) {
        var result = caller->respond("Hello, World!");

        if (result is error) {
            log:printError("Error responding", err = result);
        }
    }
}
