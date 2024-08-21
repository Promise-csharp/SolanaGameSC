const GameToken = artifacts.require("GameToken");

contract("GameToken", (accounts) => {
  let gameToken;

  before(async () => {
    gameToken = await GameToken.deployed();
  });

  it("should have correct initial supply", async () => {
    const totalSupply = await gameToken.totalSupply();
    assert.equal(totalSupply.toString(), "1000000000000000000000000", "Initial supply should be 1,000,000 tokens");
  });

  it("should execute game transaction", async () => {
    const amount = web3.utils.toWei("100", "ether");
    await gameToken.transfer(accounts[1], amount, { from: accounts[0] });
    const result = await gameToken.executeGameTransaction(accounts[1], accounts[2], amount);
    assert.equal(result.logs[0].event, "GameTransactionExecuted", "GameTransactionExecuted event should be emitted");
  });
});