let pluginId = Deno.openPlugin("./target/debug/libdeno_actix.dylib");

export function run(): number {
  //@ts-ignore
  const { run } = Deno.core.ops();
  //@ts-ignore
  const response = Deno.core.dispatch(run)!;
  return response[0];
}

export function close(): void {
  Deno.close(pluginId);
}
