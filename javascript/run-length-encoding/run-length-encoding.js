export function encode(full) {
  let encoded = '';
  let current = '';
  let total = 0;
  for (let i = 0; i < full.length; i += 1) {
    if (current !== full.charAt(i)) {
      if (total > 1) {
        encoded += total;
      }
      encoded += current;
      total = 1;
      current = full.charAt(i);
    } else {
      total += 1;
    }
  }
  if (total > 1) {
    encoded += total;
  }
  encoded += current;
  return encoded;
}

export function decode(encoded) {
  let decoded = '';
  let countString = '';
  for (let i = 0; i < encoded.length; i += 1) {
    const c = encoded.charAt(i);
    if (c >= '0' && c <= '9') {
      countString += c;
    } else {
      let count = parseInt(countString, 10);
      countString = '';
      if (!count) {
        count = 1;
      }
      for (let j = 0; j < count; j += 1) {
        decoded += c;
      }
    }
  }
  return decoded;
}
