const GameNFT = artifacts.require("GameNFT");
const GameToken = artifacts.require("GameToken");

module.exports = function(deployer) {
  deployer.deploy(GameNFT);
  deployer.deploy(GameToken);
};