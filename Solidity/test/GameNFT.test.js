const GameNFT = artifacts.require("GameNFT");

contract("GameNFT", (accounts) => {
  let gameNFT;

  before(async () => {
    gameNFT = await GameNFT.deployed();
  });

  it("should mint a new NFT", async () => {
    const result = await gameNFT.mintNFT(accounts[1], "TestNFT", "TNFT", "https://example.com/nft");
    assert.equal(result.logs[0].event, "Transfer", "Transfer event should be emitted");
  });

  
});