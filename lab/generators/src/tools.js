export function piper(...fns) {
  return stream =>
    fns
      .filter(Boolean)
      .reduce(
        (accStream, fn) => fn(accStream),
        stream
      );
} 