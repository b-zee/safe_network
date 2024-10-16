import init, * as aut from '../pkg/autonomi.js';

describe('sum', function () {
    this.timeout(30000);
    it('should return sum of argusments', async () => {
        await init();

        aut.logInit("sn_networking=warn,autonomi=trace");
        console.log("wauw");
        const client = await aut.Client.connect(["/ip4/127.0.0.1/tcp/36075/ws/p2p/12D3KooWJ4Yp8CjrbuUyeLDsAgMfCb3GAYMoBvJCRp1axjHr9cf8"]);
        console.log(client);

        chai.expect(1 + 2).to.equal(3);
    });
});
