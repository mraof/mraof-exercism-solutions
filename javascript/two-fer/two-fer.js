export function twoFer(name) {
  let newName = name;
  if (!newName) {
    newName = 'you';
  }
  return `One for ${newName}, one for me.`;
}
