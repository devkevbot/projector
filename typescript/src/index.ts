import { parseOpts } from "./opts";
import { createConfig } from "./config";

const config = createConfig(parseOpts());
console.log(config);
