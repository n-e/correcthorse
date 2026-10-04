import { Chess } from "chess.js";
import { exec } from "node:child_process";
import { promisify } from "node:util";
const execp = promisify(exec);

const positions = [
  // start
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
  // kiwipete
  "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
  // Position 3 from https://chessprogramming.org/Perft_Results
  "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -",
  // Position 4 from https://chessprogramming.org/Perft_Results (promo)
  "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -",
  // Position 5 from the same source
  "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ -",
];

function chessjs_perft(position: Chess, depth: number): string[] {
  if (depth === 0) {
    return [""];
  }
  const moves = position.moves({ verbose: true });

  const ms = [];
  for (const m of moves) {
    position.move(m);

    const sub = chessjs_perft(position, depth - 1);
    ms.push(...sub.map((s) => `${m.lan} ${s}`));

    position.undo();
  }

  return ms;
}

for (const p of positions) {
  const depth = 3;
  const { stdout } = await execp(
    `./target/debug/correcthorse -v --perft ${depth} --epd "${p}"`,
    { maxBuffer: 10 * 1024 * 1024 },
  );
  const actual = new Set(stdout.split("\n").filter((x) => x));
  const expected = new Set(
    chessjs_perft(new Chess(p), depth).map((x) => x.trim()),
  );

  console.log("illegal moves:", actual.difference(expected));
  console.log("Missing moves:", expected.difference(actual));
}
