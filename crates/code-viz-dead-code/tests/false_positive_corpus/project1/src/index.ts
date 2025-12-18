// Entry point that dynamically loads plugins
export async function loadPlugin(name: string) {
  const plugin = await import(`./plugins/${name}_plugin.ts`);
  return plugin.default;
}

// Main function
export async function main() {
  const userPlugin = await loadPlugin('user');
  const adminPlugin = await loadPlugin('admin');

  await userPlugin.init();
  await adminPlugin.init();
}

main();
