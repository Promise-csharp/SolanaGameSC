// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract GameNFT is ERC721, Ownable {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    // Struct to hold NFT metadata
    struct NFTMetadata {
        string name;
        string symbol;
        string uri;
    }

    // Mapping from token ID to metadata
    mapping(uint256 => NFTMetadata) private _tokenMetadata;

    constructor() ERC721("GameNFT", "GNFT") {}

    // Function to mint a new NFT
    function mintNFT(address recipient, string memory name, string memory symbol, string memory uri) public onlyOwner returns (uint256) {
        _tokenIds.increment();
        uint256 newTokenId = _tokenIds.current();
        _mint(recipient, newTokenId);
        _tokenMetadata[newTokenId] = NFTMetadata(name, symbol, uri);
        return newTokenId;
    }

    
}