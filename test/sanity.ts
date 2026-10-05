import { exec } from "node:child_process";
import { readFile } from "node:fs/promises";
import { promisify } from "node:util";
const execp = promisify(exec);

const pnum = (a: string) => (a !== undefined && a.length > 0 ? +a : undefined);
const pstr = (a: string) => (a !== undefined && a.length > 0 ? a : undefined);
const positions: {
  t: string;
  d: number;
  epd: string;
  expectedScore?: number;
  expectedMove?: string[];
}[] = (await readFile(import.meta.dirname + "/" + "sanity.csv", "utf-8"))
  .split("\n")
  .filter((x) => x && !x.trim().startsWith("#"))
  .map((l) => {
    const [epd, t, d, expectedMove, expectedScore] = l.split(",");
    return {
      epd,
      t,
      d: pnum(d)!,
      expectedMove: pstr(expectedMove)?.split("|"),
      expectedScore: pnum(expectedScore),
    };
  });

for (const p of positions) {
  const url = `https://lichess.org/analysis/standard/${p.epd.replaceAll(/ /g, "_")}`;

  const cmd = `./target/release/correcthorse -v --search ${p.d} --epd "${p.epd}"`;
  const { stdout } = await execp(cmd, { maxBuffer: 10 * 1024 * 1024 });

  const sp = stdout
    .split("\n")
    .filter((x) => x)
    .at(-1)!
    .split(" ");

  const ev = sp.at(-1)!;
  const pv = sp.slice(1, -2).join(" ");

  let errors: string[] = [];

  if (p.expectedMove === undefined && p.expectedScore === undefined) {
    errors.push("NO ASSERTIONS");
  }
  if (p.expectedMove && !p.expectedMove.some((x) => pv.startsWith(x)))
    errors.push(`move: ${pv}, expected ${p.expectedMove}`);
  if (
    p.expectedScore &&
    // !!!! FUZZY MATCHING pour les mats
    (Math.abs(p.expectedScore) > 950
      ? Math.abs(p.expectedScore - +ev) > 10
      : p.expectedScore !== +ev)
  ) {
    console.log(p.expectedScore, +ev);
    errors.push(`score: ${ev}, expected ${p.expectedScore}`);
  }
  if (errors.length > 0) {
    console.log(p.t, errors.join(", "), url);
    console.log(cmd);
    console.log(stdout);
  }
}
