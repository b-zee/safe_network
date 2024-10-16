import { assert, expect, test } from 'vitest'
import * as aut from '../pkg';

test('adds 1 + 2 to equal 3', async () => {
  aut.logInit("sn_networking=warn,autonomi=trace");

  const client = await aut.Client.connect(["/ip4/127.0.0.1/tcp/36075/ws/p2p/12D3KooWJ4Yp8CjrbuUyeLDsAgMfCb3GAYMoBvJCRp1axjHr9cf8"]);

  expect(1 + 2).toBe(3);
});
