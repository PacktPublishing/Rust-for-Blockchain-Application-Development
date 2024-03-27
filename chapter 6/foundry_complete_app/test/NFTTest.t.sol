// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/NFT.sol"; // Update the path to your NFT contract
import "@openzeppelin/contracts/utils/Strings.sol";

contract NFTTest is Test {
    using Strings for uint256;

    NFT nft;
    address recipient = address(0x1);

    function setUp() public {
        nft = new NFT("TestNFT", "TNFT", "https://example.com/");
    }

    function testMintTo() public {
        uint256 tokenId = nft.mintTo(recipient);
        assertEq(nft.ownerOf(tokenId), recipient, "Owner should be the recipient after minting");
    }

    function testTokenURI() public {
        uint256 tokenId = nft.mintTo(recipient);
        string memory expectedURI = string(abi.encodePacked("https://example.com/", tokenId.toString()));
        assertEq(nft.tokenURI(tokenId), expectedURI, "Token URI should match the expected value");
    }

    function testFailMintToNonExistentToken() public view {
        nft.tokenURI(1); // Should fail if token ID 1 does not exist
    }
}
