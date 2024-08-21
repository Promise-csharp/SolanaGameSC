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

  it("should retrieve NFT metadata", async () => {
    const tokenId = 1;
    const metadata = await gameNFT.getTokenMetadata(tokenId);
    assert.equal(metadata.name, "TestNFT", "Name should match");
    assert.equal(metadata.symbol, "TNFT", "Symbol should match");
    assert.equal(metadata.uri, "https://example.com/nft", "URI should match");
  });
});