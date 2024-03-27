// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/MyContract.sol";
import "../src/MyContractVer2.sol";


contract ContractBCombinedTest is Test {
    ContractB contractB;
    ContractBVersion2 contractBVersion2;

    function setUp() public {
        contractB = new ContractB();
        contractBVersion2 = new ContractBVersion2();
    }

    // Invariant test to ensure testNumber never becomes negative for both versions
    function invariant_testNumberNonNegative() public {
        assertTrue(contractB.testNumber() >= 0, "Invariant failed: testNumber is negative in ContractB");
        assertTrue(contractBVersion2.testNumber() >= 0, "Invariant failed: testNumber is negative in ContractBVersion2");
    }

    // Differential test to compare behaviors of two contract versions with fuzzing
    function testDifferential_Subtract(uint256 num) public {
        uint256 initialNumberB = contractB.testNumber();
        uint256 initialNumberB2 = contractBVersion2.testNumber();

        if(initialNumberB >= num) {
            contractB.subtract(num);
        }
        if(initialNumberB2 >= num) {
            contractBVersion2.subtract(num);
        }

        uint256 newNumberB = contractB.testNumber();
        uint256 newNumberB2 = contractBVersion2.testNumber();

        // Invariant checks after operations
        invariant_testNumberNonNegative();

        // Differential check to ensure both contracts have processed the subtraction similarly
        if(initialNumberB >= num && initialNumberB2 >= num) {
            assertEq(newNumberB, newNumberB2, "Differential failed: Contract states differ after subtraction");
        }
    }
}
