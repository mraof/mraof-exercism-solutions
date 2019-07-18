import ballerina/http;
import ballerina/io;

@http:ServiceConfig {
    basePath: "/"
}
service EchoService on new http:Listener(9090) {

    // Resource returns the same request payload back to the client.
    @http:ResourceConfig {
        methods: ["POST"]
    }
    resource function echo(http:Caller caller, http:Request req) returns error? {
        var text = check req.getPayloadAsString();
        _ = caller->respond(untaint text);
    }
}
