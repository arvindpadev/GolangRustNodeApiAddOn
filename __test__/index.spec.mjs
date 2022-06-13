import test from 'ava'

import { sum, getFileSize } from '../index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('sum of file sizes from native', async(t) => {
  const p1 = getFileSize('/home/user1/MyClass.java');
  const p2 = getFileSize('/home/user1/qemu-errors.txt');
  const [ s1, s2 ] = await Promise.all([ p1, p2 ]);
  const actual = sum(s1, s2);
  const expected = s1 + s2;
  t.assert(actual === expected, `Failed: ${actual}, ${expected}, ${s1}, ${s2}`);
})
