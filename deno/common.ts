import { WasiReactorOut } from "../bindings/bindings.ts";

interface PythonOutput {
  value: string, // json string
  error: boolean
}

export function processOutput(out: WasiReactorOut): string {
  if ("Ok" in out) {
    // vm output is ok
    const py: PythonOutput = JSON.parse(out.Ok.res);
    if (py.error) {
      // python error
      throw Error(py.value);
    }
    return py.value;
  }
  // vm error
  throw Error(out.Err.message);
}