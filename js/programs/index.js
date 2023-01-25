const programs = [
  { name: "Hello, World!", file: "hello-world.bf" },
  { name: "Infinite Loop", code: "+[]", input: "abc" },
];

export const load = async () => {
  for (const program of programs) {
    if ("file" in program) {
      const req = await fetch(new URL(program.file, import.meta.url));
      const body = await req.text();

      program.code = body;

      delete program.file;
    }

    if (!("input" in program)) {
      program.input = "";
    }
  }

  return programs;
};
