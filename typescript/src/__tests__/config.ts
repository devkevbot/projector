import { createConfig, Operation } from "../config";

test("simple print all", function () {
  const config = createConfig({});
  expect(config.operation).toEqual(Operation.Print);
  expect(config.args).toEqual([]);
});

test("print key value", function () {
  const config = createConfig({
    args: ["foo"],
  });
  expect(config.operation).toEqual(Operation.Print);
  expect(config.args).toEqual(["foo"]);
});

test("add key value", function () {
  const config = createConfig({
    args: ["add", "foo", "bar"],
  });
  expect(config.operation).toEqual(Operation.Add);
  expect(config.args).toEqual(["foo", "bar"]);
});

test("remove key value", function () {
  const config = createConfig({
    args: ["rm", "foo"],
  });
  expect(config.operation).toEqual(Operation.Remove);
  expect(config.args).toEqual(["foo"]);
});
