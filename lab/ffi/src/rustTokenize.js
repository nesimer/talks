import { DataType, load, open } from "node-ffi-rs";
import { join } from "path";

open({
  library: "rustjwt",
  path: join(import.meta.dirname, "./rust-jwt/target/release/librustjwt.so")
});

export default (...args) => load({
  library: "rustjwt",
  funcName: "tokenize",
  retType: DataType.Void,
  paramsType: [DataType.String, DataType.String],
  paramsValue: args
});