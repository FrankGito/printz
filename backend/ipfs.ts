const command = new Deno.Command("cat", {
  args: ["./README.md"],
});
command.spawn();
