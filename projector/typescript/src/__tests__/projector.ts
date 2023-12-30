import Projector from "../projector";
import { Operation } from "../config";

function createData() {
    return {
        projector: {
            "/": {
                "foo": "bar1",
                "test": "test1",
            },
            "/foo": {
                "foo": "bar2",
            },
            "/foo/bar": {
                "foo": "bar3",
            },
        }
    };
}

function getProjector(pwd: string, data = createData()): Projector {
    return new Projector({
        args: [],
        operation: Operation.Print,
        pwd,
        config: "Test Data",
    }, data);
}

test("getValueAll", function() {
    const projector = getProjector("/foo/bar");
    expect(projector.getValueAll()).toEqual({
        "test": "test1",
        "foo": "bar3",
    });
});

test("getValue", function() {
    let projector = getProjector("/foo/bar");
    expect(projector.getValue("foo")).toEqual("bar3");
    projector = getProjector("/foo");
    expect(projector.getValue("foo")).toEqual("bar2");
});

test("setValue", function() {
    let data = createData();
    let projector = getProjector("/foo/bar", data);
    projector.setValue("foo", "baz");
    expect(projector.getValue("foo")).toEqual("baz");
    projector.setValue("test", "test2");
    expect(projector.getValue("test")).toEqual("test2");

    projector = getProjector("/", data);
    expect(projector.getValue("test")).toEqual("test1");
});

test("removeValue", function() {
    const projector = getProjector("/foo/bar");
    projector.removeValue("test");
    expect(projector.getValue("test")).toEqual("test1");

    projector.removeValue("foo");
    expect(projector.getValue("foo")).toEqual("bar2");
});
