// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/MyContract.sol";

contract ContractBTest is Test {
    ContractB contractB;

    function setUp() public {
        contractB = new ContractB();
    }

    function test_NumberIs42() public {
        assertEq(contractB.testNumber(), 42);
    }

    function testFail_Subtract43() public {
        contractB.subtract(43);
    }
}
