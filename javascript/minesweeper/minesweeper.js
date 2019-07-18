export function annotate(board) {
  const modified = board.map(row => row.split(''));
  for (let y = 0; y < modified.length; y += 1) {
    for (let x = 0; x < modified[y].length; x += 1) {
      if (modified[y][x] === '*') {
        const maxY = Math.min(y + 1, modified.length - 1);
        for (let adjacentY = Math.max(0, y - 1); adjacentY <= maxY; adjacentY += 1) {
          const maxX = Math.min(x + 1, modified[0].length - 1);
          for (let adjacentX = Math.max(0, x - 1); adjacentX <= maxX; adjacentX += 1) {
            let c = modified[adjacentY][adjacentX];
            if (c !== '*') {
              if (c === ' ') {
                c = '0';
              }
              const n = parseInt(c, 10) + 1;
              modified[adjacentY][adjacentX] = n.toString();
            }
          }
        }
      }
    }
  }
  return modified.map(row => row.join(''));
}
