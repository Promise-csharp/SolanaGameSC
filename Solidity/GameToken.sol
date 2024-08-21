// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract GameToken is ERC20, Ownable {
    constructor() ERC20("GameToken", "GTK") {
        _mint(msg.sender, 1000000 * 10**decimals());
    }

    

    event GameTransactionExecuted(address indexed from, address indexed to, uint256 amount);
}