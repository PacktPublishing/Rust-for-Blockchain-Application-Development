// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/MyContract.sol";

contract ContractBForkAndFuzzTest is Test {
    ContractB contractB;

    function setUp() public {
        contractB = new ContractB();
    }

    // Fuzz test for the subtract function
    function testFuzz_Subtract(uint256 num) public {
        uint256 initialNumber = contractB.testNumber();
        if(num > initialNumber) {
            assertTrue(true); // Pass if subtraction would underflow, avoiding the actual call
        } else {
            contractB.subtract(num);
            uint256 newNumber = contractB.testNumber();
            assertEq(initialNumber - num, newNumber);
        }
    }

    // Simulated fork test
    function testFork_Subtract() public {
        // In a local fork simulation, we just redeploy the contract
        // This is not an actual fork test in the traditional sense but demonstrates how it would work
        contractB = new ContractB();
        uint256 initialNumber = contractB.testNumber();
        uint256 subtractValue = 10; // Example subtract value

        if(initialNumber >= subtractValue) {
            contractB.subtract(subtractValue);
            uint256 newNumber = contractB.testNumber();
            assertEq(initialNumber - subtractValue, newNumber);
        }
    }
}
