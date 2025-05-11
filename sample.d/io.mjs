/**
 * @template T
 * @typedef {function(): Promise<T>} IO<T>
 */

/**
 * @template T
 * @param {T} t
 * @returns {IO<T>}
 */
export function of(t) {
  return () => {
    return Promise.resolve(t);
  };
}

/**
 * @template T
 * @template U
 * @param {IO<T>} io
 * @param {function(T): IO<U>} mapper
 * @returns {IO<U>}
 */
export function bind(io, mapper) {
  return () => {
    return Promise.resolve()
      .then((_) => io())
      .then((t) => mapper(t)());
  };
}
