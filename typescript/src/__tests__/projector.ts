import { Operation } from "../config";
import { Projector } from "../projector";

function createData() {
  return {
    projector: {
      "/": {
        foo: "bar1",
        fem: "is_great",
      },
      "/foo": {
        foo: "bar2",
      },
      "/foo/bar": {
        foo: "bar3",
      },
    },
  };
}

function getProjector(pwd: string, data = createData()): Projector {
  return new Projector(
    {
      args: [],
      operation: Operation.Print,
      pwd,
      config: "Hello",
    },
    data
  );
}

test("getValueAll", () => {
  const proj = getProjector("/foo/bar");
  expect(proj.getValueAll()).toEqual({
    fem: "is_great",
    foo: "bar3",
  });
});

test("getValue", () => {
  let proj = getProjector("/foo/bar");
  expect(proj.getValue("foo")).toEqual("bar3");

  proj = getProjector("/foo");
  expect(proj.getValue("foo")).toEqual("bar2");
  expect(proj.getValue("fem")).toEqual("is_great");
});

test("setValue", () => {
  let data = createData();
  let proj = getProjector("/foo/bar", data);
  proj.setValue("foo", "baz");

  expect(proj.getValue("foo")).toEqual("baz");
  proj.setValue("fem", "is_better_than_great");
  expect(proj.getValue("fem")).toEqual("is_better_than_great");

  proj = getProjector("/", data);
  expect(proj.getValue("fem")).toEqual("is_great");
});

test("removeValue", () => {
  const proj = getProjector("/foo/bar");
  proj.removeValue("fem");
  expect(proj.getValue("fem")).toEqual("is_great");

  proj.removeValue("foo");
  expect(proj.getValue("foo")).toEqual("bar2");
});
