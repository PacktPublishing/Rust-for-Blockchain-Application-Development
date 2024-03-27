// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../src/MyContract.sol";

contract ContractBTest is Test {
    ContractB contractB;

    function setUp() public {
        contractB = new ContractB();
    }

    function test_ContractBDeployment() public {
        // Deploy the contract inside the test function
        ContractB localContractB = new ContractB();

        // Assert the contract was deployed successfully by checking an initial state
        // For example, if ContractB's constructor sets testNumber to 42, we can check that
        uint256 expectedInitialValue = 42;
        assertEq(localContractB.testNumber(), expectedInitialValue, "ContractB should initialize with the correct value.");
    }
}
