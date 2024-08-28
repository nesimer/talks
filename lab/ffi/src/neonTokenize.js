
import { createRequire } from "module";

const require = createRequire(import.meta.dirname);

export default require("./src/neon-jwt/index").parseAsJWT;